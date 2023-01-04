use std::collections::VecDeque;
use crate::file_lines::get_file_lines;
use crate::alphabet_nums::alphabet_char_to_int;

struct UniquenessBuffer {
    num_buf: [usize; 52],
    chars: VecDeque<char>,
    max_size: usize,
    cur_unique_chars: usize,
}

impl UniquenessBuffer {
    pub fn new(size: usize) -> UniquenessBuffer {
        UniquenessBuffer { max_size: size, num_buf: [0; 52], chars: VecDeque::new(), cur_unique_chars: 0 }
    }


    // TODO: If buffer has multiple of the same character then the cur_unique_chars gets
    // underloaded
    fn add_char(&mut self, ch: char){
        let idx = alphabet_char_to_int(ch).unwrap();
        if self.chars.len() >= self.max_size {
            let pop_ch = self.chars.pop_front().unwrap();
            
            let pop_idx = alphabet_char_to_int(pop_ch).unwrap();
            self.num_buf[pop_idx as usize] -= 1;
            if self.num_buf[pop_idx as usize] == 0 {
                self.cur_unique_chars -= 1;     
            }
            
        }

        
        if self.num_buf[idx as usize] == 0 { self.cur_unique_chars += 1 };
        self.num_buf[idx as usize] += 1;
        self.chars.push_back(ch);
    }

    fn max_num_unique_chars(&self) -> bool {
        // println!("{:?} {}", self.chars, self.cur_unique_chars);
        self.max_size == self.cur_unique_chars
    }
}

struct MessageStream<T>
    where T: Iterator<Item = char>
{
    iter: T,
    packet_size: usize,
    pos: usize
}

impl<T> MessageStream<T>
    where 
    T: Iterator<Item = char>
{
    pub fn new(iter: T, packet_size: usize) -> MessageStream<T> {
        MessageStream { iter, packet_size, pos: 0 }
    }

    fn find_start_of_buffer(mut self) -> usize {
        let mut uniq_buf = UniquenessBuffer::new(self.packet_size);
        let buf_count_iter = self.iter.enumerate().take_while(|(i, ch)| -> bool {
            let uniq_buf_copy = &mut uniq_buf;

            uniq_buf_copy.add_char(*ch);
            self.pos = *i;
            !uniq_buf_copy.max_num_unique_chars()
        });

        buf_count_iter.count() + 1 // idk why +1
    }
}


pub fn get_start_of_stream() -> usize {
    let mut file_iter = get_file_lines(&"./inputs/day6.txt".to_string());
    let file_str = file_iter.next().unwrap();

    let file_chars = file_str.chars();

    let mess_stream = MessageStream::new(file_chars, 4);

    mess_stream.find_start_of_buffer()
}

pub fn get_start_of_message() -> usize {
    let mut file_iter = get_file_lines(&"./inputs/day6.txt".to_string());
    let file_str = file_iter.next().unwrap();

    let file_chars = file_str.chars();

    let mess_stream = MessageStream::new(file_chars, 14);

    mess_stream.find_start_of_buffer()
}
