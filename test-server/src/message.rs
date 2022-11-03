use bytes::{BytesMut, BufMut};

pub enum MessageKind {
    Text,
}

pub struct Message {
    contents: String,
}

impl Message {
    pub fn text(text: String) -> Self {
        Self { contents: text }
    }
}

pub trait Packet {
    fn size(&self) -> usize;
    fn serialize(&self) -> BytesMut;
}

impl Packet for Message {
    fn size(&self) -> usize {
        self.contents.len()
    }

    fn serialize(&self) -> BytesMut {
        BytesMut::with_capacity(self.size())
    }

    // fn serialize(&self) -> &[u8] {
    //     let mut buffer = BytesMut::with_capacity(self.size());
    //     buffer.put(self.contents.as_bytes());
    //     buffer
    // }
}

pub unsafe fn as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::std::mem::size_of::<T>()
    )
}

#[test]
fn serializer() {
    // let message = Message::text("Hello, world!".into());
    // let buffer = unsafe { as_u8_slice(&message) };
    let message = String::from("Hello, world!").as_bytes();
    println!("{:?}", std::mem::size_of::<Message>());
    // let original: String = unsafe { std::mem::transmute::<&[u8], String>(&*message) };
}
