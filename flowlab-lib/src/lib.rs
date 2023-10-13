// Device module
pub mod device;

// Parser module
pub mod parser;

// Instruction module
pub mod instruction;

#[cfg(test)]
mod tests {
    use super::*;

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
}
