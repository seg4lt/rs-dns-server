use crate::common::AsBytes;

use super::{label::Label, RecordClass, RecordType};

#[derive(Debug, Clone)]
pub struct Answer {
    pub name: Label,
    pub typez: RecordType,
    pub class: RecordClass,
    pub ttl: u32,
    pub rdata: RData,
}

impl AsBytes for Answer {
    fn as_bytes(&self) -> Vec<u8> {
        let mut buf = self.name.as_bytes();
        buf.extend(self.typez.as_bytes());
        buf.extend(self.class.as_bytes());
        buf.extend(self.ttl.to_be_bytes());
        let rdata = self.rdata.as_bytes();
        let len = rdata.len() as u16;
        buf.extend(len.to_be_bytes());
        buf.extend(rdata);
        buf
    }
}

#[derive(Debug, Clone)]
pub struct RData(pub String);
impl AsBytes for RData {
    fn as_bytes(&self) -> Vec<u8> {
        self.0
            .split(".")
            .map(|s| s.parse::<u8>().unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        common::AsBytes,
        dns::{label::Label, RecordClass, RecordType},
    };

    use super::Answer;

    #[test]
    fn test_dns_answer() {
        let answer = Answer {
            name: Label("google.com".to_string()),
            typez: RecordType::A,
            class: RecordClass::IN,
            ttl: 60,
            rdata: super::RData("8.8.8.8".to_string()),
        };
        assert_eq!(
            answer.as_bytes(),
            vec![
                6, // length
                103, 111, 111, 103, 108, 101, // google
                3,   // length
                99, 111, 109, // com
                0,   // label end
                0, 1, 0, 1, // class and type
                0, 0, 0, 60, // ttl 4-byte
                0, 4, 8, 8, 8, 8
            ]
        );
    }
}
