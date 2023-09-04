mod nic;
use nic::{Nic, NicType};

const DEFAULT_TTY: &str = "/dev/ttyAMA0";

#[tokio::main]
async fn main() {
    let mut nic = Nic::new("ttyAMA0".to_string(), NicType::ETHERNET, 8, DEFAULT_TTY.to_string())
        .expect("Failed to create NIC");
    println!("{:?}", nic);

    nic.send(vec![1,2,3,4,5])
        .await
        .expect("Failed to send");
    loop {}
}
