mod nic;
use nic::{ Nic, NicType };

fn main() {
    let mut nic = Nic::new("ttyAMA0".to_string(), NicType::ETHERNET, "/dev/ttyAMA0".to_string());

    println!("{:?}", nic);

    nic.transmit(&[1, 2, 3, 4, 5]);

    loop {
        println!("{:?}", nic.recv());
    }
}
