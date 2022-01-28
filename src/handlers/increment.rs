extern crate clap;
use clap::{ArgMatches};
use log::{debug};
use super::super::client::semver_client;

pub fn handle(matches: &ArgMatches) {
    debug!("handling action");
    if let Some(increment) = matches.subcommand_matches("increment") {
        handle_increment(increment);
    }
}

pub fn handle_increment(matches: &ArgMatches) {
    debug!("handling increment");
    if let Some(major) = matches.subcommand_matches("major")  {
        handle_major_increment(major);
    }
}

pub fn handle_major_increment(matches: &ArgMatches) {
    debug!("handling major increment");
    let version = matches.value_of("version").unwrap().to_string();

    debug!("parsing version");
    let mut semver_client = semver_client::new(None, None, None);
    let res1 = semver_client.from_string(version);
    let res2 = semver_client.increment_major();
    let res3 = semver_client.to_string();

}
