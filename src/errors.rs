use std::io;
use std::error;
use std::fmt;
use protobuf;

/// Represents an error that has occurred on the server side.
#[derive(Debug)]
pub struct ServerError {
    code: u8,
    data: Vec<u8>,
}

impl ServerError {
    pub fn new<T: Into<Vec<u8>>>(error_code: u8, error_data: T) -> ServerError {
        ServerError {
            code: error_code,
            data: error_data.into(),
        }
    }
}

impl error::Error for ServerError {
    fn description(&self) -> &str {
        "a server error occurred"
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "received code {}, error was: {}",
               self.code,
               String::from_utf8_lossy(self.data.as_slice()))
    }
}

// Represents errors that can occur with this Riak client and its components.
#[derive(Debug)]
pub enum RiakErr {
    IoError(io::Error),
    ProtobufError(protobuf::ProtobufError),
    ServerError(ServerError),
}

impl fmt::Display for RiakErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RiakErr::IoError(ref err) => write!(f, "error pinging riak: {}", err),
            RiakErr::ProtobufError(ref err) => write!(f, "connection to riak terminated: {}", err),
            RiakErr::ServerError(ref err) => write!(f, "error from server: {}", err),
        }
    }
}

impl error::Error for RiakErr {
    fn description(&self) -> &str {
        match *self {
            RiakErr::IoError(ref err) => err.description(),
            RiakErr::ProtobufError(ref err) => err.description(),
            RiakErr::ServerError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            RiakErr::IoError(ref err) => Some(err),
            RiakErr::ProtobufError(ref err) => Some(err),
            RiakErr::ServerError(ref err) => Some(err),
        }
    }
}
