use crate::dns::header::Header;
use crate::dns::label::Label;
use crate::dns::packet::{Merge, Packet};
use crate::dns::question::Question;
use crate::dns::resolver::DnsResolver;
use crate::dns::{RecordClass, RecordType};
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

    if CliArgs::test_resolver() {
        test_resolver();
    }

    DnsServer::start(format!("0.0.0.0:{}", CliArgs::port()).as_str());
}

fn test_resolver() {
    let mut header = Header::default();
    header.id = 99;
    let packet = Packet::builder()
        .header(header)
        .question(Question {
            name: Label("codecrafters.io".to_string()),
            class: RecordClass::IN,
            typez: RecordType::A,
        })
        .build();
    let resolver_address = CliArgs::resolver().unwrap();

    let vec = DnsResolver::new(resolver_address)
        .resolve_with_new_socket(packet.split())
        .merge();

    tracing::debug!("Got response: {vec:#?}");

    panic!("EXIST NOW !!!");
}
