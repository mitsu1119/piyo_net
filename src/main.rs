mod nic;
use nic::{ Nic, NicType };

fn main() {
    let mut nic = Nic::new("ttyAMA0".to_string(), NicType::ETHERNET, 16, "/dev/ttyAMA0".to_string());

    println!("{:?}", nic);

    let payload: &[u8] = &[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];
    nic.transmit(payload);

    loop {
        println!("{:?}", nic.recv());
    }
}
