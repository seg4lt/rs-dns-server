use thiserror::Error;

use self::dns_reader::DnsReader;

pub mod binary_macros;
pub mod dns_reader;

pub trait AsBytes {
    fn as_bytes(&self) -> Vec<u8>;
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unable to parse the data")]
    ParseError(#[from] std::io::Error),
    #[error("Unable to read {n} bytes, current position = {cur_pos}, buf_length = {buf_length}")]
    BufOverflow {
        cur_pos: usize,
        n: usize,
        buf_length: usize,
    },
}

pub trait Parse {
    fn parse(reader: &mut DnsReader) -> Self;
}
