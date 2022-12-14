mod custom;
mod errors;

pub use bytes::{Buf, BufMut, BytesMut};
pub use custom::SimpleText;
pub use errors::*;

pub trait Message {
    fn len(&self) -> usize;
    fn serialize(&self, buffer: &mut BytesMut);
    fn deserialize(&mut self, buffer: &mut BytesMut);
}

pub fn serialize_with_capacity<T>(buffer: &mut BytesMut, data: &T) -> Result<(), MessageError>
where
    T: Message,
{
    if buffer.capacity() < data.len() {
        return Err(MessageError::ExceedBufferCapacity);
    }
    data.serialize(buffer);
    Ok(())
}

pub fn deserialize_with_capacity<T>(buffer: &mut BytesMut, data: &mut T) -> Result<(), MessageError>
where
    T: Message,
{
    data.deserialize(buffer);
    Ok(())
}
