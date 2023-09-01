// NIC 構造体
#[allow(dead_code)]
#[derive(Debug)]
pub struct Nic {
    name: String,
    nic_type: NicType
}

impl Nic {
    // NIC 構造体の作成
    pub fn new(name: String, nic_type: NicType) -> Self {
        Nic { name, nic_type }
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
