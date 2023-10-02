#[derive(Debug)]
pub enum CompressionMethods {
    Null,
    Deflate,
    Unassigned(u8),
    Lzs,
    Reserved(u8),
}

impl From<u8> for CompressionMethods {
    fn from(value: u8) -> Self {
        match value {
            0 => CompressionMethods::Null,
            1 => CompressionMethods::Deflate,
            64 => CompressionMethods::Lzs,
            method @ (2..=63 | 65..=223) => CompressionMethods::Unassigned(method),
            method @ 224..=255 => CompressionMethods::Reserved(method),
        }
    }
}
