use crate::common::AsBytes;

use super::header::DnsHeader;
use super::message::DnsQuestion;

pub mod packet_builder;

pub struct DnsPacket {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
}

impl AsBytes for DnsPacket {
    fn as_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::with_capacity(512);
        buf.extend(self.header.as_bytes());
        self.questions.iter().for_each(|q| buf.extend(q.as_bytes()));
        buf
    }
}
