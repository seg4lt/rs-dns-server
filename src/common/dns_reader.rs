use anyhow::bail;

use crate::common::ParseError;

/// Crude implementation of Reader, probably can use Cursor but wanted to code this myself :D
#[derive(Debug)]
pub struct DnsReader<'a> {
    pub buf: &'a [u8],
    pub cur_pos: usize,
}

impl<'a> DnsReader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self {
            buf,
            cur_pos: 0,
        }
    }

    pub fn read_exact(&mut self, target_buf: &mut [u8]) -> anyhow::Result<()> {
        self.peek_exact(target_buf)?;
        self.cur_pos += target_buf.len();
        Ok(())
    }

    pub fn peek_exact(&mut self, target_buf: &mut [u8]) -> anyhow::Result<()> {
        let len = target_buf.len();
        let upto = self.cur_pos + len;
        let buf_len = self.buf.len();
        if upto > buf_len {
            bail!(ParseError::BufOverflow {
                buf_length: buf_len,
                cur_pos: self.cur_pos,
                n: len
            })
        }
        let source_buf = &self.buf[self.cur_pos..upto];
        target_buf.copy_from_slice(source_buf);
        Ok(())
    }
}
