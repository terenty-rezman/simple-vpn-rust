use tokio::io::{AsyncReadExt, AsyncWriteExt};
use etherparse::PacketHeaders;

use tunio::traits::{DriverT, InterfaceT};
use tunio::{DefaultDriver, DefaultTokioInterface};

#[tokio::main]
async fn main() {
    let mut driver = DefaultDriver::new().unwrap();
    let if_config: tunio::IfConfig<tunio::platform::wintun::PlatformIfConfig> =
        DefaultDriver::if_config_builder()
            .name("custom-tunnel".to_string())
            .build()
            .unwrap();

    // Then, we create the interface using config and start it immediately.
    let interface = DefaultTokioInterface::new_up(
        &mut driver, if_config
    ).unwrap();
    interface.handle().set_mtu(1400).unwrap();

    // Write to interface using Write trait
    // let buf = [0u8; 4096];
    // let _ = interface.write(&buf).await;

    let (mut read, write) = tokio::io::split(interface);

    // Read from interface using Read trait
    let mut buf = vec![0u8; 4096];
    while let Ok(n) = read.read(buf.as_mut_slice()).await {
        buf.truncate(n);
        println!("{buf:x?}");

        match PacketHeaders::from_ip_slice(&buf) {
            Err(value) => println!("Err {:?}", value),
            Ok(value) => {
                println!("link: {:?}", value.link);
                println!("vlan: {:?}", value.vlan);
                println!("ip: {:?}", value.ip);
                println!("transport: {:?}", value.transport);
            }
        }
        buf.resize(4096, 0u8);
    }

    let mut interface = read.unsplit(write);

    let _ = interface.down();
}
