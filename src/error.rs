use std;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    LibFtdi(LibFtdiError),
    MallocFailure,
}

#[derive(Debug)]
pub struct LibFtdiError {
    err_str : String,
}

impl LibFtdiError {
    pub fn new<S: Into<String>>(err_str : S) -> LibFtdiError {
        LibFtdiError {
            err_str : err_str.into(),
        }
    }
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::LibFtdi(_) => {
                write!(f, "libftdi-internal error")
            },
            Error::MallocFailure => {
                write!(f, "malloc() failure")
            }
        }
    }
}

impl fmt::Display for LibFtdiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.err_str)
    }
}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            Error::LibFtdi(ref ftdi_err) => {
                Some(ftdi_err)
            },
            Error::MallocFailure => {
                None
            }
        }
    }
}

impl std::error::Error for LibFtdiError {}
