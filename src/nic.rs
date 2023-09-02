use rppal::uart::{Uart, Parity, Error};

// NIC 構造体
#[allow(dead_code)]
#[derive(Debug)]
pub struct Nic {
    name: String,
    nic_type: NicType,
    port: Uart,
    mtu: u16
}

#[allow(dead_code)]
impl Nic {
    // NIC 構造体の作成
    pub fn new(name: String, nic_type: NicType, mtu: u16, serial_path: String) -> Self {
        Nic { name, nic_type, port: Uart::with_path(serial_path, 115200, Parity::None, 8, 1).unwrap(), mtu }
    }

    // 別の NIC へバイト列を送信
    pub fn transmit(&mut self, data: &[u8]) -> Result<usize, Error> {
        self.port.write(data)
    }

    // 別の NIC からバイト列を受信
    pub fn recv(&mut self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![0u8; self.mtu as usize];
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
