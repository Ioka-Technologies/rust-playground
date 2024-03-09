use std::fmt::{self, Display, Formatter};

pub type Message = String;
pub type Reason = String;

#[derive(Debug, PartialEq)]
pub struct BaseError {
    pub message: Message,
    pub reason: Reason,
}

impl Display for BaseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Error: {} - {}", self.message, self.reason)
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    NotFound(BaseError),
    Internal(BaseError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Error::NotFound(err) => write!(f, "Not found error: {}", err),
            Error::Internal(err) => write!(f, "Internal error: {}", err),
        }
    }
}