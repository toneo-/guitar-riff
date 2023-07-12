use std::{fmt::Display, io::Read};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChunkIdentifier(pub [u8; 4]);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChunkDataType(pub [u8; 4]);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileChunk {
    pub form_type: ChunkDataType,
    pub sub_chunks: Vec<Chunk>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Chunk {
    Simple {
        identifier: ChunkIdentifier,
        data: Vec<u8>,
    },
    List {
        data_type: ChunkDataType,
        sub_chunks: Vec<Chunk>,
    },
}

impl Chunk {
    pub fn outer_size(&self) -> usize {
        match self {
            Chunk::Simple {
                identifier: _,
                data,
            } => 8 + data.len(),
            Chunk::List {
                data_type: _,
                sub_chunks,
            } => 12 + sub_chunks.iter().map(|x| x.outer_size()).sum::<usize>(),
        }
    }

    pub fn inner_size(&self) -> usize {
        match self {
            Chunk::Simple {
                identifier: _,
                data,
            } => data.len(),
            Chunk::List {
                data_type: _,
                sub_chunks,
            } => 4 + sub_chunks.iter().map(|x| x.outer_size()).sum::<usize>(),
        }
    }
}

#[derive(Debug)]
pub enum RiffError {
    IoError(std::io::Error),
    FileNotRiff,
}

pub type RiffResult<T> = Result<T, RiffError>;

pub(crate) trait IoResultExt<T> {
    fn to_riff_result(self) -> RiffResult<T>;
}

impl<T> IoResultExt<T> for std::io::Result<T> {
    fn to_riff_result(self) -> RiffResult<T> {
        match self {
            Ok(inner) => Ok(inner),
            Err(io_error) => Err(RiffError::IoError(io_error)),
        }
    }
}

impl ChunkIdentifier {
    pub fn read_from<R: Read>(reader: &mut R) -> std::io::Result<Self> {
        let mut bytes = [0u8; 4];

        reader.read_exact(&mut bytes)?;

        return Ok(ChunkIdentifier(bytes));
    }
}

impl Display for ChunkIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&String::from_utf8_lossy(self.0.as_slice()))
    }
}

impl ChunkDataType {
    pub fn read_from<R: Read>(reader: &mut R) -> std::io::Result<Self> {
        let mut bytes = [0u8; 4];

        reader.read_exact(&mut bytes)?;

        return Ok(ChunkDataType(bytes));
    }
}

impl Display for ChunkDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&String::from_utf8_lossy(self.0.as_slice()))
    }
}
