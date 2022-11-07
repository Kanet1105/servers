use std::{
    fmt::{Debug, Display},
    error::Error,
};

pub enum MessageError {
    ExceedBufferCapacity,
    ExceedDataLength,
}

impl Debug for MessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for MessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExceedBufferCapacity => write!(f, "The message length exceeds the buffer length."),
            Self::ExceedDataLength => write!(f, "The buffer length exceeds the data struct length."),
        }
    }
}

impl Error for MessageError {}
