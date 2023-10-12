use flowlab_lib::device::{Device, Protocols, TCP};
use flowlab_lib::parser::parse;
use std::path::Path;

#[tokio::main]
async fn main() {
    let device_path = Path::new("../config/devices/devices.yaml");
    let devices: Vec<Device<Protocols>> = parse(device_path).await.unwrap();
    println!("{:#?}", devices);
}
