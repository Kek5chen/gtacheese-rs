use std::fmt::{Debug, Display, Formatter};
use thiserror::Error;
use crate::cheese::mem::signatures::SignatureError;

#[derive(Error, Debug)]
pub enum MemoryError {
    SignatureError(#[from] SignatureError),
}

impl Display for MemoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryError::SignatureError(msg) => write!(f, "{msg}"),
        }
    }
}
