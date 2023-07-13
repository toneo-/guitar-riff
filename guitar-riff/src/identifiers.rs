use crate::chunk::ChunkIdentifier;

/// Built-in [`ChunkIdentifier`] for the "INFO" list chunk, which contains metadata.
pub const INFO: ChunkIdentifier = ChunkIdentifier(*b"INFO");
/// Built-in [`ChunkIdentifier`] for the "LIST" chunk, which can contain sub-chunks.
pub const LIST: ChunkIdentifier = ChunkIdentifier(*b"LIST");
/// Built-in [`ChunkIdentifier`] for the "RIFF" chunk, which represents a RIFF file and can contain sub-chunks.
pub const RIFF: ChunkIdentifier = ChunkIdentifier(*b"RIFF");
