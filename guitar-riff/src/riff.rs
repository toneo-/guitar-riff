use std::{fs::File, io::Read, path::PathBuf};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::{chunk::*, identifiers};

pub struct RiffFile {
    file: File,
    pub form_type: ChunkDataType,
    pub chunks_size: u32,
}

pub struct RiffStream<R: Read> {
    stream: R,
    pub form_type: ChunkDataType,
    pub chunks_size: u32,
}

pub struct ChunkIterator<'a, R: Read> {
    reader: &'a mut R,
}

fn read_chunk<R: Read>(reader: &mut R) -> std::io::Result<Chunk> {
    let chunk_id = ChunkIdentifier::read_from(reader)?;
    let chunks_size = reader.read_u32::<LittleEndian>()?;

    if chunk_id == identifiers::LIST {
        let data_type = ChunkDataType::read_from(reader)?;
        let mut data = vec![0u8; (chunks_size - 4) as usize];
        reader.read_exact(&mut data)?;

        let new_reader = &mut data.as_slice();
        let chunks = ChunkIterator::new(new_reader).collect::<Vec<Chunk>>();

        return Ok(Chunk::List {
            data_type,
            sub_chunks: chunks,
        });
    }

    let mut data = vec![0u8; chunks_size as usize];
    reader.read_exact(&mut data)?;

    Ok(Chunk::Simple {
        identifier: chunk_id,
        data,
    })
}

impl<'a, R: Read> ChunkIterator<'a, R> {
    fn new(reader: &'a mut R) -> Self {
        ChunkIterator { reader }
    }
}

impl<'a, R: Read> Iterator for ChunkIterator<'a, R> {
    type Item = Chunk;

    fn next(&mut self) -> Option<Self::Item> {
        match read_chunk(self.reader) {
            Ok(chunk) => Some(chunk),
            Err(_) => None,
        }
    }
}

impl RiffFile {
    /// Open a [`RiffFile`] from a file on disk.
    pub fn open<'a, P: Into<PathBuf>>(path: P) -> RiffResult<RiffFile> {
        let path: PathBuf = path.into();
        let mut file = File::open(path).to_riff_result()?;
        let (form_type, chunks_size) = read_riff_header(&mut file)?;

        Ok(Self {
            file,
            form_type,
            chunks_size,
        })
    }

    pub fn chunks<'a>(&'a mut self) -> ChunkIterator<'a, File> {
        ChunkIterator::new(&mut self.file)
    }
}

impl<R: Read> RiffStream<R> {
    /// Open a [`RiffStream`] from an in-memory stream (i.e. implementing [`Read`]).
    pub fn open(mut stream: R) -> RiffResult<Self> {
        let (form_type, chunks_size) = read_riff_header(&mut stream)?;

        Ok(Self {
            form_type,
            stream,
            chunks_size,
        })
    }

    /// Takes the stream out of this [`RiffStream`], consuming it in the process.
    pub fn take_stream(self) -> R {
        self.stream
    }

    pub fn chunks<'a>(&'a mut self) -> ChunkIterator<'a, R> {
        ChunkIterator::new(&mut self.stream)
    }
}

fn read_riff_header<R: Read>(mut stream: &mut R) -> RiffResult<(ChunkDataType, u32)> {
    let riff_id = ChunkIdentifier::read_from(&mut stream).to_riff_result()?;

    if riff_id != crate::identifiers::RIFF {
        return Err(RiffError::FileNotRiff);
    }

    let chunks_size = stream.read_u32::<LittleEndian>().to_riff_result()?;

    let form_type = ChunkDataType::read_from(&mut stream).to_riff_result()?;

    Ok((form_type, chunks_size))
}

#[cfg(test)]
mod test {
    use byteorder::*;
    use std::io::Write;

    use crate::identifiers;

    use super::*;

    fn write_example_file(
        identifier: &ChunkIdentifier,
        size: u32,
        form_type: &ChunkDataType,
    ) -> Vec<u8> {
        let mut stream = Vec::<u8>::new();

        stream.write(&identifier.0).unwrap();
        stream.write_u32::<LittleEndian>(size).unwrap();
        stream.write(&form_type.0).unwrap();
        stream
    }

    #[test]
    fn should_correctly_read_riff_header() -> RiffResult<()> {
        // Arrange
        let provided_identifier = identifiers::RIFF;
        let expected_size = 1234u32;
        let expected_form_type = ChunkDataType(*b"INFO");
        let stream = write_example_file(&provided_identifier, expected_size, &expected_form_type);

        // Act
        let (actual_form_type, actual_size) = read_riff_header(&mut stream.as_slice())?;

        // Assert
        assert_eq!(actual_form_type, expected_form_type);
        assert_eq!(actual_size, expected_size);

        Ok(())
    }

    #[test]
    fn should_error_if_identifier_is_not_riff() {
        // Arrange
        let provided_identifier = ChunkIdentifier(*b"OOPS");
        let provided_size = 1234u32;
        let provided_form_type = ChunkDataType(*b"INFO");
        let stream = write_example_file(&provided_identifier, provided_size, &provided_form_type);

        // Act
        let result = read_riff_header(&mut stream.as_slice());
        assert!(matches!(result, Err(RiffError::FileNotRiff)));
    }
}
