use std::io;

use super::base::{BaseError, Error};

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        match error.kind() {
            io::ErrorKind::NotFound => Error::NotFound(BaseError {
                message: "File not found".to_string(),
                reason: error.to_string(),
            }),
            _ => Error::Internal(BaseError {
                message: "An internal error occurred".to_string(),
                reason: error.to_string(),
            }),
        }
    }
}