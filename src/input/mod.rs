use std::{error::Error, fmt::Display, net::IpAddr};

use crate::lookup::IpDetails;

pub mod caddy;

#[derive(Debug)]
pub enum LogLineError {
    ParseLine(ParseLineError),
    NotFound(NotFoundError),
    ParseIP(ParseIPError),
    FoundButInvalid(FoundButInvalidError),
}

impl Display for LogLineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLineError::ParseLine(err) => Display::fmt(err, f),
            LogLineError::NotFound(err) => Display::fmt(err, f),
            LogLineError::ParseIP(err) => Display::fmt(err, f),
            LogLineError::FoundButInvalid(err) => Display::fmt(err, f),
        }
    }
}

impl Error for LogLineError {}

#[derive(Debug)]
pub struct ParseLineError {
    pub reason: String,
}

impl Display for ParseLineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error while parsing: {}", self.reason)
    }
}

#[derive(Debug)]
pub struct NotFoundError {
    /// Query that didn't find any results (regex, JSON pointer, ...).
    pub query: String,
}

impl Display for NotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "No results for query '{}'", self.query)
    }
}

#[derive(Debug)]
pub struct FoundButInvalidError {
    pub reason: String,
}

impl Display for FoundButInvalidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Found item, but it's invalid: '{}'", self.reason)
    }
}

#[derive(Debug)]
pub struct ParseIPError {
    pub reason: String,
}

impl Display for ParseIPError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unable to parse IP, reason: {}", self.reason)
    }
}

pub trait LogLine {
    fn get_ip(&self, line: &str) -> Result<IpAddr, LogLineError>;
    fn inject_details(&self, details: IpDetails) -> Option<String>;
}
