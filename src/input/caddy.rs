use std::{net::IpAddr, str::FromStr};

use jsonptr::{Assign, Pointer};
use serde_json::{json, Value};

use crate::lookup::IpDetails;

use super::{LogLine, LogLineError};

#[derive(Debug)]
pub struct Caddy {
    pub input: Pointer,
    pub output: Pointer,
    current: Option<Value>,
}

impl Caddy {
    pub fn new(input: Pointer, output: Pointer) -> Caddy {
        Caddy {
            input,
            output,
            current: None,
        }
    }
}

impl LogLine for Caddy {
    fn get_ip(&self, line: &str) -> Result<IpAddr, LogLineError> {
        let json = match serde_json::from_str::<Value>(line) {
            Ok(value) => value,
            Err(err) => {
                return Err(LogLineError::ParseLine(super::ParseLineError {
                    reason: err.to_string(),
                }))
            }
        };

        let Some(value) = json.pointer(&self.input) else {
            return Err(LogLineError::NotFound(super::NotFoundError {
                query: self.input.to_string()
            }));
        };

        match value {
            Value::String(str) => {
                // Gets rid of `"quotes"` normally present.
                let jsonstr = str.as_str();

                match IpAddr::from_str(jsonstr) {
                    Ok(ip) => Ok(ip),
                    Err(addr_parse_err) => {
                        Err(LogLineError::ParseIP(super::ParseIPError {
                            reason: format!(
                                "{}: '{}'",
                                addr_parse_err, jsonstr
                            ),
                        }))
                    }
                }
            }
            _ => Err(LogLineError::FoundButInvalid(
                super::FoundButInvalidError {
                    reason: format!(
                        "Item '{}' at '{}' is not a JSON string.",
                        value, self.input
                    ),
                },
            )),
        }
    }

    fn inject_details(&self, _details: IpDetails) -> Option<String> {
        let Some(mut current) = self.current.to_owned() else {
            return None;
        };

        match current.assign(&self.output, json!("Implement me.")) {
            Ok(_) => Some(current.to_string()),
            Err(_) => None,
        }
    }
}
