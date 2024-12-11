use std::{env, fs::File, io::BufReader};

pub fn open_file_cmd_line() -> BufReader<File> {
    let filepath = env::args().nth(1).unwrap();
    File::open(filepath).map(BufReader::new).unwrap()
}