use tracing::info;

use crate::{
    config::{cli_args::CliArgs, setup_log},
};

mod common;
mod config;
mod dns;

fn main() {
    setup_log().expect("Failed to setup log");
    CliArgs::init();

    info!("Logs from your program will appear here!");
    DnsServer::start("127.0.0.1", "2053");
}
