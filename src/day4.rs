use crate::file_lines;

pub fn num_fully_contained(assign_file: String) -> usize {
    let lines = file_lines::get_file_lines(&assign_file);

    range_iter(lines)
        .filter(|(l1, r1, l2, r2)| ((l1 <= l2 && r1 >= r2) || (l1 >= l2 && r1 <= r2)))
        .count()
}

fn range_iter<I>(lines_iter: I) -> impl Iterator<Item = (u32, u32, u32, u32)>
where
    I: Iterator<Item = String>,
{
    lines_iter.map(|s| sscanf::sscanf!(s, "{u32}-{u32},{u32}-{u32}").unwrap())
}

pub fn partially_contained(assign_file: String) -> usize {
    let lines = file_lines::get_file_lines(&assign_file);

    range_iter(lines)
        .filter(|(l1, r1, l2, r2)| (l1 <= l2 && r1 >= l2) || (l1 >= l2 && l1 <= r2))
        .count()
}
