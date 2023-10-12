use flowlab_lib::device::{Device, TCP};

#[tokio::main]
async fn main() {
    let device = Device::<TCP>::new(
        "device1",
        std::net::IpAddr::V4(std::net::Ipv4Addr::new(192, 168, 1, 2)),
        5000,
    );

    print!("{:#?}", device)
}
