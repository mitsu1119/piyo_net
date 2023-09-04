mod nic;
use nic::{Nic, NicType};

const DEFAULT_TTY: &str = "/dev/ttyAMA0";

#[tokio::main]
async fn main() {
    let nic = Nic::new("ttyAMA0".to_string(), NicType::ETHERNET, 8, DEFAULT_TTY.to_string())
        .await
        .expect("Failed to create NIC");
    println!("{:?}", nic);
}
