use flowlab_lib::{device, instruction, parser};
use test_log::test;
use tracing::info;

#[test(tokio::test)]
async fn test_read_file() {
    info!("Testing reading file asynchronously");
    let contents = parser::read_file("../config/devices/devices.yaml")
        .await
        .unwrap();
    assert!(contents.contains("name:"));
}

#[test(tokio::test)]
async fn test_parse_devices() {
    use serde_yaml::{Mapping, Number, Value};

    info!("Testing parse file contents to device struct");
    let devices: Vec<device::Device<Value>> = parser::parse("../config/devices/devices.yaml")
        .await
        .unwrap();
    let mut temp_protocol = Mapping::new();
    temp_protocol.insert(
        Value::String("ip".to_string()),
        Value::String("192.168.1.2".to_string()),
    );
    temp_protocol.insert(
        Value::String("port".to_string()),
        Value::Number(Number::from(5000)),
    );
    let device_612a = device::Device {
        name: "612A".to_string(),
        instruction: vec!["scpi".to_string(), "612".to_string()],
        protocol: Value::Mapping(temp_protocol),
        default_arguments: Some(vec![
            device::Arguments {
                name: "channel".to_string(),
                value: "CHA".to_string(),
            },
            device::Arguments {
                name: "units".to_string(),
                value: "K".to_string(),
            },
        ]),
    };
    assert_eq!(devices[1], device_612a);
}

#[test(tokio::test)]
async fn test_parse_instructions() {
    info!("Testing parsing file contents to instructions struct");
    let instructions: Vec<instruction::DeviceCommand> =
        parser::parse("../config/instructions/scpi.yaml")
            .await
            .unwrap();
    let instruction_reset_device = instruction::DeviceCommand {
        name: "Reset the device".to_string(),
        alias: None,
        prelude: None,
        command: instruction::Command {
            query: "*RST".to_string(),
            parameters: None,
        },
        response: None,
        description: Some("Reset the instrument. Depending on the instrument, this may reset the device to a known state or reboot the device.".to_string()),
    };
    assert_eq!(
        instructions[instructions.len() - 1],
        instruction_reset_device
    );
}

#[test(tokio::test)]
async fn test_find_instruction_with_name() {
    info!("Testing finding instruction with specific name");
    // Get the instructions from the file
    let instructions: Vec<instruction::DeviceCommand> =
        parser::parse("../config/instructions/scpi.yaml")
            .await
            .unwrap();
    // Get the instruction with the name "Identify the device"
    let filtered_instructions =
        instruction::find_instruction_with_name(&instructions, "Identify the device").unwrap();
    assert!(std::ptr::eq(filtered_instructions, &instructions[0]));
}

#[test(tokio::test)]
async fn test_hashmap_to_context() {
    use std::collections::HashMap;
    use tera::Context;

    info!("Testing converting hashmap to tera::Context");
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

#[test(tokio::test)]
async fn test_parse_instructions_with_parameters() {
    use std::collections::HashMap;

    info!("Testing formatting an instruction with parameters");
    // Get the instructions from the file
    let instructions: Vec<instruction::DeviceCommand> =
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

#[test(tokio::test)]
async fn test_parse_pipeline() {
    info!("Testing parsing a pipeline");
    // Get the pipeline from the file
    let pipeline: Vec<instruction::PipelineStep> =
        parser::parse("../config/pipelines/example_pipeline.yaml")
            .await
            .unwrap();
    print!("{:?}", pipeline);
    // Check if there is at least one type of each Instruction enum
    let mut device_command = false;
    let mut wait_for = false;
    let mut scan = false;
    for instruction in pipeline {
        match instruction {
            instruction::PipelineStep::DeviceInstruction(_) => device_command = true,
            instruction::PipelineStep::WaitFor(_) => wait_for = true,
            instruction::PipelineStep::Scan(_) => scan = true,
        }
    }
    assert!(device_command);
    assert!(wait_for);
    assert!(scan);
}
