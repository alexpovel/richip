use std::{env, io, net::Ipv4Addr, str::FromStr};

use lookup::LookupService;

mod lookup;

#[tokio::main]
async fn main() {
    let handle = io::stdin();

    let mut lookup_service =
        lookup::ipinfo::IpInfo::new(&env::var("IPINFO_API_KEY").expect("Need IP info API key."));

    for line in handle.lines() {
        let Ok(input) = line else {
            eprintln!("Unable to read input line, continuing.");
            continue;
        };

        let Ok(ip) = Ipv4Addr::from_str(&input) else {
            eprintln!("Unable to parse {input} as IPv4, continuing.");
            continue;
        };

        let Ok(details) = lookup_service.lookup(ip).await else {
            eprintln!("Unable to look up {ip}, continuing.");
            continue;
        };

        println!("{details:?}");
    }
}
