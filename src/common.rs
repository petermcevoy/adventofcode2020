use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};

pub fn reader_as_string<R: BufRead>(reader: R) -> String {
    let input_vec: Vec<String> = reader.lines()
        .collect::<Result<_, _>>().unwrap();
    input_vec.join("\n")
}

pub fn file_as_string(file_path: &Path) -> String {
    let file = File::open(file_path).expect("Could not open file");
    let reader = BufReader::new(&file);
    reader_as_string(reader)
}
