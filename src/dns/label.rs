use anyhow::Context;

use crate::{
    bits,
    common::{dns_reader::DnsReader, AsBytes, Parse},
};

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
        // Label end is represented by 0x00
        labels.push(0x00);
        labels
    }
}
impl Parse for Label {
    fn parse(reader: &mut DnsReader) -> Self {
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

            match length {
                0x00 => break,
                // if msb is 11, then it s the pointer, 01 and 10 are reserved but we don't care
                n if bits!(@msb; length as u8, 2) & 0b11 > 1 => {
                    let mut one_more: [u8; 1] = [0; 1];
                    reader
                        .read_exact(&mut one_more)
                        .expect("should be able to read one more byte to get the pointer location");
                    let offset = (bits!(@lsb; length as u8, 6) + one_more[0]) as usize;
                    tracing::debug!(offset, "Reading from pointer");
                    let mut pointer_reader = DnsReader {
                        buf: reader.buf,
                        cur_pos: offset,
                    };
                    let label = Label::parse(&mut pointer_reader);
                    label_parts.push(label.0);
                    return Label(label_parts.join("."));
                }
                _ => {} // Not a special case, parse as normal
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

    use crate::common::{dns_reader::DnsReader, AsBytes, Parse};

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
        let mut reader = DnsReader::new(&label);
        assert_eq!("example.com", Label::parse(&mut reader).0)
    }

    #[test]
    fn test_num_from_be_byte() {
        let bytes: [u8; 2] = [11, 233];
        let num = u16::from_be_bytes(bytes);
        assert_eq!(num, 3049);

        let bytes: [u8; 2] = [175, 200];
        let num = u16::from_be_bytes(bytes);
        assert_eq!(num, 45000);
    }
}
