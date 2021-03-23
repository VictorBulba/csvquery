#![warn(clippy::missing_inline_in_public_items)]
#![warn(clippy::missing_const_for_fn)]
#![warn(missing_docs)]

//! Query csv rows from large file

mod read_offset;
mod opts;
mod location;
mod index;
mod error;

use std::io;
use std::fs::File;
use std::path::Path;
use std::hash::Hash;
use std::borrow::Borrow;
use serde::de::DeserializeOwned;
use location::Location;
pub use opts::EngineOptions;
pub use error::QueryError;


/// # Engine for making key-value queries in csv file
///
/// ## Example
///
/// ```
/// let opts = EngineOptions::default();
/// let mut engine: Engine<String, YourType> = Engine::from_file_with_opts("file.csv", YourType::make_key, opts).unwrap();
/// let value = engine.get_from_file("key");
/// println!("{:?}", value);
/// ```
///
pub struct Engine<K, V> {
    index: index::Index<K>,
    file: File,
    delimiter: u8,
    _phantom: std::marker::PhantomData<V>,
}


impl<K: Hash + Eq, V: DeserializeOwned> Engine<K, V> {
    /// Open and index file from `path`.
    /// To index record `indexing_key_fn` should return some key
    #[inline]
    pub fn from_file_with_opts<P, F>(path: P, indexing_key_fn: F, opts: EngineOptions) -> io::Result<Self> 
        where
            P: AsRef<Path>,
            F: Fn(V) -> Option<K>
    {
        let index = index::make_index(&path, opts, indexing_key_fn)?;
        let file = File::open(path)?;
        let _phantom = std::marker::PhantomData::default();
        Ok(Self { index, file, delimiter: opts.delimiter, _phantom})
    }

    /// Reads value from file avoiding checking cache
    #[inline]
    pub fn get_from_file<Q>(&self, key: &Q) -> Result<V, QueryError>
        where 
            K: Borrow<Q>,
            Q: Hash + Eq,
    {
        let loc = self.index.get(key).ok_or(QueryError::NotPresented)?;
        let mut buf = vec![0; loc.len];
        read_offset::read_offset(&self.file, &mut buf, loc.offset).map_err(QueryError::IO)?;
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(self.delimiter)
            .from_reader(&buf as &[u8]);
        reader.deserialize().next()
            .map(|v| v.map_err(QueryError::CSV))
            .unwrap_or(Err(QueryError::NotPresented))
    }
}