use std::fmt::Display;

use clap::{command, Arg, Command};
use jsonptr::{MalformedPointerError, Pointer};

pub fn build_cli() -> clap::ArgMatches {
    command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("caddy")
                .about("Works on Caddy structured log files.")
                .arg(
                    Arg::new("input")
                        .short('i')
                        .long("input")
                        .help("JSON Pointer (RFC 6901) to field containing input IP.")
                        .default_value("/request/remote_ip")
                        .value_parser(parse_to_json_pointer),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .help("JSON Pointer (RFC 6901) to destination for IP details.")
                        .default_value("/request/ip_details")
                        .value_parser(parse_to_json_pointer),
                ),
        )
        .get_matches()
}

#[derive(Debug)]
struct JSONPointerError(MalformedPointerError);

impl Display for JSONPointerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

// `MalformedPointerError` does this already, but it somehow doesn't propagate to here,
// so that `clap` complains about our value parser function signature being incorrect
// (not implementing `std::error::Error`).
impl std::error::Error for JSONPointerError {}

fn parse_to_json_pointer(arg: &str) -> Result<Pointer, JSONPointerError> {
    match Pointer::try_from(arg) {
        Ok(pointer) => Ok(pointer),
        Err(e) => Err(JSONPointerError(e)),
    }
}
