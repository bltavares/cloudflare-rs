extern crate hyper;
extern crate rustc_serialize;

use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum CloudFlareErrors {
    APIError(hyper::error::Error),
    ParsingError(rustc_serialize::json::DecoderError),
}

impl fmt::Display for CloudFlareErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CloudFlareErrors::APIError(ref err) => err.fmt(f),
            CloudFlareErrors::ParsingError(ref err) => err.fmt(f),
        }
    }
}

impl Error for CloudFlareErrors {
    fn description(&self) -> &str {
        match *self {
            CloudFlareErrors::APIError(ref err) => err.description(),
            CloudFlareErrors::ParsingError(ref err) => err.description(),
        }
    }
}

impl From<hyper::error::Error> for CloudFlareErrors {
    fn from(error: hyper::error::Error) -> CloudFlareErrors {
        CloudFlareErrors::APIError(error)
    }
}

impl From<rustc_serialize::json::DecoderError> for CloudFlareErrors {
    fn from(error: rustc_serialize::json::DecoderError) -> CloudFlareErrors {
        CloudFlareErrors::ParsingError(error)
    }
}
