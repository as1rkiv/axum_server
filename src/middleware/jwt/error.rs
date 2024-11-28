#![allow(dead_code)]

use crate::common::error::Error;

#[derive(Debug)]
pub enum AuthError {
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl From<AuthError> for Error {
    fn from(_: AuthError) -> Self {
        Error::Auth("请重新登录")
    }
}
