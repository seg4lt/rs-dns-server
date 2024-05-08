use std::net::UdpSocket;

use tracing::{error, info};

use crate::{
    common::AsBytes,
    config::setup_log,
    dns::{
        message::{DnsQuestion, DnsRecordClass, DnsRecordType},
        packet::DnsPacket,
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

                let packet = DnsPacket::builder()
                    .add_question(DnsQuestion {
                        name: "codecrafters.io".to_string(),
                        record_class: DnsRecordClass::IN,
                        record_type: DnsRecordType::A,
                    })
                    .build();
                let response = packet.as_bytes();
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
