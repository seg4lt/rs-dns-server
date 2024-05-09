use std::{default, io::Read};

use tracing::info;

use crate::{
    bits, bits16,
    common::{binary_macros::push_bits, AsBytes, DnsReader, Parse},
};

/// DnsHeader represents the header of a DNS packet.
/// It occupies 12 bytes in the packet.
#[derive(Default, Debug)]
pub struct Header {
    /// ID         16 bits    Packet Identifier
    pub id: u16,
    /// QR         1 bit      Query/Response flag (QR)
    pub qr: u8,
    /// OPCODE     4 bits     Operation Code (OPCODE)
    pub opcode: u8,
    /// AA         1 bit      Authoritative Answer (AA)
    pub aa: u8,
    /// TC         1 bit      Truncated (TC)
    pub tc: u8,
    /// RD         1 bit      Recursion Desired (RD)
    pub rd: u8,
    /// RA         1 bit      Recursion Available (RA)
    pub ra: u8,
    /// Z          3 bits     Reserved (Z)
    pub z: u8,
    /// RCODE      4 bits     Response Code (RCOODE)
    pub rcode: u8,
    /// QDCOUNT    16 bits    Question Count (QDCOUNT)
    pub qdcount: u16,
    /// ANCOUNT    16 bits    Answer Record Count (ANCOUNT)
    pub ancount: u16,
    /// NSCOUNT    16 bits    Authority Record Count (NSCOUNT)
    pub nscount: u16,
    /// ARCOUNT    16 bits    Additional Record Count (ARCOUNT)
    pub arcount: u16,
}

impl AsBytes for Header {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: [u8; 12] = [0; 12];
        bytes[0..2].copy_from_slice(&self.id.to_be_bytes());
        bytes[2..4].copy_from_slice(&self.flags_as_bytes());
        bytes[4..6].copy_from_slice(&self.qdcount.to_be_bytes());
        bytes[6..8].copy_from_slice(&self.ancount.to_be_bytes());
        bytes[8..10].copy_from_slice(&self.nscount.to_be_bytes());
        bytes[10..12].copy_from_slice(&self.arcount.to_be_bytes());
        return bytes.to_vec();
    }
}
impl Parse for Header {
    fn parse(reader: &mut DnsReader) -> Self {
        let mut header = Header::default()
            .read_id(reader)
            .read_flags(reader)
            .read_qd_count(reader)
            .read_an_count(reader)
            .read_ns_count(reader)
            .read_ar_count(reader);
        header
    }
}

impl Header {
    fn read_id(mut self, reader: &mut DnsReader) -> Self {
        let mut buf: [u8; 2] = [0; 2];
        reader
            .read_exact(&mut buf)
            .expect("unable to read dns header id");
        self.id = u16::from_be_bytes(buf);
        self
    }
    fn read_qd_count(mut self, reader: &mut DnsReader) -> Self {
        self.qdcount = Self::read_two_byte_number(reader);
        self
    }
    fn read_an_count(mut self, reader: &mut DnsReader) -> Self {
        self.ancount = Self::read_two_byte_number(reader);
        self
    }
    fn read_ns_count(mut self, reader: &mut DnsReader) -> Self {
        self.nscount = Self::read_two_byte_number(reader);
        self
    }
    fn read_ar_count(mut self, reader: &mut DnsReader) -> Self {
        self.arcount = Self::read_two_byte_number(reader);
        self
    }
    fn read_two_byte_number(reader: &mut DnsReader) -> u16 {
        let mut buf: [u8; 2] = [0; 2];
        reader
            .read_exact(&mut buf)
            .expect("unable to read two bytes of number");
        u16::from_be_bytes(buf)
    }
    fn read_flags(mut self, reader: &mut DnsReader) -> Self {
        let mut buf: [u8; 2] = [0; 2];
        reader
            .read_exact(&mut buf)
            .expect("unable to read header flags");
        let mut flags = u16::from_be_bytes(buf);

        self.qr = bits16!(@msb; flags, 1) as u8;
        flags = flags << 1;

        self.opcode = bits16!(@msb; flags, 4) as u8;
        flags = flags << 4;

        self.aa = bits16!(@msb; flags, 1) as u8;
        flags = flags << 1;

        self.tc = bits16!(@msb; flags, 1) as u8;
        flags = flags << 1;

        self.rd = bits16!(@msb; flags, 1) as u8;
        flags = flags << 1;

        self.ra = bits16!(@msb; flags, 1) as u8;
        flags = flags << 1;

        self.z = bits16!(@msb; flags, 3) as u8;
        flags = flags << 3;

        self.rcode = bits16!(@msb; flags, 4) as u8;
        flags = flags << 4;

        self
    }

