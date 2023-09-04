use std::io;
use bytes::{BytesMut, BufMut};

use futures::{stream::{StreamExt, SplitSink}, SinkExt};

use tokio::time::{sleep, Duration};
use tokio_util::codec::{Encoder, Decoder, Framed};

use tokio_serial::{SerialPortBuilderExt, SerialStream};

// シリアル通信のコーデック
#[derive(Debug, Copy, Clone)]
struct NicCodec;

impl Decoder for NicCodec {
    type Item = Vec<u8>;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() == 0 {
            Ok(None)
        } else {
            Ok(Some(src.split_to(src.len()).to_vec()))
        }
    }
}

impl Encoder<Vec<u8>> for NicCodec {
    type Error = io::Error;

    fn encode(&mut self, item: Vec<u8>, dst: &mut BytesMut) -> Result<(), Self::Error> {
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
    tx: SplitSink<Framed<SerialStream, NicCodec>, Vec<u8>>,
}

#[allow(dead_code)]
impl Nic {
    // NIC 構造体の作成
    pub fn new(name: String, nic_type: NicType, mtu: u16, serial_path: String) -> tokio_serial::Result<Nic> {
        let mut port = tokio_serial::new(serial_path, 115200).open_native_async()?;
        port.set_exclusive(false)?;

        let stream = NicCodec.framed(port);
        let (mut tx, mut rx) = stream.split();

        tokio::spawn(async move {
            loop {
                let item = rx.next()
                    .await
                    .unwrap()
                    .unwrap();
                println!("{:?}", item);
            }
        });

        Ok(Nic { name, nic_type, mtu, tx })
    }

    // NIC から出力
    pub async fn send(&mut self, data: Vec<u8>) -> tokio_serial::Result<()> {
        sleep(Duration::from_millis(1)).await;
        self.tx.send(data).await?;
        sleep(Duration::from_millis(1)).await;

        Ok(())
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
