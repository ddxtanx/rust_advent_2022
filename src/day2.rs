use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum ParseMoveError {
    LengthError(usize),
    AlphabetError,
}

impl TryFrom<&str> for Move {
    type Error = ParseMoveError;
    fn try_from(string: &str) -> Result<Move, ParseMoveError> {
        let strlen = string.len();

        if strlen != 1 {
            return Err(ParseMoveError::LengthError(strlen));
        }

        match string {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err(ParseMoveError::AlphabetError),
        }
    }
}

impl TryFrom<&str> for Outcome {
    type Error = ParseMoveError;

    fn try_from(string: &str) -> Result<Outcome, ParseMoveError> {
        let strlen = string.len();

        if strlen != 1 {
            return Err(ParseMoveError::LengthError(strlen));
        }

        match string {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(ParseMoveError::AlphabetError),
        }
    }
}
enum Outcome {
    Win,
    Draw,
    Lose,
}

fn wins_against(p1_move: &Move, p2_move: &Move) -> Outcome {
    match (p1_move, p2_move) {
        (Move::Rock, Move::Scissors) => Outcome::Win,
        (Move::Scissors, Move::Paper) => Outcome::Win,
        (Move::Paper, Move::Rock) => Outcome::Win,
        (val1, val2) if val1 == val2 => Outcome::Draw,
        _ => Outcome::Lose,
    }
}

fn move_score(p1_move: &Move) -> u32 {
    match p1_move {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

fn outcome_score(out: &Outcome) -> u32 {
    match out {
        Outcome::Lose => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    }
}

fn match_score(p1_move: Move, p2_move: Move) -> u32 {
    let out = wins_against(&p1_move, &p2_move);
    move_score(&p1_move) + outcome_score(&out)
}

pub fn score_matches(match_file_path: String) -> u32 {
    let file = File::open(match_file_path).unwrap();

    let reader = BufReader::new(file).lines();

    let mut score = 0;

    for line in reader {
        let line = line.unwrap();

        let splits: Vec<&str> = line.split(" ").collect();
        if splits.len() != 2 {
            continue;
        }

        let m2: Move = splits[0].try_into().unwrap();
        let m1: Move = splits[1].try_into().unwrap();

        score += match_score(m1, m2);
    }

    score
}

fn get_move_for_outcome(p1_move: &Move, out: &Outcome) -> Move {
    match out {
        Outcome::Draw => match p1_move {
            Move::Rock => Move::Rock,
            Move::Paper => Move::Paper,
            Move::Scissors => Move::Scissors,
        },
        Outcome::Win => match p1_move {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        },
        Outcome::Lose => match p1_move {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        },
    }
}

pub fn score_outcomes(outcome_file_path: String) -> u32 {
    let file = File::open(outcome_file_path).unwrap();

    let reader = BufReader::new(file).lines();

    let mut score = 0;

    for line in reader {
        let line = line.unwrap();

        let splits: Vec<&str> = line.split(" ").collect();

        if splits.len() != 2 {
            continue;
        }

        let opp_move: Move = splits[0].try_into().unwrap();
        let outcome: Outcome = splits[1].try_into().unwrap();

        let p_move = get_move_for_outcome(&opp_move, &outcome);

        score += move_score(&p_move) + outcome_score(&outcome);
    }

    score
}
