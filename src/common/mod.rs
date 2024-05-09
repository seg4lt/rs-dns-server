pub mod binary_macros;
pub trait AsBytes {
    fn as_bytes(&self) -> Vec<u8>;
}

pub trait Parse<R>
where
    R: std::io::Read,
{
    fn parse(reader: &mut R) -> Self;
}
