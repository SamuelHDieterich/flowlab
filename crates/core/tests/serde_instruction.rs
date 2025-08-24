use flowlab_core::serde::instruction;

#[test]
fn deserialize_instruction() {
    let yaml = r#"
    name: Get temperature
    description: Reports the current temperature reading on any of the input channels.
    command:
      format: "INPUT {{channel}}:TEMP?\n"
      parameters:
        - name: channel
          description: "The channel identification, options include: number (example: 0), characther (example: A), or channel ID (example: CHA)."
          type: string
    response:
      format: "{{temperature}}"
      parameters:
        - name: temperature
          description: "The current temperature reading on the specified channel."
          type: number
    "#;

    let instruction = instruction::Instruction {
        name: "Get temperature".to_string(),
        description: Some("Reports the current temperature reading on any of the input channels.".to_string()),
        command: instruction::Command {
            format: "INPUT {{channel}}:TEMP?\n".to_string(),
            parameters: vec![
                instruction::Parameter {
                    name: "channel".to_string(),
                    description: Some("The channel identification, options include: number (example: 0), characther (example: A), or channel ID (example: CHA).".to_string()),
                    data_type: instruction::ParameterType::String,
                    values: None,
                    default: None
                },
            ].into(),
        },
        response: Some(instruction::Response {
            format: "{{temperature}}".to_string(),
            parameters: vec![
                instruction::Parameter {
                    name: "temperature".to_string(),
                    description: Some("The current temperature reading on the specified channel.".to_string()),
                    data_type: instruction::ParameterType::Number,
                    values: None,
                    default: None
                },
            ].into(),
        }),
    };

    let deserialized: instruction::Instruction = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(deserialized, instruction);
}
