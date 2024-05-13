use std::net::UdpSocket;

use tracing::{debug, error, info};

use crate::{
    common::{AsBytes, DnsReader, Parse},
    config::setup_log,
    dns::{
        answer::{Answer, RData},
        packet::Packet,
        RecordClass, RecordType,
    },
};

mod common;
mod config;
mod dns;

fn main() {
    setup_log().expect("Failed to setup log");
    info!("Logs from your program will appear here!");

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                info!("Received {} bytes from {}", size, source);
                debug!("Received buffer {:?}", &buf[0..size]);
                let mut dns_reader = DnsReader::new(&buf);
                let received_packet = Packet::parse(&mut dns_reader);
                tracing::debug!("Received packet: {:#?}", received_packet);

                let packet = Packet::builder()
                    .header(received_packet.header)
                    .answers(
                        received_packet
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
                    .questions(received_packet.questions)
                    .build();
                tracing::debug!("Response packet: {:#?}", packet);
                let response = packet.as_bytes();
                tracing::debug!("Response bytes: {:?}", response);
                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                error!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
