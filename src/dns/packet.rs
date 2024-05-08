use crate::common::AsBytes;

use super::header::DnsHeader;
use super::message::DnsQuestion;

pub struct DnsPacket {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
}
impl DnsPacket {
    pub fn builder() -> DnsPacketBuilder {
        DnsPacketBuilder::default()
    }
}
impl AsBytes for DnsPacket {
    fn as_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::with_capacity(512);
        buf.extend(self.header.as_bytes());
        self.questions.iter().for_each(|q| buf.extend(q.as_bytes()));
        buf
    }
}

#[derive(Default)]
pub struct DnsPacketBuilder {
    questions: Vec<DnsQuestion>,
}

impl DnsPacketBuilder {
    pub fn add_question(mut self, question: DnsQuestion) -> Self {
        self.questions.push(question);
        self
    }
    pub fn build(self) -> DnsPacket {
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
                qdcount: self.questions.len() as u16,
                ancount: 0,
                nscount: 0,
                arcount: 0,
            },
            questions: self.questions,
        };
        packet
    }
}
