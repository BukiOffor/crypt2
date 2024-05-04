use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum Crypt2Errors{
    WordCountCouldNotBeParsed(u8)
}

impl Display for Crypt2Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for Crypt2Errors {}