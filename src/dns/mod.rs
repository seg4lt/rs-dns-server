#![allow(unused)]

use crate::common::AsBytes;
pub mod answer;
pub mod header;
pub mod packet;
pub mod question;
pub mod server;

/// https://www.rfc-editor.org/rfc/rfc1035#section-3.2.2
#[derive(Debug)]
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

/// https://www.rfc-editor.org/rfc/rfc1035#section-3.2.4
#[derive(Debug)]
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

#[derive(Debug)]
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
