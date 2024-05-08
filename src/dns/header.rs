use crate::common::AsBytes;

/// DnsHeader represents the header of a DNS packet.
/// It occupies 12 bytes in the packet.
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
        bytes[2] = 0b1000_0000;
        bytes[3] = 0b0000_0000;
        bytes[4..6].copy_from_slice(&self.qdcount.to_be_bytes());
        bytes[6..8].copy_from_slice(&self.ancount.to_be_bytes());
        bytes[8..10].copy_from_slice(&self.nscount.to_be_bytes());
        bytes[10..12].copy_from_slice(&self.arcount.to_be_bytes());
        return bytes.to_vec();
    }
}

#[cfg(test)]
mod tests {
    use crate::common::AsBytes;

    use super::Header;

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
