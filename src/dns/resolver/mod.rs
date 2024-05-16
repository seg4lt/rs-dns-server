use anyhow::bail;
use tracing::debug;

use crate::common::{dns_reader::DnsReader, AsBytes, Parse};

use super::{header::Header, packet::Packet, question::Question};
use std::net::{SocketAddr, UdpSocket};

pub struct DnsResolver {
    addr: String,
}

impl DnsResolver {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
    pub fn resolve(&self, socket: &UdpSocket, packets: Vec<Packet>) -> Vec<Packet> {
        tracing::debug!("resolving packets: {packets:#?}");
        let packets = packets
            .iter()
            .map(|p| {
                let Ok(r_socket) = UdpSocket::bind("0.0.0.0:0") else {
                    tracing::error!("Unable to bind UDP socket");
                    panic!("Unable to bind UDP socket");
                };
                let mut buf = [0; 512];
                let r_addr = &self.addr.parse::<SocketAddr>().unwrap();
                tracing::info!("sending packet to resolver: {r_addr:#?} -- {p:#?}");
                let Ok(_) = r_socket.send_to(&p.as_bytes(), r_addr) else {
                    tracing::error!("couldn't send to resolver");
                    panic!("couldn't send to resolver")
                };
                match r_socket.recv_from(&mut buf) {
                    Ok((received, addr)) => {
                        debug!(
                            "resolver received from {addr:?} - {received} bytes {:?}",
                            &buf[..received]
                        );
                        let mut dns_reader = DnsReader::new(&buf);
                        let received_packet = Packet::parse(&mut dns_reader);
                        debug!("received packet from resolver: {received_packet:#?}");
                        received_packet
                    }
                    Err(e) => {
                        panic!("couldn't receive from resolver: {e}")
                    }
                }
            })
            .collect::<Vec<_>>();
        return packets;
    }

    fn map_question_to_packet(&self, header: &Header, question: &Question) -> Packet {
        let random_num: u16 = rand::random();
        Packet::builder()
            .header(Header {
                id: random_num,
                qr: header.qr,
                opcode: header.opcode,
                aa: header.aa,
                tc: header.tc,
                rd: header.rd,
                ra: header.ra,
                z: header.z,
                rcode: header.rcode,
                qdcount: 1,
                ancount: 0,
                nscount: 0,
                arcount: 0,
            })
            .question(question.clone())
            .build()
    }
}
