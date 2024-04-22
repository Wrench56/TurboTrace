#[derive(PartialEq)]
pub enum ChunkType {
    End,
    SizeChunk,
    KnownSizeChunk(u16),
    UnknownChunk,
}
