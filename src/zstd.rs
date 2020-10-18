use bare_io::{Read, Write, Result, Seek, SeekFrom};

#[cfg(feature = "nightly")]
use bare_io::copy;

#[cfg(not(feature = "nightly"))]
use std::io::copy;

use zstd::stream::{read::Decoder, write::Encoder};

use crate::{com::ByteCount, Compressor, Decompress, Decompressor};

#[derive(Debug, Copy, Clone)]
pub struct ZstdDecompressor;

impl Decompressor for ZstdDecompressor {
    fn new() -> Self {
        ZstdDecompressor
    }

    #[cfg(feature = "nightly")]
    fn copy<R: Read, W: Write>(&self, source: R, mut dest: W) -> Result<u64> {
        let mut decoder = Decoder::new(source)?;
        copy::<_, _, 4096>(&mut decoder, &mut dest)
    }

    #[cfg(not(feature = "nightly"))]
    fn copy<R: Read, W: Write>(&self, source: R, mut dest: W) -> Result<u64> {
        let mut decoder = Decoder::new(source)?;
        copy(&mut decoder, &mut dest)
    }

    fn from_reader<R: Read, V: Decompress>(&self, reader: R) -> Result<V>
    where
        Self: Sized,
    {
        let decoder = Decoder::new(reader)?;
        V::from_reader(decoder)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ZstdCompressor;

impl Compressor for ZstdCompressor {
    fn new() -> Self {
        ZstdCompressor
    }

    #[cfg(feature = "nightly")]
    fn compress<W: Write + Seek, R: Read>(
        &self,
        writer: &mut W,
        reader: &mut R,
    ) -> Result<ByteCount> {
        let start = writer.seek(SeekFrom::Current(0))?;
        let mut encoder = Encoder::new(writer, 21)?;
        let read = copy::<_, _, 4096>(reader, &mut encoder)?;
        let writer = encoder.finish()?;
        let end = writer.seek(SeekFrom::Current(0))?;
        let write = end - start;
        Ok(ByteCount { read, write })
    }

    #[cfg(not(feature = "nightly"))]
    fn compress<W: Write + Seek, R: Read>(
        &self,
        writer: &mut W,
        reader: &mut R,
    ) -> Result<ByteCount> {
        let start = writer.seek(SeekFrom::Current(0))?;
        let mut encoder = Encoder::new(writer, 21)?;
        let read = copy(reader, &mut encoder)?;
        let writer = encoder.finish()?;
        let end = writer.seek(SeekFrom::Current(0))?;
        let write = end - start;
        Ok(ByteCount { read, write })
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn basic() {
//         let mut map = ZstdHashMap::<String, String>::new();
//         map.insert("foo".into(), "bar".into());
//         assert_eq!("bar".to_string(), map.get("foo").unwrap());
//         assert_ne!("bap".to_string(), map.get("foo").unwrap());
//     }
// }
