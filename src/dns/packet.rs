use crate::common::{AsBytes, Parse};

use super::answer::Answer;
use super::header::Header;
use super::question::Question;

pub mod packet_builder;

#[derive(Debug)]
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
impl<R> Parse<R> for Packet
where
    R: std::io::Read,
{
    fn parse(buf: &mut R) -> Self {
        let header = Header::parse(buf);
        let mut packet = Packet::builder().header(header).build();
        return packet;
    }
}

#[cfg(test)]
mod tests {
    use std::io::{BufRead, BufReader, Cursor, Read};

    use crate::{
        common::{AsBytes, Parse},
        dns::{
            answer::{Answer, RData},
            header::Header,
            question::Question,
            Label, RecordClass, RecordType,
        },
    };

    use super::Packet;

    #[test]
    fn test() {
        let mut header = Header::default();
        header.id = 99;
        let packet = Packet::builder()
            .header(header)
            .question(Question {
                name: Label {
                    label: "codecrafters.io".to_string(),
                },
                record_class: RecordClass::IN,
                record_type: RecordType::A,
            })
            .answer(Answer {
                name: Label {
                    label: "codecrafters.io".to_string(),
                },
                answer_type: RecordType::A,
                class: RecordClass::IN,
                ttl: 60,
                rdata: RData("8.8.8.8".to_string()),
            })
            .build();
        let packet_byte = packet.as_bytes();
        let mut reader = Cursor::new(&packet_byte);
        let header = Header::parse(&mut reader);
        assert_eq!(header.id, 99);
    }
}
