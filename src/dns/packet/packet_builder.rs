use crate::dns::{answer::Answer, header::Header, question::Question};

use super::Packet;

#[derive(Default)]
pub struct PacketBuilder {
    questions: Vec<Question>,
    answers: Vec<Answer>,
}

impl PacketBuilder {
    pub fn add_question(mut self, question: Question) -> Self {
        self.questions.push(question);
        self
    }
    pub fn add_answer(mut self, answer: Answer) -> Self {
        self.answers.push(answer);
        self
    }
    pub fn build(self) -> Packet {
        let packet = Packet {
            header: Header {
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
