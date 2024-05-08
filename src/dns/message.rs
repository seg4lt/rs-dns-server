use crate::common::AsBytes;

pub struct DnsQuestion {
    pub name: String,
    pub record_type: DnsRecordType,
    pub record_class: DnsRecordClass,
}
impl AsBytes for DnsQuestion {
    fn as_bytes(&self) -> Vec<u8> {
        let mut labels = self
            .name
            .split(".")
            .map(|label| {
                let len = label.len();
                let mut vec = Vec::with_capacity(len + 1);
                vec.push(len as u8);
                label.chars().map(|c| c as u8).for_each(|b| vec.push(b));
                return vec;
            })
            .flat_map(|f| f.into_iter())
            .collect::<Vec<u8>>();
        labels.push(0x00);
        labels.extend(self.record_type.as_bytes().iter());
        labels.extend(self.record_class.as_bytes().iter());
        return labels;
    }
}

pub enum DnsRecordType {
    A,
}
impl AsBytes for DnsRecordType {
    fn as_bytes(&self) -> Vec<u8> {
        use DnsRecordType::*;
        let value: u16 = match self {
            A => 1,
        };
        return value.to_be_bytes().to_vec();
    }
}
pub enum DnsRecordClass {
    IN,
}
impl AsBytes for DnsRecordClass {
    fn as_bytes(&self) -> Vec<u8> {
        use DnsRecordClass::*;
        let value: u16 = match self {
            IN => 1,
        };
        return value.to_be_bytes().to_vec();
    }
}

#[cfg(test)]
mod tests {
    use crate::common::AsBytes;

    use super::{DnsQuestion, DnsRecordClass, DnsRecordType};

    #[test]
    fn test_dns_message() {
        let message = DnsQuestion {
            name: "google.com".to_string(),
            record_type: DnsRecordType::A,
            record_class: DnsRecordClass::IN,
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
