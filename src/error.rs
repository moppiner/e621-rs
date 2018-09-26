use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct E621Error {
    message: String,
}

impl E621Error {
    pub fn new(msg: String) -> E621Error {
        E621Error {
            message: msg,
        }
    }
}

impl Error for E621Error {
    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl Display for E621Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
