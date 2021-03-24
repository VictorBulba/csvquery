use std::collections::HashMap;
use std::io::{self, BufReader};
use std::fs::File;
use std::path::Path;
use std::hash::Hash;
use serde::de::DeserializeOwned;
use super::{Location, EngineOptions};


pub(crate) type Index<K> = HashMap<K, Location>;
pub(crate) fn make_index<P, K, V, F>(path: P, opts: EngineOptions, indexing_key_fn: F) -> io::Result<Index<K>> 
    where 
        P: AsRef<Path>,
        K: Hash + Eq,
        V: DeserializeOwned,
        F: FnMut(V) -> Option<K>,
{
    let reader = make_csv_reader(&path, opts)?;
    let index = reader.into_records()
        .flatten()
        .scan(indexing_key_fn, |f, value| extract_key_location_pair(value, f))
        .collect();
    Ok(index)
}


type CSVBufFileReader = csv::Reader<BufReader<File>>;
fn make_csv_reader<P: AsRef<Path>>(path: P, opts: EngineOptions) -> io::Result<CSVBufFileReader> {
    let file = File::open(path)?;
    let reader = BufReader::with_capacity(opts.buf_capacity, file);
    let csv_reader = csv::ReaderBuilder::new()
        .delimiter(opts.delimiter)
        .has_headers(false)
        .from_reader(reader);
    Ok(csv_reader)
}


fn extract_key_location_pair<K, V, F>(rec: csv::StringRecord, indexing_key_fn: &mut F) -> Option<(K, Location)>
    where 
        K: Hash + Eq,
        V: DeserializeOwned,
        F: FnMut(V) -> Option<K>,
{
    const DELIMITER_LEN: usize = 1;
    let offset = rec.position()?.byte();
    let len = rec.as_slice().len() + (rec.len() - 1) * DELIMITER_LEN;
    let loc = Location { offset, len };
    let key = rec.deserialize(None).ok().and_then(|v| indexing_key_fn(v))?;
    Some((key, loc))
}