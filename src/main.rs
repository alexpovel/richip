use std::io;

use input::{caddy::Caddy, Process};
use jsonptr::Pointer;

use crate::cli::build_cli;

mod cli;
mod input;
mod lookup;

#[tokio::main]
async fn main() {
    let cli = build_cli();

    let process: Box<dyn Process> = match cli.subcommand() {
        Some(("caddy", matches)) => Box::new(Caddy::new(
            matches
                .get_one::<Pointer>("input")
                .expect("Default value missing.")
                .to_owned(),
            matches
                .get_one::<Pointer>("output")
                .expect("Default value missing.")
                .to_owned(),
        )),
        _ => panic!("Unsupported CLI subcommand."),
    };

    // // let mut lookup_service =
    // //     lookup::ipinfo::IpInfo::new(&env::var("IPINFO_API_KEY").expect("Need IP info API key."));

    for line in io::stdin().lines() {
        let Ok(input) = line else {
            eprintln!("Unable to read input line, continuing.");
            continue;
        };

        let ip = process.get_ip(&input);

        match ip {
            Some(value) => {
                println!("{}", value)
            }
            None => {
                println!("No IP found.")
            }
        }
    }

    // let Ok(details) = lookup_service.lookup(ip).await else {
    //     eprintln!("Unable to look up {ip}, continuing.");
    //     continue;
    // };
}
