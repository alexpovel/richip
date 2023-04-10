use std::net::Ipv4Addr;

use ::ipinfo::{IpDetails, IpError};
use async_trait::async_trait;

pub mod ipinfo;

#[async_trait]
pub trait LookupService {
    // Architecturally ugly, but reuse `ipinfo`'s types as they're really useful.
    // If/when adding more providers, define a type like `IpDetails` ourselves locally
    // in this repository.
    async fn lookup(&mut self, ip: Ipv4Addr) -> Result<IpDetails, IpError>;
}
