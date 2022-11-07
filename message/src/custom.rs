use crate::{Buf, BufMut};
use std::str::from_utf8;

#[repr(C)]
#[derive(Debug)]
pub struct SimpleText {
    channel: String,
    contents: String,
}

impl super::Message for SimpleText {
    fn len(&self) -> usize {
        (self.channel.len() * 8) + (self.contents.len() * 8)
    }

    fn serialize(&self, buffer: &mut bytes::BytesMut) {
        buffer.put_u64(self.channel.len() as u64);
        buffer.put_slice(self.channel.as_bytes());

        buffer.put_u64(self.contents.len() as u64);
        buffer.put_slice(self.contents.as_bytes());
    }

    fn deserialize(&mut self, buffer: &mut bytes::BytesMut) {
        let channel_len = buffer.get_u64() as usize;
        self.channel = from_utf8(&buffer[0..channel_len]).unwrap().into();
        buffer.advance(channel_len);
        
        let contents_len = buffer.get_u64() as usize;
        self.contents = from_utf8(&buffer[0..contents_len]).unwrap().into();
        buffer.advance(contents_len);
    }
}

#[test]
fn example() {
    use bytes::BytesMut;
    
    let a = "Hello, world".to_string();
    let mut buffer = BytesMut::new();

    // Serialize
    buffer.put_u64(a.len() as u64);
    println!("{:?}", buffer);
    buffer.put_slice(a.as_bytes());
    println!("{:?}", buffer);
    println!("{:?}", buffer.len());

    // Deserialize
    let str_len = buffer.get_u64() as usize;
    let message = std::str::from_utf8(&buffer[0..str_len]).unwrap();
    println!("{:?}", message);
    buffer.advance(str_len);
    println!("{:?}", buffer);
    println!("{:?}", buffer.len());
}
