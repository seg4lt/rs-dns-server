use crate::dns::{answer::Answer, header::Header, question::Question};

use super::Packet;

#[derive(Default)]
pub struct PacketBuilder {
    header: Header,
    questions: Vec<Question>,
    answers: Vec<Answer>,
}

impl PacketBuilder {
    pub fn header(mut self, header: Header) -> Self {
        self.header = header;
        self
    }
    pub fn question(mut self, question: Question) -> Self {
        self.questions.push(question);
        self
    }
    pub fn answer(mut self, answer: Answer) -> Self {
        self.answers.push(answer);
        self
    }
    pub fn build(self) -> Packet {
        let packet = Packet {
            header: Header {
                id: self.header.id,
                qr: 1,
                opcode: self.header.opcode,
                aa: 0,
                tc: 0,
                rd: self.header.rd,
                ra: 0,
                z: 0,
                rcode: if self.header.opcode == 0 { 0 } else { 4 },
                qdcount: self.questions.len() as u16,
                ancount: self.answers.len() as u16,
                nscount: 0,
                arcount: 0,
            },
            questions: self.questions,
            answers: self.answers,
        };
        packet
    }
}

impl Packet {
    pub fn builder() -> PacketBuilder {
        PacketBuilder::default()
    }
}
