use itertools::Itertools;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};
use crate::alphabet_nums::{alphabet_char_to_int, alphabet_char_to_priority};

struct Sack {
    left_comp: [u32; 52],
    right_comp: [u32; 52],
    common_item: Option<char>,
}

#[derive(Debug)]
enum ParseSackError {
    LengthErr(usize),
}


impl TryFrom<&str> for Sack {
    type Error = ParseSackError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let strlen = value.len();

        if strlen % 2 != 0 {
            return Err(ParseSackError::LengthErr(strlen));
        }

        let s1 = &value[0..strlen / 2];
        let s2 = &value[strlen / 2..strlen];

        let mut v1 = [0; 52];
        let mut v2 = [0; 52];

        s1.trim().chars().for_each(|ch| -> () {
            let idx = alphabet_char_to_int(ch).unwrap() as usize;
            v1[idx] += 1;
        });

        let match_ch = s2.trim().chars().fold(None, |mat, ch| -> Option<char> {
            let idx = alphabet_char_to_int(ch).unwrap() as usize;
            v2[idx] += 1;

            if v1[idx] != 0 {
                Some(ch)
            } else {
                mat
            }
        });

        Ok(Sack {
            left_comp: v1,
            right_comp: v2,
            common_item: match_ch,
        })
    }
}

pub fn get_total_match_priority(sack_file: String) -> u32 {
    let file = File::open(sack_file).unwrap();

    let reader = BufReader::new(file).lines();

    let mut tot_priority = 0;
    for line in reader {
        let line_str = line.unwrap();
        let line = line_str.as_str();

        let sack: Sack = line.try_into().unwrap();

        let match_ch = sack.common_item;
        if let Some(ch) = match_ch {
            tot_priority += alphabet_char_to_priority(ch).unwrap();
        }
    }

    tot_priority
}

pub fn get_sum_of_badges(sack_file: String) -> u32 {
    let file = File::open(sack_file).unwrap();

    let reader = BufReader::new(file).lines();

    let mut badge_priorities = 0;

    let flines: Vec<Result<String, io::Error>> = reader.collect();

    for line_chunk in flines.into_iter().chunks(3).into_iter() {
        let unwrapped_chunks: Vec<String> = line_chunk.map(|res| res.unwrap()).collect();

        if let [line1, line2, line3] = &unwrapped_chunks[..] {
            let mut vec1 = [false; 52];
            let mut vec2 = [false; 52];

            line1.chars().for_each(|c| {
                let idx = alphabet_char_to_int(c).unwrap() as usize;
                vec1[idx] = true;
            });

            line2.chars().for_each(|c| {
                let idx = alphabet_char_to_int(c).unwrap() as usize;
                vec2[idx] = true;
            });

            let badge_ch = line3.chars().fold(' ', |acc, c| -> char {
                let idx = alphabet_char_to_int(c).unwrap() as usize;

                if vec1[idx] && vec2[idx] {
                    c
                } else {
                    acc
                }
            });

            badge_priorities += alphabet_char_to_priority(badge_ch).unwrap();
        }
    }

    badge_priorities
}
