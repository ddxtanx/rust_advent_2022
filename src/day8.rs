use std::{cmp::max, ops::Add};
use crate::file_lines::get_file_lines;
use itertools::Itertools;
#[derive(Clone, Copy)]
struct MaxDirs<T: Copy> {
    up_max: Option<T>,
    down_max: Option<T>,
    right_max: Option<T>,
    left_max: Option<T>
}

impl<T: Copy> MaxDirs<T> {
    pub fn get_dir(&self, dir: &Dir) -> Option<T> {
        match dir {
            Dir::Up => self.up_max,
            Dir::Down => self.down_max,
            Dir::Left => self.left_max,
            Dir::Right => self.right_max
        }
    }

    pub fn get_dir_mut(&mut self, dir: &Dir) -> &mut Option<T> {
        match dir {
            Dir::Up => &mut self.up_max,
            Dir::Down => &mut self.down_max,
            Dir::Left => &mut self.left_max,
            Dir::Right => &mut self.right_max
        }
    }
}

struct RectangularArray<T: Copy> {
    buffer: Vec<Vec<T>>,
    width: usize,
    height: usize
}

impl<T: Copy> RectangularArray<T> {   
    pub fn new(height: usize, width: usize, i_val: T) -> Self {
        let mut buf = vec![];
        for _ in 0 .. height {
            buf.push(vec![i_val; width])
        }

        RectangularArray {
            buffer: buf,
            width,
            height
        }
    }
}

struct BoundsViewer<T: Ord + Copy + Add<usize, Output=T>> {
    arr: RectangularArray<T>
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right
}

impl<T: Ord + Copy + Add<usize, Output=T>> BoundsViewer<T> {
    fn get_initial_indices(&self, dir: &Dir) -> Vec<(usize, usize)> {
        let initial_idx = match dir {
            Dir::Up => self.arr.height - 1,
            Dir::Down | Dir::Right => 0,
            Dir::Left => self.arr.width - 1
        };

        let max = match dir {
            Dir::Up | Dir::Down => self.arr.width,
            Dir::Left | Dir::Right => self.arr.height
        };

        let init_range = 0 .. max;
        match dir {
            Dir::Up | Dir::Down => init_range.map(|i| (i, initial_idx)).collect(),
            Dir::Left | Dir:: Right => init_range.map(|i| (initial_idx, i)).collect()
        }
    }

    fn get_iter_indices(&self, dir: &Dir) -> Vec<(usize, usize)> {
        let (start_x, end_x) = match dir {
            Dir::Left => (self.arr.width - 2, 0),
            Dir::Right => (1, self.arr.width - 1),
            Dir::Up | Dir::Down => (0, self.arr.width - 1)
        };
       
        let (start_y, end_y) = match dir {
            Dir::Up => (self.arr.height - 2, 0),
            Dir::Down => (1, self.arr.height - 1),
            Dir::Left | Dir::Right => (0, self.arr.height - 1)
        };

        if *dir == Dir::Up || *dir == Dir::Left {
            println!("Oof");
        }

        let mut indices = vec![];
        
        match dir {
            Dir::Right => {
                for y in start_y ..= end_y {
                    for x in start_x ..= end_x {
                        indices.push((x,y));
                    }
                }
            },
            Dir::Left => {
                for y in start_y ..= end_y {
                    for x in (end_x ..= start_x).rev() {
                        indices.push((x,y));
                    }
                }
            },
            Dir::Down => {
                for x in start_x ..= end_x {
                    for y in start_y ..= end_y {
                        indices.push((x,y));
                    }
                } 
            },
            Dir::Up => {
                for x in start_x ..= end_x {
                    for y in (end_y ..= start_y).rev() {
                        indices.push((x,y));
                    }
                }
            }
        }

        indices
    }

    fn get_prev_index(coord: (usize, usize), dir: &Dir) -> (usize, usize) {
        let (x, y) = coord;
        match dir {
            Dir::Up => (x, y + 1),
            Dir::Down => (x, y - 1),
            Dir::Left => (x + 1, y),
            Dir::Right => (x - 1, y)
        }
    }

