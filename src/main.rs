use structopt::StructOpt;

use crate::config::Config;

mod config;

fn main() {
    let config = Config::from_args();

    dbg!(config);
}
