use crate::dns::{header::DnsHeader, message::DnsQuestion};

use super::DnsPacket;

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

impl DnsPacket {
    pub fn builder() -> DnsPacketBuilder {
        DnsPacketBuilder::default()
    }
}
