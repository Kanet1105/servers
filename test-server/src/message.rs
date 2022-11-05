use bytes::{BytesMut, BufMut};
use tokio::io::copy;

pub trait Packet {
    fn len(&self) -> usize;
    fn serialize(&self) -> &[u8];
}

pub struct Message {
    concurrency: usize,
    requests: usize,
    elapsed: f32,
    contents: String,
}

// impl Packet for Message {
//     fn len(&self) -> usize {
//         (8 + 8 + 4) + self.contents.len() * 8
//     }

//     fn serialize(&self, buffer: &mut buffer) -> &[u8] {
//         buffer.put_u8(self.concurrency);
//         buffer.put_u8(self.requests);
//         buffer.put_f32(self.elapsed);
//     }
// }

#[test]
fn serialization() {
    let a = String::from("12345");
    let mut buffer = BytesMut::with_capacity(20);
    buffer.put_u64(1);
    buffer.put_u64(13);
    buffer.put_f32(1.252);
    buffer.put_slice(a.as_bytes());
    println!("{:?}", buffer);
    println!("{:?}", buffer.len());
}

#[test]
fn serialize() {
    let a: usize = 1;
    let mut buffer_1 = BytesMut::with_capacity(1);
    let mut buffer_2 = BytesMut::with_capacity(1);
    buffer_1.put_slice(&a.to_be_bytes());
    buffer_2.put_u64(1);
    println!("{:?}", buffer_1);
    println!("{:?}", buffer_2);
    assert_eq!(buffer_1, buffer_2);
}

#[test]
fn deserialize() {
    let mut buffer = BytesMut::with_capacity(1);
    buffer.put_slice(&(123 as u64).to_be_bytes());
    buffer.put_slice(&(456 as u64).to_be_bytes());
    
}
