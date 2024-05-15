use std::net::UdpSocket;

use tracing::{debug, error, info};

use crate::{
    common::{dns_reader::DnsReader, AsBytes, Parse},
    dns::{
        answer::{Answer, RData},
        packet::Packet,
        RecordClass, RecordType,
    },
};

pub struct DnsServer {}
impl DnsServer {
    pub fn start(host: &str, port: &str) {
        let udp_socket =
            UdpSocket::bind(format!("{}:{}", host, port)).expect("Failed to bind to address");
        let mut buf = [0; 512];

        loop {
            let Ok((size, source)) = udp_socket.recv_from(&mut buf) else {
                error!("Error receiving data");
                break;
            };

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
    }
}
