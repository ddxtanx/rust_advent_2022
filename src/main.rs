mod day1p1;
mod day1p2;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

mod tree;
mod alphabet_nums;
mod file_lines;
fn main() {
    println!(
        "Day 1 P1: {}",
        day1p1::calc_max("./inputs/day1.txt".to_string())
    );

    println!(
        "Day 1 P2: {}",
        day1p2::get_top_3("./inputs/day1.txt".to_string())
    );

    println!(
        "Day 2 P1: {}",
        day2::score_matches("./inputs/day2.txt".to_string())
    );

    println!(
        "Day 2 P2 : {}",
        day2::score_outcomes("./inputs/day2.txt".to_string())
    );

    println!(
        "Day 3 P1: {}",
        day3::get_total_match_priority("./inputs/day3.txt".to_string())
    );

    println!(
        "Day 3 P2: {}",
        day3::get_sum_of_badges("./inputs/day3.txt".to_string())
    );
    println!(
        "Day 4 P1: {}",
        day4::num_fully_contained("./inputs/day4.txt".to_string())
    );
    println!(
        "Day 4 P2: {}",
        day4::partially_contained("./inputs/day4.txt".to_string())
    );
    println!("Day 5 P1: {}", day5::get_leading_chars());
    println!("Day 5 P2: {}", day5::get_leading_chars_drag());
     
    println!("Day 6 P1: {}", day6::get_start_of_stream());
    println!("Day 6 P2: {}", day6::get_start_of_message());

    println!("Day 7 P1: {}", day7::get_largest_dirs());
    println!("Day 7 P2: {}", day7::get_delete_dir());
}
