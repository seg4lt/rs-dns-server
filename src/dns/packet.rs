use tracing_subscriber::field::display::Messages;

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
    fn parse(reader: &mut R) -> Self {
        let header = Header::parse(reader);
        // tracing::debug!(?header, "Value of header");
        let questions: Vec<Question> = (0..header.qdcount)
            .map(|_| Question::parse(reader))
            .collect();

        Packet::builder()
            .header(header)
            .questions(questions)
            .build()
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
    fn test_parse() {
        let mut header = Header::default();
        header.id = 99;
        let packet = Packet::builder()
            .header(header)
            .question(Question {
                name: Label("codecrafters.io".to_string()),
                class: RecordClass::IN,
                typez: RecordType::A,
            })
            .build();
        let packet_byte = packet.as_bytes();
        let mut reader = Cursor::new(&packet_byte);
        let header = Header::parse(&mut reader);
        assert_eq!(header.id, 99);
    }
    #[test]
    fn test_as_bytes() {
        let mut header = Header::default();
        header.qdcount = 1;
        let packet = Packet::builder()
            .header(header)
            .question(Question {
                name: Label("codecrafters.io".to_string()),
                class: RecordClass::IN,
                typez: RecordType::A,
            })
            .answer(Answer {
                name: Label("codecrafters.io".to_string()),
                typez: RecordType::A,
                class: RecordClass::IN,
                ttl: 60,
                rdata: RData("8.8.8.8".to_string()),
            })
            .build();
        let bytes = packet.as_bytes();
        assert_eq!(
            bytes,
            vec![
                0, 0, 128, 0, 0, 1, 0, 1, 0, 0, 0, 0, 12, 99, 111, 100, 101, 99, 114, 97, 102, 116,
                101, 114, 115, 2, 105, 111, 0, 0, 1, 0, 1, 12, 99, 111, 100, 101, 99, 114, 97, 102,
                116, 101, 114, 115, 2, 105, 111, 0, 0, 1, 0, 1, 0, 0, 0, 60, 0, 0, 0, 0, 0, 0, 0,
                4, 8, 8, 8, 8
            ]
        )
    }

    #[test]
    fn test_parse_2() {
        let bytes = vec![
            249, 79, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 12, 99, 111, 100, 101, 99, 114, 97, 102, 116,
            101, 114, 115, 2, 105, 111, 0, 0, 1, 0, 1,
        ];
        let mut reader = Cursor::new(bytes);
        let packet = Packet::parse(&mut reader);

        assert_eq!(packet.header.id, 63823);
        assert_eq!(packet.header.qdcount, 1);
        assert_eq!(packet.questions.first().unwrap().name.0, "codecrafters.io");
    }

    #[test]
    fn test_parse_compression_packet() {
        let bytes = vec![
            198, 32, 1, 0, 0, 2, 0, 0, 0, 0, 0, 0, 3, 97, 98, 99, 17, 108, 111, 110, 103, 97, 115,
            115, 100, 111, 109, 97, 105, 110, 110, 97, 109, 101, 3, 99, 111, 109, 0, 0, 1, 0, 1, 3,
            100, 101, 102, 192, 16, 0, 1, 0, 1,
        ];
        let mut reader = Cursor::new(bytes);
        let packet = Packet::parse(&mut reader);
        eprintln!("PACKET: {:?}", packet);
        assert_eq!(packet.header.id, 63823);
        assert_eq!(packet.header.qdcount, 1);
        assert_eq!(packet.questions.first().unwrap().name.0, "codecrafters.io");
    }
}
