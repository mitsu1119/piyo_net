use std::io;
use bytes::{BytesMut, BufMut};

use tokio_util::codec::{Encoder, Decoder, Framed};

use tokio_serial::{SerialPortBuilderExt, SerialStream};

// シリアル通信のコーデック
#[derive(Debug)]
struct NicCodec;

impl Decoder for NicCodec {
    type Item = Vec<u8>;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        println!("decode {:?}", src);
        Ok(Some(src.to_vec()))
    }
}

impl Encoder<Vec<u8>> for NicCodec {
    type Error = io::Error;

    fn encode(&mut self, item: Vec<u8>, dst: &mut BytesMut) -> Result<(), Self::Error> {
        println!("In write {:?}", &item);
        dst.reserve(item.len());
        dst.put(&item[..]);
        Ok(())
    }
}

// NIC 構造体
#[allow(dead_code)]
#[derive(Debug)]
pub struct Nic {
    name: String,
    nic_type: NicType,
    mtu: u16,
    stream: Framed<SerialStream, NicCodec> 
}

#[allow(dead_code)]
impl Nic {
    // NIC 構造体の作成
    pub async fn new(name: String, nic_type: NicType, mtu: u16, serial_path: String) -> tokio_serial::Result<Nic> {
        let mut port = tokio_serial::new(serial_path, 115200).open_native_async()?;
        port.set_exclusive(false)?;

        let stream = NicCodec.framed(port);

        Ok(Nic { name, nic_type, mtu, stream })
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
