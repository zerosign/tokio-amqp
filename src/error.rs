use std::convert::From;
use std::{io, error, fmt};
use url;

#[derive(Debug, Clone)]
pub enum AMQPError {
    IoError(io::ErrorKind),
    DecodeError(&'static str),
    Protocol(String),
    SchemeError(String),
    UrlParseError(url::ParseError),
    QueueEmpty,
    SyncError,
    FrameError(u8, String),
    VHostError
}

impl fmt::Display for AMQPError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "AMQP Error: {}", error::Error::description(self))
    }
}

impl error::Error for AMQPError {
    fn description<'a>(&'a self) -> &'a str {
        match *self {

        }
    }
}

impl From<io::Error> for AMQPError {
    fn from(err: io::Error) -> AMQPError {
        AMQPError::IoError(err.kind())
    }
}

impl <T> From<::std::sync::PoisonError<T>> for AMQPError {
    fn from(_: ::std::sync::PoisonError<T>) -> AMQPError { AMQPError::SyncError }
}

impl From<url::ParseError> for AMQPError {
    fn from(err: url::ParseError) -> AMQPError {
        AMQPError::UrlParseError(err)
    }
}
