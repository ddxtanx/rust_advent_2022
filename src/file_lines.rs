use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_file_lines(file_path: String) -> impl Iterator<Item = String> {
    let file = File::open(file_path).unwrap();

    BufReader::new(file)
        .lines()
        .map(|line_res| line_res.as_ref().unwrap().to_owned())
}
