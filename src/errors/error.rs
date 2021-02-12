use std::convert::From;
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum CommitError {
    Setup,
    Commit,
    Open,
}

impl Error for CommitError {}

impl fmt::Display for CommitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CommitError::Setup => write!(f, "Setup Error"),
            CommitError::Commit => write!(f, "Commit Error"),
            CommitError::Open => write!(f, "Open Error"),
        }
    }
}
