use rppal::uart::{Uart, Parity, Error};

// NIC 構造体
#[allow(dead_code)]
#[derive(Debug)]
pub struct Nic {
    name: String,
    nic_type: NicType,
    port: Uart
}

#[allow(dead_code)]
impl Nic {
    // NIC 構造体の作成
    pub fn new(name: String, nic_type: NicType, serial_path: String) -> Self {
        Nic { name, nic_type, port: Uart::with_path(serial_path, 9600, Parity::None, 8, 1).unwrap() }
    }

    // 別の NIC へバイト列を送信
    pub fn transmit(&mut self, data: &[u8]) -> Result<usize, Error> {
        self.port.write(data)
    }

    // 別の NIC からバイト列を受信
    pub fn recv(&mut self) -> [u8; 10] {
        let mut buf = [0u8; 10];
        self.port.read(&mut buf).expect("Could not recv");
        buf
    }
}

#[repr(u8)]
#[allow(dead_code)]
#[derive(Debug)]
pub enum NicType {
    DUMMY = 0,
    LOOPBACK = 1,
    ETHERNET = 2
}