    fn compute_bounds_in_dir(&self, bounds: &mut RectangularArray<MaxDirs<T>>, dir: &Dir) {
        let init_idxs = self.get_initial_indices(dir);

        for (x, y) in init_idxs {
            let value = Some(self.arr.buffer[y][x]);
            let buf_val = bounds.buffer.get_mut(y).unwrap().get_mut(x).unwrap();
            // match dir {
            //     Dir::Up => buf_val.up_max = value,
            //     Dir::Down => buf_val.down_max = value,
            //     Dir::Left => buf_val.left_max = value,
            //     Dir::Right => buf_val.right_max = value
            // }
            *buf_val.get_dir_mut(dir) = value;
        }

        let loop_idxs = self.get_iter_indices(dir);

        for (x, y) in loop_idxs {
            let (prev_x, prev_y) = BoundsViewer::<T>::get_prev_index((x,y), dir);
            let prev_buf_val = bounds.buffer[prev_y][prev_x];
            let cur_buf_val = bounds.buffer.get_mut(y).unwrap().get_mut(x).unwrap();
            let prev_arr_val = self.arr.buffer[prev_y][prev_x];

            if prev_buf_val.get_dir(dir).is_none() {
                println!("oof");
            }
            let prev_max = prev_buf_val.get_dir(dir).unwrap();

            let new_max = max(prev_arr_val, prev_max);

            *cur_buf_val.get_dir_mut(dir) = Some(new_max);
        }
    }

    pub fn bounds(&self) -> RectangularArray<MaxDirs<T>> {
        let def_dir = MaxDirs {
            up_max: None,
            down_max: None,
            right_max: None,
            left_max: None
        };
        let mut bounds = RectangularArray::new(self.arr.height, self.arr.width, def_dir);

        for dir in [Dir::Left, Dir::Right, Dir::Up, Dir::Down] {
            self.compute_bounds_in_dir(&mut bounds, &dir);
        }        
        bounds
   } 
}

pub fn calculate_visible_trees() -> usize {
    let fname = "./inputs/day8.txt".to_string();
    let mut lines1 = get_file_lines(&fname).peekable();
    let line1 = lines1.peek().unwrap();
    let width = line1.len();
    let height = lines1.count();

    let lines2 = get_file_lines(&fname);

    let mut tree_table = RectangularArray::<usize>::new(height, width, 0);

    for (i, line) in lines2.enumerate() {
        for (j, int_char) in line.chars().enumerate() {
            let int_v = int_char.to_digit(10).unwrap() as usize;
            *tree_table.buffer.get_mut(i).unwrap().get_mut(j).unwrap() = int_v;
        }
    }

    let bounds_viewer = BoundsViewer {
        arr: tree_table
    };

    let max_buff = bounds_viewer.bounds();

    let mut count = 0;

    let dirs = vec![Dir::Up, Dir::Down, Dir::Left, Dir::Right];

    let mut vis_arr = vec![];
    for i in 0 .. height {
        let mut vis_row = vec![];
        for j in 0 .. width {
            let tree_val = bounds_viewer.arr.buffer[i][j];
            let max_val = max_buff.buffer[i][j];
            let is_viewable = dirs.iter().
                any(|dir| {
                    if max_val.get_dir(dir).is_none() { 
                        println!("oof");
                    }
                    max_val.get_dir(dir).unwrap() < tree_val
                })
            || i == 0 || i == height - 1 || j == 0 || j == width - 1;  // TODO: Make better
                                                                       // detection on the edges
            if is_viewable {
                count += 1;
                vis_row.push("@".to_string());
            } else {
                vis_row.push("O".to_string());
            }
        }
        vis_arr.push(vis_row);
    }

    let str: String = vis_arr.into_iter().
        map(|row: Vec<String>| row.into_iter().
            reduce(|acc, st| acc + &st).unwrap()
        ).
        reduce(|acc, st| acc + "\n" + &st).unwrap();
    
    let mut up_str = "".to_string();
    let mut down_str = "".to_string();
    let mut left_str = "".to_string();
    let mut right_str = "".to_string();

    for row in max_buff.buffer {
        for max_dir in row {
            for dir in dirs.iter() {
                let max_val = max_dir.get_dir(dir).unwrap().to_string();
                
                match dir {
                    Dir::Up => up_str = up_str + &max_val,
                    Dir::Down => down_str = down_str + &max_val,
                    Dir::Left => left_str = left_str + &max_val,
                    Dir::Right => right_str = right_str + &max_val
                }
            }

            up_str += ","; 
            down_str += ",";
            right_str += ",";
            left_str += ",";
        }

        up_str += "\n"; 
        down_str += "\n";
        right_str += "\n";
        left_str += "\n"; 
    }

    println!("Up str \n{up_str}");
    println!("Down str \n{down_str}");
    println!("Left str \n{left_str}");
    println!("Right str \n{right_str}");

    println!("Board \n{str}");
    count
}
