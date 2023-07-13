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
    /// Calculates the size of this chunk including bytes required to encode the chunk.
    ///
    /// For a [`Chunk::Simple`] this is equal to the size of its data, plus 8 bytes.
    /// A [`Chunk::List`] is very similar, but requires an additional 4 bytes to store the type of the list.
    ///
    /// # Examples
    /// Simple chunk:
    /// ```rust
    /// use guitar_riff::prelude::*;
    /// let chunk = Chunk::Simple { identifier: ChunkIdentifier(*b"TEST"), data: Vec::new() };
    /// assert_eq!(8, chunk.outer_size());
    /// ```
    /// List chunk:
    /// ```rust
    /// use guitar_riff::prelude::*;
    /// let chunk = Chunk::List { data_type: ChunkDataType(*b"TEST"), sub_chunks: Vec::new() };
    /// assert_eq!(12, chunk.outer_size());
    /// ```
    pub fn outer_size(&self) -> usize {
        8 + self.inner_size()
    }

    /// Calculates the size of this chunk's data.
    ///
    /// For a [`Chunk::Simple`] this is equal to the size of its data and does not include the chunk identifier or size.
    /// This is very similar for a [`Chunk::List`], but an additional 4 bytes are added for the type of list.
    /// Also, the size of a [`Chunk::List`] is not cached. Instead, it is calculated on-demand by summing the outer size of all child chunks and their children.
    ///
    /// # Examples
    /// Simple chunk:
    /// ```rust
    /// use guitar_riff::prelude::*;
    ///
    /// let expected_size = 16;
    /// let chunk = Chunk::Simple { identifier: ChunkIdentifier(*b"TEST"), data: vec![0u8; expected_size] };
    ///
    /// assert_eq!(expected_size, chunk.inner_size());
    /// ```
    ///
    /// List chunk:
    /// ```rust
    /// use guitar_riff::prelude::*;
    ///
    /// let sub_chunk_size = 16;
    /// let sub_chunk = Chunk::Simple { identifier: ChunkIdentifier(*b"TEST"), data: vec![0u8; sub_chunk_size] };
    /// let expected_size = sub_chunk.outer_size() + 4;
    /// let chunk = Chunk::List { data_type: ChunkDataType(*b"TEST"), sub_chunks: vec![sub_chunk] };
    ///
    /// assert_eq!(expected_size, chunk.inner_size());
    /// ```
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
    pub(crate) fn read_from<R: Read>(reader: &mut R) -> std::io::Result<Self> {
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
    pub(crate) fn read_from<R: Read>(reader: &mut R) -> std::io::Result<Self> {
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
