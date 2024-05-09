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
                record_class: RecordClass::IN,
                record_type: RecordType::A,
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
        eprintln!("{:?}", header.as_bytes())
    }

    #[test]
    fn test_parse_2() {
        let bytes = vec![
            249, 79, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 12, 99, 111, 100, 101, 99, 114, 97, 102, 116,
            101, 114, 115, 2, 105, 111, 0, 0, 1, 0, 1,
        ];
        let mut reader = Cursor::new(bytes);
        let header = Header::parse(&mut reader);
        let alp = vec![99, 111, 100, 101, 99, 114, 97, 102, 116, 101, 114, 115];
        let s = String::from_utf8(alp).unwrap();
        eprintln!("s: {}, Header {:?}", s, header);
    }
}
