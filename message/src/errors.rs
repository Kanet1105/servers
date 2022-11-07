use std::{
    error::Error,
    fmt::{Debug, Display},
};

pub enum MessageError {
    ExceedBufferCapacity,
    IncompleteBuffer,
}

impl Debug for MessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for MessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExceedBufferCapacity => {
                write!(f, "The message length exceeds the buffer length.")
            }
            Self::IncompleteBuffer => {
                write!(f, "The buffer length does not match the struct length.")
            }
        }
    }
}

impl Error for MessageError {}
