// use std::net::Ipv4Addr;

// use async_trait::async_trait;
// use ipinfo::{IpDetails, IpError, IpInfoConfig};

// use super::Lookup;

// pub struct IpInfo {
//     backing_service: ipinfo::IpInfo,
// }

// impl IpInfo {
//     pub fn new(api_key: &str) -> IpInfo {
//         let config = IpInfoConfig {
//             token: Some(api_key.to_string()),
//             ..Default::default()
//         };

//         IpInfo {
//             backing_service: ipinfo::IpInfo::new(config).expect("Invalid configuration."),
//         }
//     }
// }

// #[async_trait]
// impl Lookup for IpInfo {
//     async fn lookup(&mut self, ip: Ipv4Addr) -> Result<IpDetails, IpError> {
//         self.backing_service.lookup(&ip.to_string()).await
//     }
// }
