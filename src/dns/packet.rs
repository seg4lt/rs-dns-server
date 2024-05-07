use crate::common::AsBytes;

use super::header;

pub struct DnsPacket {
    pub header: header::DnsHeader,
}
impl AsBytes for DnsPacket {
    fn as_bytes(&self) -> Vec<u8> {
        self.header.as_bytes()
    }
}
