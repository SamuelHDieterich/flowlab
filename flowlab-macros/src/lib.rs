extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Lit, Meta, NestedMeta};

#[proc_macro_derive(MapKey)]
pub fn map_key_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`
    let name = &ast.ident;

    // Get the name of the first field
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
        ..
    }) = &ast.data
    {
        named
    } else {
        panic!("expected a struct with named fields");
    };

    let first_field = match fields.first() {
        Some(first_field) => &first_field.ident,
        None => panic!("struct has no fields"),
    };

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl MapKey for #name {
            fn key(&self) -> String {
                self.#first_field.to_string()
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
#[proc_macro_derive(Mapify)]
pub fn mapify_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let (fields, is_tuple_struct): (Vec<_>, bool) = match &ast.data {
        syn::Data::Struct(s) => match &s.fields {
            syn::Fields::Named(_) => (s.fields.iter().collect(), false),
            syn::Fields::Unnamed(_) => (s.fields.iter().collect(), true),
            _ => panic!("Mapify can only be used with structs"),
        },
        _ => panic!("Mapify can only be used with structs"),
    };

    if fields.len() != 1 {
        panic!("struct must have exactly one field");
    }

    let first_field = match fields.first() {
        Some(first_field) => first_field,
        None => panic!("struct has no fields"),
    };

    let first_field_type = &first_field.ty;

    let (key_type, value_type) = match first_field_type {
        syn::Type::Path(type_path) => {
            let path = &type_path.path;
            let last_segment = path.segments.last().unwrap();
            let ident = &last_segment.ident;
            if ident != "HashMap" && ident != "BTreeMap" {
                panic!("field must be of type HashMap or BTreeMap");
            }

            if let syn::PathArguments::AngleBracketed(params) = &last_segment.arguments {
                if params.args.len() != 2 {
                    panic!("HashMap or BTreeMap must have exactly two type parameters");
                }

                let key_type = &params.args[0];
                let value_type = &params.args[1];

                (key_type, value_type)
            } else {
                panic!("HashMap or BTreeMap must have type parameters");
            }
        }
        _ => panic!("field must be of type HashMap or BTreeMap"),
    };

    let field_access = if is_tuple_struct {
        quote!(0)
    } else {
        let first_field_ident = match &first_field.ident {
            Some(ident) => ident,
            None => panic!("field must have an identifier"),
        };
        quote!(#first_field_ident)
    };

    let keys_method = quote! {
        fn keys(&self) -> Vec<&#key_type> {
            self.#field_access.keys().collect()
        }
    };

    let get_method = quote! {
        fn get(&self, key: &#key_type) -> Option<&#value_type> {
            self.#field_access.get(key)
        }
    };

    let get_mut_method = quote! {
        fn get_mut(&mut self, key: &#key_type) -> Option<&mut #value_type> {
            self.#field_access.get_mut(key)
        }
    };

    let insert_method = quote! {
        fn insert(&mut self, key: #key_type, value: #value_type) {
            self.#field_access.insert(key, value);
        }
    };

    let remove_method = quote! {
        fn remove(&mut self, key: &#key_type) -> Option<#value_type> {
            self.#field_access.remove(key)
        }
    };

    let contains_key_method = quote! {
        fn contains_key(&self, key: &#key_type) -> bool {
            self.#field_access.contains_key(key)
        }
    };

    let iter_method = quote! {
        fn iter(&self) -> std::collections::hash_map::Iter<#key_type, #value_type> {
            self.#field_access.iter()
        }
    };

    let iter_mut_method = quote! {
        fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<#key_type, #value_type> {
            self.#field_access.iter_mut()
        }
    };

    let len_method = quote! {
        fn len(&self) -> usize {
            self.#field_access.len()
        }
    };

    let is_empty_method = quote! {
        fn is_empty(&self) -> bool {
            self.#field_access.is_empty()
        }
    };

    let expanded = quote! {
        impl #impl_generics Mapify<#key_type, #value_type> for #name #ty_generics #where_clause {
            #keys_method
            #get_method
            #get_mut_method
            #insert_method
            #remove_method
            #contains_key_method
            #iter_method
            #iter_mut_method
            #len_method
            #is_empty_method
        }
    };
    TokenStream::from(expanded)
}
