use std::{
    collections::BinaryHeap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_top_3(file_name: String) -> u32 {
    let file = File::open(file_name).unwrap();

    let reader = BufReader::new(file).lines();

    let mut heap = BinaryHeap::new();

    let mut cur_cal: u32 = 0;

    for line in reader {
        let line = line.unwrap();

        if line.trim().is_empty() {
            heap.push(cur_cal);
            cur_cal = 0;
        }

        if let Ok(cal) = line.parse::<u32>() {
            cur_cal += cal;
        }
    }

    let mut ret_vec: Vec<u32> = Vec::new();
    while !heap.is_empty() && ret_vec.len() < 3 {
        ret_vec.push(heap.pop().unwrap());
    }

    ret_vec.into_iter().reduce(|e, s| e + s).unwrap()
}