    /// Create a bits representation for flags that we can send as paylaod
    /// This will return exactly 16 bit (2 bytes) value
    fn flags_as_bytes(&self) -> [u8; 2] {
        let mut buf: u16 = 0;

        let mut qr = self.qr;
        qr = qr << 7; // qr is only q bit, so hack is to discard first 7 bits
        let bit = bits!(@msb; qr, 1);
        push_bits(&mut buf, bit);

        let mut opcode = self.opcode;
        opcode = opcode << 4;
        for _ in 0..4 {
            let bit = bits!(@msb; opcode, 1);
            opcode = opcode << 1;
            push_bits(&mut buf, bit);
        }

        let mut aa = self.aa;
        aa = aa << 7;
        let bit = bits!(@msb; aa, 1);
        push_bits(&mut buf, bit);

        let mut tc = self.tc;
        tc = tc << 7;
        let bit = bits!(@msb; tc, 1);
        push_bits(&mut buf, bit);

        let mut rd = self.rd;
        rd = rd << 7;
        let bit = bits!(@msb; rd, 1);
        push_bits(&mut buf, bit);

        let mut ra = self.ra;
        ra = ra << 7;
        let bit = bits!(@msb; ra, 1);
        push_bits(&mut buf, bit);

        let mut z = self.z;
        z = z << 5;
        for _ in 0..3 {
            let bit = bits!(@msb; z, 1);
            z = z << 1;
            push_bits(&mut buf, bit);
        }

        let mut rcode = self.rcode;
        rcode = rcode << 4;
        for _ in 0..4 {
            let bit = bits!(@msb; rcode, 1);
            rcode = rcode << 1;
            push_bits(&mut buf, bit);
        }
        buf.to_be_bytes()
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::common::{AsBytes, DnsReader, Parse};

    use super::Header;

    #[test]
    fn test_parse() {
        let header = Header {
            id: 9999,
            qr: 1,
            opcode: 1,
            aa: 1,
            tc: 1,
            rd: 1,
            ra: 1,
            z: 1,
            rcode: 1,
            qdcount: 1,
            ancount: 1,
            nscount: 1,
            arcount: 1,
        };
        let byte = header.as_bytes();
        let mut reader = DnsReader::new(&byte[..]);

        let parsed = Header::parse(&mut reader);
        assert_eq!(header.id, parsed.id);
        assert_eq!(header.qr, parsed.qr);
        assert_eq!(header.opcode, parsed.opcode);
        assert_eq!(header.aa, parsed.aa);
        assert_eq!(header.tc, parsed.tc);
        assert_eq!(header.rd, parsed.rd);
        assert_eq!(header.ra, parsed.ra);
        assert_eq!(header.z, parsed.z);
        assert_eq!(header.rcode, parsed.rcode);
    }

    #[test]
    fn test_as_bytes() {
        let dns_header = Header {
            id: 1234,
            qr: 1,
            opcode: 0,
            aa: 0,
            tc: 0,
            rd: 0,
            ra: 0,
            z: 0,
            rcode: 0,
            qdcount: 0,
            ancount: 0,
            nscount: 0,
            arcount: 0,
        };
        let byte = dns_header.as_bytes();

        assert_eq!(
            byte,
            vec![
                // id
                0b100,
                0b11010010,
                // other bits
                0b1000_0000,
                0b0000_0000,
                // qdcount
                0b0,
                0b0,
                //ancount
                0b0,
                0b0,
                //nscount
                0b0,
                0b0,
                // arcount
                0b0,
                0b0
            ]
        );
    }
}
