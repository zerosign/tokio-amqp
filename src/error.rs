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
    FrameError(String),
    VHostError,
}

#[derive(Debug, Clone)]
pub enum ExchangeError {
    PrecondFailed,
    NotAllowed,
    CommandInvalid,
    AccessRefused,
    NotFound
}

impl fmt::Display for ExchangeError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Exchange Error: {}", error::Error::description(self))
    }
}


impl fmt::Display for AMQPError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "AMQP Error: {}", error::Error::description(self))
    }
}

impl error::Error for AMQPError {
    fn description<'a>(&'a self) -> &'a str {
        match *self {
            AMQPError::IoError(_) => "IoError",
            AMQPError::DecodeError(err) => err,
            AMQPError::Protocol(ref err) => err,
            AMQPError::SchemeError(ref err) => err,
            AMQPError::UrlParseError(_) => "URL parsing error",
            AMQPError::QueueEmpty => "Queue is empty",
            AMQPError::SyncError => "Synchronisation error",
            AMQPError::FrameError(ref err) => err,
            AMQPError::VHostError => "Access to vhost is denied for a current user",
        }
    }
}

impl error::Error for ExchangeError {
    fn description<'a>(&'a self) -> &'a str {
        match *self {
            ExchangeError::PrecondFailed => "precondition failed",
            ExchangeError::NotAllowed => "not allowed",
            ExchangeError::CommandInvalid => "command invalid",
            ExchangeError::AccessRefused => "access refused",
            ExchangeError::NotFound => "not found"
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
