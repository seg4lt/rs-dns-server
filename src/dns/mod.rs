#![allow(unused)]

use std::io::Read;

use anyhow::Context;

use crate::common::{AsBytes, Parse};
pub mod answer;
pub mod header;
pub mod packet;
pub mod question;
pub mod server;

/// https://www.rfc-editor.org/rfc/rfc1035#section-3.2.2
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RecordType {
    A,
    NS,
    MD,
    MF,
    CNAME,
    SOA,
    MB,
    MG,
    MR,
    NULL,
    WKS,
    PTR,
    HINFO,
    MINFO,
    MX,
    TXT,
}
impl AsBytes for RecordType {
    fn as_bytes(&self) -> Vec<u8> {
        use RecordType::*;
        let value: u16 = match self {
            A => 1,
            NS => 2,
            MD => 3,
            MF => 4,
            CNAME => 5,
            SOA => 6,
            MB => 7,
            MG => 8,
            MR => 9,
            NULL => 10,
            WKS => 11,
            PTR => 12,
            HINFO => 13,
            MINFO => 14,
            MX => 15,
            TXT => 16,
        };
        return value.to_be_bytes().to_vec();
    }
}
impl<R> Parse<R> for RecordType
where
    R: Read,
{
    fn parse(reader: &mut R) -> Self {
        use RecordType::*;
        let mut buf: [u8; 2] = [0; 2];
        reader
            .read_exact(&mut buf)
            .expect("unable to parse record type");
        match u16::from_be_bytes(buf) {
            1 => A,
            n => unimplemented!("{n} record type are not implemented yet!!!"),
        }
    }
}

/// https://www.rfc-editor.org/rfc/rfc1035#section-3.2.4
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RecordClass {
    IN,
    CS,
    CH,
    HS,
}
impl AsBytes for RecordClass {
    fn as_bytes(&self) -> Vec<u8> {
        use RecordClass::*;
        let value: u16 = match self {
            IN => 1,
            CS => 2,
            CH => 3,
            HS => 4,
        };
        return value.to_be_bytes().to_vec();
    }
}

impl<R> Parse<R> for RecordClass
where
    R: Read,
{
    fn parse(reader: &mut R) -> Self {
        use RecordClass::*;
        let mut buf: [u8; 2] = [0; 2];
        reader
            .read_exact(&mut buf)
            .expect("unable to parse record class");
        match u16::from_be_bytes(buf) {
            1 => IN,
            n => unimplemented!("{n} record class are not implemented yet!!!"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Label(pub String);

impl AsBytes for Label {
    fn as_bytes(&self) -> Vec<u8> {
        let mut labels = self
            .0
            .split(".")
            .map(|label| {
                let len = label.len();
                let mut vec = Vec::with_capacity(len + 1);
                vec.push(len as u8);
                label.chars().map(|c| c as u8).for_each(|b| vec.push(b));
                vec
            })
            .flat_map(|f| f.into_iter())
            .collect::<Vec<u8>>();
        labels.push(0x00);
        labels
    }
}
impl<R> Parse<R> for Label
where
    R: Read,
{
    fn parse(reader: &mut R) -> Self {
        let mut label_parts = vec![];
        loop {
            let mut length: [u8; 1] = [0; 1];
            reader
                .read_exact(&mut length)
                .expect("unable to read length for a label");
            let length = length[0] as usize;
            if length == 0x00 {
                break;
            }
            let mut content = vec![0u8; length];
            reader
                .read_exact(&mut content)
                .with_context(|| format!("unable to read content with size {}", length))
                .unwrap();
            let content = String::from_utf8(content).expect("unable to prase content to string");
            label_parts.push(content);
        }
        Self(label_parts.join("."))
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::common::{AsBytes, Parse};

    use super::Label;

    #[test]
    fn test_as_bytes() {
        let label = Label("example.com".to_string());
        assert_eq!(
            label.as_bytes(),
            vec![7, 101, 120, 97, 109, 112, 108, 101, 3, 99, 111, 109, 0]
        )
    }
    #[test]
    fn test_parse() {
        let label = Label("example.com".to_string()).as_bytes();
        let mut reader = Cursor::new(label);
        assert_eq!("example.com", Label::parse(&mut reader).0)
    }
}
