use crate::common::AsBytes;

use super::{Label, RecordClass, RecordType};

#[derive(Debug)]
pub struct Answer {
    pub name: Label,
    pub answer_type: RecordType,
    pub class: RecordClass,
    pub ttl: u32,
    pub rdata: RData,
}

impl AsBytes for Answer {
    fn as_bytes(&self) -> Vec<u8> {
        let mut buf = self.name.as_bytes();
        buf.extend(self.answer_type.as_bytes());
        buf.extend(self.class.as_bytes());
        buf.extend(self.ttl.to_be_bytes());
        let rdata = self.rdata.as_bytes();
        buf.extend(rdata.len().to_be_bytes());
        buf.extend(rdata);
        buf
    }
}

#[derive(Debug)]
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
    use crate::{
        common::AsBytes,
        dns::{Label, RecordClass, RecordType},
    };

    use super::Answer;

    #[test]
    fn test_dns_answer() {
        let answer = Answer {
            name: Label {
                label: "google.com".to_string(),
            },
            answer_type: RecordType::A,
            class: RecordClass::IN,
            ttl: 60,
            rdata: super::RData("8.8.8.8".to_string()),
        };
        assert_eq!(
            answer.as_bytes(),
            vec![
                6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0, 0, 1, 0, 1, 0, 0, 0, 60, 0, 0,
                0, 0, 0, 0, 0, 4, 8, 8, 8, 8
            ]
        );
    }
}
