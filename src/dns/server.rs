use std::net::UdpSocket;

use anyhow::Context;
use tracing::{debug, error, info};

use crate::{
    common::{dns_reader::DnsReader, AsBytes, Parse},
    config::cli_args::CliArgs,
    dns::{
        answer::{Answer, RData},
        packet::Packet,
        resolver::DnsResolver,
        RecordClass, RecordType,
    },
    fdbg,
};

use super::packet::Merge;

pub struct DnsServer {}
impl DnsServer {
    pub fn start(host: &str, port: &str) {
        let addr = format!("{}:{}", host, port);
        let socket = UdpSocket::bind(addr).expect("Failed to bind to address");
        let mut buf = [0; 512];
        loop {
            let (size, source) = match socket.recv_from(&mut buf) {
                Ok((size, source)) => (size, source),
                Err(e) => {
                    error!("Error receiving data !!!, {e:#?}");
                    continue;
                }
            };
            let packet = Self::read_packet(&mut buf, size);
            let response = Self::get_response_byte(&socket, packet);
            socket
                .send_to(&response, source)
                .context(fdbg!("Failed to send response"))
                .unwrap();
        }
    }

    fn get_response_byte(socket: &UdpSocket, packet: Packet) -> Vec<u8> {
        match CliArgs::resolver() {
            None => get_mock_response_byte(packet),
            Some(addr) => DnsResolver::new(addr)
                .resolve(&socket, packet.split())
                // .resolve_with_new_socket(packet.split())
                .merge()
                .as_bytes(),
        }
    }

    fn read_packet(buf: &mut [u8], packet_size: usize) -> Packet {
        let mut dns_reader = DnsReader::new(&buf);
        Packet::parse(&mut dns_reader)
    }
}

fn get_mock_response_byte(packet: Packet) -> Vec<u8> {
    let packet = Packet::builder()
        .header(packet.header.clone())
        .answers(
            packet
                .questions
                .iter()
                .map(|q| Answer {
                    name: q.name.clone(),
                    typez: RecordType::A,
                    class: RecordClass::IN,
                    ttl: 60,
                    rdata: RData("8.8.8.8".to_string()),
                })
                .collect(),
        )
        .questions(packet.questions)
        .build();
    packet.as_bytes()
}
