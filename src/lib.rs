use parquet::file::{reader::{FileReader, SerializedFileReader}, serialized_reader::SliceableCursor};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main(contents: Vec<u8>) {
    let cursor = SliceableCursor::new(contents);
    let reader = SerializedFileReader::new(cursor).unwrap();
    let mut iter = reader.get_row_iter(None).unwrap();
    while let Some(record) = iter.next() {
        println!("{}", record);
    }
}
