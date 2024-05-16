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
    /// Try to resolve by creating new UdpSocket
    pub fn resolve_with_new_socket(&self, packets: Vec<Packet>) -> Vec<Packet> {
        let mut buf = [0; 512];
        let r_addr = &self
            .addr
            .parse::<SocketAddr>()
            .context(fdbg!("Unable to parse address"))
            .unwrap();
        let packets = packets
            .iter()
            .map(|p| {
                let r_socket = UdpSocket::bind("0.0.0.0:0")
                    .context(fdbg!("Unable to bind UDP socket"))
                    .unwrap();
                r_socket
                    .connect(r_addr)
                    .context(fdbg!("Unable to connect to resolver"))
                    .unwrap();
                r_socket
                    .send(&p.as_bytes())
                    .context(fdbg!("Unable to send to resolver address"))
                    .unwrap();
                let size = r_socket
                    .recv(&mut buf)
                    .context(fdbg!("Unable to receive from resolver"))
                    .unwrap();
                let mut dns_reader = DnsReader::new(&buf);
                let received_packet = Packet::parse(&mut dns_reader);
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
                tracing::debug!("Sending packet to resolver: HeaderId({})", p.header.id);
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
                debug!("Received packet from resolver: {received_packet:#?}");
                received_packet
            })
            .collect::<Vec<_>>();
        return packets;
    }
}
