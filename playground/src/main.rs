use regex::Regex;
use std::collections::HashMap;

fn parse_response(format: &str, response: &str, parameters: &[(&str, &str)]) -> HashMap<String, String> {
    let mut pattern = format.to_string();
    // Dynamically construct the regex pattern based on parameter types
    for (name, type_) in parameters {
        let placeholder = format!("{{{{{}}}}}", name);
        let regex_part = match type_ {
            &"float" => "-?\\d*\\.?\\d+", // Match a floating point number
            &"string" => "\\w+", // Match a word (adjust as needed for more complex strings)
            _ => "\\S+", // Default to matching non-space characters
        };
        pattern = pattern.replace(&placeholder, regex_part);
    }
    println!("{}", pattern);
    let re = Regex::new(&pattern).unwrap();
    let mut result = HashMap::new();

    if let Some(caps) = re.captures(response) {
        // Assuming the order of captures matches the order of parameters
        for (i, (name, _)) in parameters.iter().enumerate() {
            if let Some(mat) = caps.get(i + 1) { // captures are 1-indexed
                result.insert(name.to_string(), mat.as_str().to_string());
            }
        }
    }

    result
}

fn main() {
    let format = "{{magnet current}}{{units}}";
    let response = "-40.000kG";
    let parameters = vec![("magnet current", "float"), ("units", "string")];
    let parsed = parse_response(format, response, &parameters);
    println!("{:?}", parsed);
}