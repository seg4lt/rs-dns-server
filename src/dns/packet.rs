use tracing_subscriber::field::display::Messages;

use crate::common::{dns_reader::DnsReader, AsBytes, Parse};

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

pub trait Merge<T> {
    fn merge(&self) -> T;
}

impl Merge<Packet> for Vec<Packet> {
    fn merge(&self) -> Packet {
        let count = self.len();
        Packet::builder()
            .header(Header {
                id: rand::random(),
                qr: 1,
                opcode: 0,
                aa: 0,
                tc: 0,
                rd: 1,
                ra: 0,
                z: 0,
                rcode: 0,
                qdcount: count as u16,
                ancount: count as u16,
                nscount: 0,
                arcount: 0,
            })
            .questions(self.iter().flat_map(|p| p.questions.clone()).collect())
            .answers(self.iter().flat_map(|p| p.answers.clone()).collect())
            .build()
    }
}

impl Packet {
    pub fn split(&self) -> Vec<Self> {
        self.questions
            .iter()
            .map(|question| {
                Packet::builder()
                    .header(Header {
                        id: rand::random(),
                        qr: self.header.qr,
                        opcode: self.header.opcode,
                        aa: self.header.aa,
                        tc: self.header.tc,
                        rd: self.header.rd,
                        ra: self.header.ra,
                        z: self.header.z,
                        rcode: self.header.rcode,
                        qdcount: 1,
                        ancount: self.header.ancount,
                        nscount: self.header.nscount,
                        arcount: self.header.arcount,
                    })
                    .question(question.clone())
                    .build()
            })
            .collect()
    }
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

impl Parse for Packet {
    fn parse(dns_reader: &mut DnsReader) -> Self {
        let header = Header::parse(dns_reader);
        let questions: Vec<Question> = (0..header.qdcount)
            .map(|_| Question::parse(dns_reader))
            .collect();

        Packet::builder()
            .header(header)
            .questions(questions)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::io::{BufRead, BufReader, Cursor, Read};

    use crate::{
        common::{dns_reader::DnsReader, AsBytes, Parse},
        config::setup_log,
        dns::{
            answer::{Answer, RData},
            header::Header,
            label::Label,
            question::Question,
            RecordClass, RecordType,
        },
    };

    use super::Packet;

    #[test]
    fn test_packet_parse() {
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
        let mut reader = DnsReader::new(&packet_byte);
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
                0, 0, 128, 0, 0, 1, 0, 0, 0, 0, 0, 0, 12, 99, 111, 100, 101, 99, 114, 97, 102, 116,
                101, 114, 115, 2, 105, 111, 0, 0, 1, 0, 1, 12, 99, 111, 100, 101, 99, 114, 97, 102,
                116, 101, 114, 115, 2, 105, 111, 0, 0, 1, 0, 1, 0, 0, 0, 60, 0, 4, 8, 8, 8, 8
            ]
        )
    }

    #[test]
    fn test_parse_2() {
        let bytes = vec![
            249, 79, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 12, 99, 111, 100, 101, 99, 114, 97, 102, 116,
            101, 114, 115, 2, 105, 111, 0, 0, 1, 0, 1,
        ];
        // [8, 215,
        // 1, 0,

        // 0, 1, << qdcount
        // 0, 0, << ancount
        // 0, 0, << nscount
        //  0, 0, << arcount
        // 12, 99, 111, 100, 101, 99, 114, 97, 102, 116, 101, 114, 115, 2, 105, 111, 0,
        //  0, 1, 0, 1]
        let mut reader = DnsReader::new(&bytes);
        let packet = Packet::parse(&mut reader);

        assert_eq!(packet.header.id, 63823);
        assert_eq!(packet.header.qdcount, 1);
        assert_eq!(packet.questions.first().unwrap().name.0, "codecrafters.io");
    }

    #[test]
    fn test_parse_compression_packet() {
        let bytes = vec![
            198, 32, 1, 0, //
            0, 2, // << qdcount
            0, 0, // << ancound
            0, 0, 0, 0, //
            //
            3, // 3 length
            97, 98, 99, //
            //
            17, // 17 length
            108, 111, 110, 103, 97, 115, 115, 100, 111, 109, 97, 105, 110, 110, 97, 109, 101,
            //
            3, // 3 length
            99, 111, 109, //
            //
            0, // 0x00 for q end
            //
            0, 1, 0, 1, // These 4 bytes are type and class
            //
            3, 100, 101, 102, // 3 length
            192, 16, // pointer to existing label
            0, 1, 0, 1,
        ];
        let mut reader = DnsReader::new(&bytes);
        let packet = Packet::parse(&mut reader);
        eprintln!("PACKET: {:?}", packet);
        assert_eq!(packet.header.id, 50720);
        assert_eq!(packet.header.qdcount, 2);
        assert_eq!(packet.questions.len(), 2);
        assert_eq!(
            packet.questions.first().unwrap().name.0,
            "abc.longassdomainname.com"
        );
        assert_eq!(
            packet.questions.last().unwrap().name.0,
            "def.longassdomainname.com"
        );

        let res_packet = Packet::builder()
            .header(packet.header)
            .answers(
                packet
                    .questions
                    .iter()
                    .map(|q| Answer {
                        name: q.name.clone(),
                        typez: RecordType::A,
                        class: RecordClass::IN,
                        ttl: 60,
                        rdata: RData("8.8.8.8".to_string()),
                    })
                    .collect(),
            )
            .questions(packet.questions)
            .build();
        assert_eq!(
            res_packet.as_bytes(),
            vec![
                198, 32, 129, 0, 0, 2, 0, 0, 0, 0, 0, 0, // Header
                //
                3, 97, 98, 99, 17, 108, 111, 110, 103, 97, 115, 115, 100, 111, 109, 97, 105, 110,
                110, 97, 109, 101, 3, 99, 111, 109, 0, 0, 1, 0, 1, 3, 100, 101, 102, 17, 108, 111,
                110, 103, 97, 115, 115, 100, 111, 109, 97, 105, 110, 110, 97, 109, 101, 3, 99, 111,
                109, 0, 0, 1, 0, 1, 3, 97, 98, 99, 17, 108, 111, 110, 103, 97, 115, 115, 100, 111,
                109, 97, 105, 110, 110, 97, 109, 101, 3, 99, 111, 109, 0, 0, 1, 0, 1, 0, 0, 0, 60,
                0, 4, 8, 8, 8, 8, 3, 100, 101, 102, 17, 108, 111, 110, 103, 97, 115, 115, 100, 111,
                109, 97, 105, 110, 110, 97, 109, 101, 3, 99, 111, 109, 0, 0, 1, 0, 1, 0, 0, 0, 60,
                0, 4, 8, 8, 8, 8
            ]
        );
        assert_eq!(res_packet.as_bytes().len(), 156);
    }
}
