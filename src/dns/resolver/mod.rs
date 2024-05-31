use anyhow::{bail, Context};
use tracing::debug;

use crate::{
    common::{dns_reader::DnsReader, AsBytes, Parse},
    fdbg,
};

use super::{header::Header, packet::Packet, question::Question};
use std::net::{SocketAddr, UdpSocket};

pub struct DnsResolver {
    addr: String,
}

impl DnsResolver {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
    /// Try to solve by creating new UdpSocket
    pub fn resolve_with_new_socket(&self, packets: Vec<Packet>) -> Vec<Packet> {
        let r_socket = UdpSocket::bind("0.0.0.0:0")
            .context(fdbg!("Unable to bind UDP socket"))
            .unwrap();
        r_socket
            .connect(&self.addr)
            .context(fdbg!("Unable to connect to resolver address"))
            .unwrap();
        let mut buf = [0; 512];
        let packets = packets
            .iter()
            .map(|p| {
                debug!("Sending packet to resolver {}: {p:#?}", self.addr.clone());
                r_socket
                    .send(&p.as_bytes())
                    .context(fdbg!(
                        "Unable to send to resolver address: {}",
                        self.addr.clone()
                    ))
                    .unwrap();
                debug!("Receiving packet from resolver");
                let size = r_socket
                    .recv(&mut buf)
                    .context(fdbg!("Unable to receive from resolver"))
                    .unwrap();
                debug!(
                    "Received packet from resolver with size: {:?}",
                    &buf[0..size]
                );
                let mut dns_reader = DnsReader::new(&buf);
                let received_packet = Packet::parse(&mut dns_reader);
                assert_eq!(p.header.id, received_packet.header.id);
                debug!("received packet from resolver: {received_packet:#?}");
                received_packet
            })
            .collect::<Vec<_>>();
        return packets;
    }

    /// Try to resolve with existing socket
    pub fn resolve(&self, socket: &UdpSocket, packets: Vec<Packet>) -> Vec<Packet> {
        let mut buf = [0; 512];
        let r_addr = &self.addr.parse::<SocketAddr>().unwrap();
        let packets = packets
            .iter()
            .map(|p| {
                debug!("Sending packet to resolver: {p:#?}");
                socket
                    .send_to(&p.as_bytes(), r_addr)
                    .context(fdbg!("Unable to send to resolver address"))
                    .unwrap();
                let size = socket
                    .recv_from(&mut buf)
                    .context(fdbg!("Unable to receive from resolver"))
                    .unwrap();
                let mut dns_reader = DnsReader::new(&buf);
                let received_packet = Packet::parse(&mut dns_reader);
                assert_eq!(p.header.id, received_packet.header.id);
                debug!("Received packet from resolver: {received_packet:#?}");
                received_packet
            })
            .collect::<Vec<_>>();
        return packets;
    }
}
