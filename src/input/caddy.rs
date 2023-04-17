use std::{net::IpAddr, str::FromStr};

use jsonptr::{Assign, Pointer};
use serde_json::{json, Value};

use crate::lookup::IpDetails;

use super::Process;

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

impl Process for Caddy {
    fn get_ip(&self, line: &str) -> Option<IpAddr> {
        let json: Result<Value, serde_json::Error> = serde_json::from_str(line);

        let Ok(json) = json else {
            return None;
        };

        let Some(value) = json.pointer(&self.input) else {
            return None;
        };

        match value {
            Value::String(str) => match IpAddr::from_str(str.as_str()) {
                Ok(ip) => Some(ip),
                Err(_) => None,
            },
            _ => None,
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
