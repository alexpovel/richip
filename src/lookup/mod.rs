use std::net::IpAddr;

use ::ipinfo::IpError;
use async_trait::async_trait;

pub mod ipinfo;

#[derive(Debug)]
pub struct City {
    pub name: String,
}

#[derive(Debug)]
pub struct Region {
    pub name: String,
}

#[derive(Debug)]
pub struct Country {
    pub name: String,
}

#[derive(Debug)]
pub struct IpDetails {
    pub ip: IpAddr,
    pub hostname: Option<String>,
    pub city: Option<City>,
    pub region: Option<Region>,
    pub country: Option<Country>,
}

#[async_trait]
pub trait Lookup {
    async fn lookup(&self, ip: IpAddr) -> Result<IpDetails, IpError>;
}
