extern crate clap;

use clap::{ArgMatches};
use log::{debug, error};
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
    } else if let Some(minor) = matches.subcommand_matches("minor") {
        handle_minor_increment(minor);
    } else if let Some(patch) = matches.subcommand_matches("patch") {
        handle_patch_increment(patch);
    }
}

pub fn handle_major_increment(matches: &ArgMatches) {
    debug!("handling major increment");
    let version = matches.value_of("version").unwrap().to_string();

    debug!("parsing version");
    let mut semver_client = semver_client::new(None, None, None);
    match semver_client.from_string(version) {
        Ok(_v) => {
            debug!("successfully parsed version");
        }

        Err(e) => {
            error!("failed to parse version: {}", e);
            return
        }
    }

    debug!("incrementing major version");
    match semver_client.increment_major() {
        Ok(_v) => {
            debug!("successfully incremented major version");
        }

        Err(e) => {
            error!("failed to increment major version: {}", e);
            return
        }
    }

    semver_client.to_string();
}

pub fn handle_minor_increment(matches: &ArgMatches) {
    debug!("handling minor increment");
    let version = matches.value_of("version").unwrap().to_string();

    debug!("parsing version");
    let mut semver_client = semver_client::new(None, None, None);
    match semver_client.from_string(version) {
        Ok(_v) => {
            debug!("successfully parsed version");
        }

        Err(e) => {
            error!("failed to parse version: {}", e);
            return
        }
    }

    debug!("incrementing minor version");
    match semver_client.increment_minor() {
        Ok(_v) => {
            debug!("successfully incremented minor version");
        }

        Err(e) => {
            error!("failed to increment minor version: {}", e);
            return
        }
    }

    semver_client.to_string();
}

pub fn handle_patch_increment(matches: &ArgMatches) {
    debug!("handling patch increment");
    let version = matches.value_of("version").unwrap().to_string();

    debug!("parsing version");
    let mut semver_client = semver_client::new(None, None, None);
    match semver_client.from_string(version) {
        Ok(_v) => {
            debug!("successfully parsed version");
        }

        Err(e) => {
            error!("failed to parse version: {}", e);
            return
        }
    }

    debug!("incrementing patch version");
    match semver_client.increment_patch() {
        Ok(_v) => {
            debug!("successfully incremented patch version");
        }

        Err(e) => {
            error!("failed to increment patch version: {}", e);
            return
        }
    }

    semver_client.to_string();
}
