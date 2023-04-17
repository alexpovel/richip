use std::net::IpAddr;

use crate::lookup::IpDetails;

pub mod caddy;

pub trait Process {
    fn get_ip(&self, line: &str) -> Option<IpAddr>;
    fn inject_details(&self, details: IpDetails) -> Option<String>;
}
