use std::fs;
use std::ops::Not;

pub fn one() -> usize {
    let input = fs::read_to_string("src/day11/input").expect("Unable to read file");
    let srows = input.split("\n");
    let mut rows: Vec<Vec<char>> = vec!();
    for x in srows {
        rows.push(x.chars().collect())
    }
    let mut row_index: usize = 0;
    let mut col_index: usize = 0;

    loop {
        if row_index > rows.len() - 1 {
            break;
        }
        col_index = 0;
        loop {
            if col_index > row.len() - 1 {
                break;
            }
        }
    }

    return 0;
}


pub fn two() -> usize {
    let input = fs::read_to_string("src/day11/input").expect("Unable to read file");
    return 0;
}
