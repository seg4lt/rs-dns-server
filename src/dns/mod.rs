#![allow(unused)]

use std::io::Read;

use anyhow::Context;

use crate::{
    bits,
    common::{AsBytes, DnsReader, Parse},
};
pub mod answer;
pub mod header;
pub mod label;
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
impl Parse for RecordType {
    fn parse(reader: &mut DnsReader) -> Self {
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

impl Parse for RecordClass {
    fn parse(reader: &mut DnsReader) -> Self {
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


