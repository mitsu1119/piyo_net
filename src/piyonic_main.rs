mod nic;
use nic::{Nic, NicType};

use piyo_net::VecEnv;

const DEFAULT_TTY: &str = "/dev/ttyAMA0";

async fn send(nic: &mut Nic) {
    nic.send(vec![0x41; 100])
        .await
        .expect("Failed to send.");
}

#[tokio::main]
async fn main() {
    let mut nic = Nic::new("ttyAMA0".to_string(), NicType::ETHERNET, 8, DEFAULT_TTY.to_string())
        .expect("Failed to create NIC");
    println!("{:?}", nic);

    let ports = match nic.get_env() {
        Ok(ports) => ports,
        Err(_) => {
            nic.create_empty_env();
            nic.get_env().unwrap()
        }
    };
    println!("ports: {:?}", ports);

    for _ in 0..5 {
        send(&mut nic).await;
    }

    loop {}
}
