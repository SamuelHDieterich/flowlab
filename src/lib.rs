//! # flowlab
//!
//! `flowlab` is a library for the program with same name used for control and monitoring system of
//! laboratory instruments.
//!
//! This is a term project for the course "Engineering Physics" at the Universidade Federal do Rio
//! Grande do Sul (UFRGS). The project was develop to work on a specific equipment, the C-MAG 9
//! Cryostat, but this project is designed to be used with any other equipment.

// Device module
pub mod device;

// Instruction module
pub mod instruction;

// Pipeline module
pub mod pipeline;

// Parser module
// pub mod parser;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DataType {
    Boolean,
    Integer,
    Float,
    String,
}

impl Default for DataType {
    fn default() -> Self {
        DataType::String
    }
}

/// The Data enum is used to define the data types that a parameter or response can take.
/// It is used to define the data type of the parameter or response.
/// The order of the variants is important for the deserialization process.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Data {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}

impl Default for Data {
    fn default() -> Self {
        Data::String(String::default())
    }
}

trait MapKey {
    fn key(&self) -> String;
}

fn from_vec_to_map<'de, D, T>(deserializer: D) -> Result<HashMap<String, T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de> + MapKey,
{
    let vec = Vec::<T>::deserialize(deserializer)?;
    let map: HashMap<String, T> = vec.into_iter().map(|r| (r.key(), r)).collect();
    Ok(map)
}

fn from_map_to_vec<S, T>(map: &HashMap<String, T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
    T: serde::Serialize,
{
    let vec: Vec<&T> = map.values().collect();
    vec.serialize(serializer)
}

pub trait Mapify<K, V> {
    fn keys(&self) -> Vec<&K>;
    fn get(&self, key: &K) -> Option<&V>;
    fn get_mut(&mut self, key: &K) -> Option<&mut V>;
    fn insert(&mut self, key: K, value: V);
    fn remove(&mut self, key: &K) -> Option<V>;
    fn contains_key(&self, key: &K) -> bool;
    fn iter(&self) -> std::collections::hash_map::Iter<K, V>;
    fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<K, V>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}
