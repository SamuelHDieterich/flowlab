use flowlab_core::instruction::serde as instructions;

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

    let instruction = instructions::Instruction {
        name: "Get temperature".to_string(),
        description: Some("Reports the current temperature reading on any of the input channels.".to_string()),
        command: instructions::Command {
            format: "INPUT {{channel}}:TEMP?\n".to_string(),
            parameters: vec![
                instructions::Parameter {
                    name: "channel".to_string(),
                    description: Some("The channel identification, options include: number (example: 0), characther (example: A), or channel ID (example: CHA).".to_string()),
                    data_type: instructions::ParameterType::String,
                    values: None,
                    default: None
                },
            ].into(),
        },
        response: Some(instructions::Response {
            format: "{{temperature}}".to_string(),
            parameters: vec![
                instructions::Parameter {
                    name: "temperature".to_string(),
                    description: Some("The current temperature reading on the specified channel.".to_string()),
                    data_type: instructions::ParameterType::Number,
                    values: None,
                    default: None
                },
            ].into(),
        }),
    };

    let deserialized: instructions::Instruction = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(deserialized, instruction);
}
