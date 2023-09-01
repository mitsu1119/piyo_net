mod nic;
use nic::{ Nic, NicType };

fn main() {
    let nic = Nic::new("ttyAMA0".to_string(), NicType::ETHERNET);

    println!("{:?}", nic);
}
