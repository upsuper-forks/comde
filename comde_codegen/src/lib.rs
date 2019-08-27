use std::hash::Hash;
use std::fmt::Debug;

use std::io::prelude::*;
use std::io::{Cursor, Write};

use byte_string::ByteString;
use phf_shared::PhfHash;
use delegate::delegate;

use comde::{Compress, Compressor};

#[cfg(feature = "xz")]
pub type XzMap<K, V> = Map<K, V, comde::xz::XzCompressor>;

#[cfg(feature = "deflate")]
pub type DeflateMap<K, V> = Map<K, V, comde::deflate::DeflateCompressor>;

#[cfg(feature = "snappy")]
pub type SnappyMap<K, V> = Map<K, V, comde::snappy::SnappyCompressor>;

#[cfg(feature = "zstandard")]
pub type ZstdMap<K, V> = Map<K, V, comde::zstd::ZstdCompressor>;

pub struct Map<K, V, C>
where
    V: Compress,
    C: Compressor<V>
{
    map: phf_codegen::Map<K>,
    compressor: C,
    #[doc(hidden)]
    __value: std::marker::PhantomData<V>,
}

impl<K: Hash + PhfHash + Eq + Debug, V, C> Map<K, V, C>
where
    V: Compress,
    C: Compressor<V>,
{
    pub fn new() -> Map<K, V, C> {
        Map {
            map: phf_codegen::Map::new(),
            compressor: C::new(),
            __value: std::marker::PhantomData::<V>,
        }
    }

    #[inline]
    pub fn entry(&mut self, key: K, value: V) -> &mut Map<K, V, C> {
        let bytes = self.compressor.compress(value).unwrap();
        self.map.entry(key, &format!("{:?}", ByteString::new(bytes)));
        self
    }

    #[inline]
    pub fn phf_path(&mut self, path: &str) -> &mut Map<K, V, C> {
        self.map.phf_path(path);
        self
    }

    delegate! {
        target self.map {
            pub fn build<W: Write>(&self, w: &mut W) -> std::io::Result<()>;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "snappy")]
    fn basic_snappy() {
        let mut map = SnappyMap::new();

        map.entry("boop", "this is a string string string string string string this is indeed a string string string".to_string());

        let mut out = Cursor::new(vec![]);
        map.build(&mut out).unwrap();
        println!("snappy: {}", String::from_utf8(out.into_inner()).unwrap());
    }

    #[test]
    #[cfg(feature = "xz")]
    fn basic_xz() {
        let mut map = XzMap::new();

        map.entry("boop", "this is a string string string string string string this is indeed a string string string".to_string());

        let mut out = Cursor::new(vec![]);
        map.build(&mut out).unwrap();
        println!("xz: {}", String::from_utf8(out.into_inner()).unwrap());
    }

    #[test]
    #[cfg(feature = "deflate")]
    fn basic_deflate() {
        let mut map = DeflateMap::new();

        map.entry("boop", "this is a string string string string string string this is indeed a string string string".to_string());

        let mut out = Cursor::new(vec![]);
        map.build(&mut out).unwrap();
        println!("deflate: {}", String::from_utf8(out.into_inner()).unwrap());
    }

    #[test]
    #[cfg(feature = "zstandard")]
    fn basic_zstd() {
        let mut map = ZstdMap::new();

        map.entry("boop", "this is a string string string string string string this is indeed a string string string".to_string());

        let mut out = Cursor::new(vec![]);
        map.build(&mut out).unwrap();
        println!("zstd: {}", String::from_utf8(out.into_inner()).unwrap());
    }
}