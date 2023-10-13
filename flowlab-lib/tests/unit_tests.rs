use std::collections::HashMap;

use flowlab_lib::{device, instruction, parser};

#[tokio::test]
async fn test_read_file() {
    let contents = parser::read_file("../config/devices/devices.yaml")
        .await
        .unwrap();
    assert!(contents.contains("name:"));
}

#[tokio::test]
async fn test_parse_devices() {
    use serde_yaml::Value;
    let devices: Vec<device::Device<Value>> = parser::parse("../config/devices/devices.yaml")
        .await
        .unwrap();
    assert_eq!(devices.len(), 2);
}

#[tokio::test]
async fn test_parse_instructions() {
    let instructions: Vec<instruction::Instruction> =
        parser::parse("../config/instructions/scpi.yaml")
            .await
            .unwrap();
    assert_eq!(instructions[0].command.query, "*IDN?");
}

#[tokio::test]
async fn test_find_instruction_with_name() {
    // Get the instructions from the file
    let instructions: Vec<instruction::Instruction> =
        parser::parse("../config/instructions/scpi.yaml")
            .await
            .unwrap();
    // Get the instruction with the name "Identify the device"
    let filtered_instructions =
        instruction::find_instruction_with_name(&instructions, "Identify the device").unwrap();
    assert!(std::ptr::eq(filtered_instructions, &instructions[0]));
}

#[tokio::test]
async fn test_hashmap_to_context() {
    use tera::Context;
    // Hashmap with the parameters
    let mut parameters: HashMap<String, String> = HashMap::new();
    parameters.insert("channel".to_string(), "1".to_string());
    // Context with the parameters (converted)
    let hashmap2context = instruction::hashmap_to_context(&parameters);
    // Context with the parameters (manually)
    let mut context = Context::new();
    context.insert("channel", "1");
    assert_eq!(context, hashmap2context);
}

#[tokio::test]
async fn test_parse_instructions_with_parameters() {
    // Get the instructions from the file
    let instructions: Vec<instruction::Instruction> =
        parser::parse("../config/instructions/612.yaml")
            .await
            .unwrap();
    // Get the instruction with the name "Get temperature"
    let instruction =
        instruction::find_instruction_with_name(&instructions, "Get temperature").unwrap();
    // Hashmap with the parameters
    let mut parameters: HashMap<String, String> = HashMap::new();
    parameters.insert("channel".to_string(), "1".to_string());
    // Format the command
    let command = instruction::format_command(&instruction.command, &parameters).unwrap();
    assert_eq!(command, "INPUT 1:TEMPERATURE?");
}
