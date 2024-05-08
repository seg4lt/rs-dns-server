use crate::common::AsBytes;

use super::answer::Answer;
use super::header::Header;
use super::question::Question;

pub mod packet_builder;

pub struct Packet {
    pub header: Header,
    pub questions: Vec<Question>,
    pub answers: Vec<Answer>,
}

impl AsBytes for Packet {
    fn as_bytes(&self) -> Vec<u8> {
        // in most cases DNS packet shouldn't be more than 512 bytes
        let mut buf: Vec<u8> = Vec::with_capacity(512);
        buf.extend(self.header.as_bytes());
        self.questions.iter().for_each(|q| buf.extend(q.as_bytes()));
        self.answers.iter().for_each(|a| buf.extend(a.as_bytes()));
        buf
    }
}
