use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn calc_max(input_file: String) -> u32 {
    let file = File::open(input_file);
    let mut cur_size: u32 = 0;
    let mut largest_size: u32 = 0;

    let file: File = file.unwrap();

    let reader = BufReader::new(file).lines();

    for line in reader {
        let line = line.unwrap();

        if line.trim().is_empty() {
            if cur_size > largest_size {
                largest_size = cur_size
            };

            cur_size = 0;
        } else {
            if let Ok(cal) = line.parse::<u32>() {
                cur_size += cal;
            }
        }
    }

    largest_size
}
