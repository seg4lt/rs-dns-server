use crate::common::AsBytes;

use super::{Label, RecordClass, RecordType};

pub struct Question {
    pub name: Label,
    pub record_type: RecordType,
    pub record_class: RecordClass,
}
impl AsBytes for Question {
    fn as_bytes(&self) -> Vec<u8> {
        let mut buf = self.name.as_bytes();
        buf.extend(self.record_type.as_bytes());
        buf.extend(self.record_class.as_bytes());
        return buf;
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        common::AsBytes,
        dns::{question::Question, Label, RecordClass, RecordType},
    };

    #[test]
    fn test_dns_message() {
        let message = Question {
            name: Label {
                label_str: "google.com".to_string(),
            },
            record_type: RecordType::A,
            record_class: RecordClass::IN,
        };
        assert_eq!(
            message.as_bytes(),
            vec![
                0x06, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x03, 0x63, 0x6f, 0x6d, 0x00,
                // record_type
                0x00, 0x1, // record_class
                0x0, 0x2
            ]
        )
    }
}
