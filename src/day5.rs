use std::{collections::VecDeque, borrow::BorrowMut, iter::Peekable};
use itertools::Itertools;
use sscanf::sscanf;
use crate::file_lines;

struct Crates {
    pub crates: Vec<Crate>,
}

struct Crate {
    items: VecDeque<char>
}

#[derive(Debug)]
enum CrateErrors {
    SizeError(usize)
}

struct Move {
    from: usize,
    to: usize,
    num: usize,
}
impl Crate {
    fn get_top_n(&mut self, n: usize) -> Result<Vec<char>, CrateErrors> {
        let items = &mut self.items;
        
        if n > items.len() {
            return Err(CrateErrors::SizeError(n))
        }

        let mut ret_vec = Vec::new();

        for _ in 0 .. n {
            ret_vec.push(items.pop_front().unwrap());
            
        }
        Ok(ret_vec)
    }

    fn push_chars<'a, I>(&mut self, chars: I) 
    where
        I: Iterator<Item = &'a char>
        {
        for c in chars {
            self.items.push_front(*c);
        }
    }

    fn add_char(&mut self, ch: char) {
        self.items.push_back(ch);       
    }
}

enum MoveType {Stack, Drag}
impl Crates {
    fn perform_move(&mut self, movement: Move, move_type: MoveType) {
        let crates = &mut self.crates;

        if movement.from > crates.len() || movement.to > crates.len(){
            return;
        }

        let from_crate: &mut Crate = crates.get_mut(movement.from - 1).unwrap();
        let chars = from_crate.get_top_n(movement.num);
        let to_crate = crates.get_mut(movement.to - 1).unwrap();
        match move_type{
            MoveType::Stack => to_crate.push_chars(chars.unwrap().iter()),
            MoveType::Drag => to_crate.push_chars(chars.unwrap().iter().rev())
        }
    }

    pub fn perform_move_stack(&mut self, movement: Move) {
        self.perform_move(movement, MoveType::Stack);
    }

    pub fn perform_move_drag(&mut self, movement: Move) {
        self.perform_move(movement, MoveType::Drag);
    }

    pub fn get_top_chars(self) -> String{
        self.crates.iter().
            map(|cr| *cr.items.front().unwrap_or(&' ')).
            filter(|c| *c != ' ').
            collect()
    }

}

fn generate_crates<I>(mut lines: Peekable<I>) -> Crates
where
    I: Iterator<Item = String>
{
    let init_len = lines.peek().unwrap().len();
    let num_crates = (init_len + 1) / 4;
    let mut crates: Vec<Crate> = Vec::new();
    for _ in 0 .. num_crates {
        crates.push(Crate{
            items: VecDeque::new()
        });
    }
    for line in lines{
        line.
            chars().
            chunks(4).
            into_iter().
            map(|ch| ch.collect::<String>()).
            enumerate().
            map(|(i, chunk)| (i, sscanf!(chunk.trim(), "[{char}]"))).
            filter(|(_, res)| res.is_ok()).
            for_each(|(i, ch)| {
                crates.get_mut(i).as_mut().unwrap().add_char(ch.unwrap())
            })
    }

    Crates{
        crates
    }
}

pub fn get_leading_chars() -> String {
    let mut lines = file_lines::get_file_lines("./inputs/day5.txt".to_string()).peekable();

    let mut init_crate_iter = lines.by_ref().take_while(|line| !line.is_empty()).peekable();
    
    let mut crates_obj = generate_crates(init_crate_iter);

    for move_line in lines {
        let (num, from, to) = sscanf!(move_line, "move {usize} from {usize} to {usize}").unwrap();
        
        crates_obj.perform_move_stack(Move{
            from,
            to,
            num
        });
    }

    crates_obj.get_top_chars()
}

pub fn get_leading_chars_drag() -> String {
    let mut lines = file_lines::get_file_lines("./inputs/day5.txt".to_string()).peekable();

    let init_crate_iter = lines.by_ref().take_while(|line| !line.is_empty()).peekable();
    
    let mut crates_obj = generate_crates(init_crate_iter);

    for move_line in lines {
        let (num, from, to) = sscanf!(move_line, "move {usize} from {usize} to {usize}").unwrap();
        
        crates_obj.perform_move_drag(Move{
            from,
            to,
            num
        });
    }

    crates_obj.get_top_chars()
}
