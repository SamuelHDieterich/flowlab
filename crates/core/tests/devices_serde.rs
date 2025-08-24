use flowlab_core::devices::serde as devices;

#[test]
fn deserialize_device() {
    let yaml = r#"
    name: TM612
    description: Temperature Monitor 612
    protocol:
      ip: 192.168.1.3
      port: 5000
    instructions:
      - ../instructions/SCPI/**/*
      - ../instructions/TM612/**/*
    "#;

    let device: devices::Device = serde_yaml::from_str(yaml).unwrap();

    assert_eq!(device.name, "TM612");
    assert_eq!(device.description, Some("Temperature Monitor 612".into()));
    assert_eq!(device.protocol.get("ip"), Some(&"192.168.1.3".into()));
    assert_eq!(device.protocol.get("port"), Some(&"5000".into()));
    assert_eq!(
        device.instructions,
        vec!["../instructions/SCPI/**/*", "../instructions/TM612/**/*"]
    );
}
