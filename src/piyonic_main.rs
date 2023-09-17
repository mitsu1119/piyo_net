use piyo_net::piyo_nic::{Nic, NicType};

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

    for _ in 0..5 {
        send(&mut nic).await;
    }

    loop {}
}
