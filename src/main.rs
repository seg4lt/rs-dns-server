use std::net::UdpSocket;

use tracing::{error, info};

use crate::{
    common::AsBytes,
    config::setup_log,
    dns::{header::DnsHeader, packet::DnsPacket},
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

                let packet = DnsPacket {
                    header: DnsHeader {
                        id: 1234,
                        qr: 1,
                        opcode: 0,
                        aa: 0,
                        tc: 0,
                        rd: 0,
                        ra: 0,
                        z: 0,
                        rcode: 0,
                        qdcount: 0,
                        ancount: 0,
                        nscount: 0,
                        arcount: 0,
                    },
                };
                let response = packet.as_bytes();

                // let response = [];
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
