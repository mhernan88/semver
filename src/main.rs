use semver::handlers;

extern crate clap;
use clap::{Arg, App};
use log::{info};


fn main() {
    env_logger::init();
    info!("semver v0.1.0");

    let matches = App::new("semver")
        .subcommand(App::new("increment")
                    .about("Increment Up Version")
                    .subcommand(App::new("major")
                                .about("Increment Major Version")
                                .arg(Arg::new("version")
                                     .long("version")
                                     .short('v')
                                     .takes_value(true)
                                )
                    )
                    .subcommand(App::new("minor")
                                .about("Increment Minor Version")
                                .arg(Arg::new("version")
                                     .long("version")
                                     .short('v')
                                     .takes_value(true)
                                )
                    )
                    .subcommand(App::new("patch")
                                .about("Increment Patch Version")
                                .arg(Arg::new("version")
                                     .long("version")
                                     .short('v')
                                     .takes_value(true)
                                )
                    )
        )
        .get_matches();
    handlers::increment::handle(&matches);
}
