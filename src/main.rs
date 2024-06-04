use crate::{
    config::{cli_args::CliArgs, setup_log},
    dns::server::DnsServer,
};

mod common;
mod config;
mod dns;

fn main() {
    setup_log().expect("Failed to setup log");
    CliArgs::init();

    DnsServer::start(format!("0.0.0.0:{}", CliArgs::port()).as_str());
}
