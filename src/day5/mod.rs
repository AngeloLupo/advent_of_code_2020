use std::fs;
use std::ops::Not;

pub fn one() -> usize {
    let input = fs::read_to_string("src/day5/input").expect("Unable to read file");
    let boarding_passes: Vec<&str> = input.split("\n").collect();

    let mut highest_id: usize = 0;

    for boarding_pass in boarding_passes {
        let id = calculate_id(boarding_pass);
        if id > highest_id {
            highest_id = id;
        }
    }

    return highest_id;
}

pub fn two() -> usize {
    let input = fs::read_to_string("src/day5/input").expect("Unable to read file");
    let boarding_passes: Vec<&str> = input.split("\n").collect();

    let mut ids: Vec<usize> = Vec::new();

    for boarding_pass in boarding_passes {
        let id = calculate_id(boarding_pass);
        let pos = ids.binary_search(&id).unwrap_or_else(|e| e);
        ids.insert(pos, id);
    }
    for i in ids[0] .. ids[ids.len()-1]{
        if ids.contains(&i).not() {
            return i;
        }
    }

    return 0;
}

fn calculate_id(sequence: &str) -> usize {

    let mut max_row: f64 = 127.0;
    let mut min_row: f64 = 0.0;
    let mut max_col: f64 = 7.0;
    let mut min_col: f64 = 0.0;

    for c in sequence.chars() {
        match c {
            'F' => max_row = ((max_row + min_row) / 2.0).floor(),
            'B' => min_row = ((max_row + min_row) / 2.0).ceil(),
            'L' => max_col = ((max_col + min_col) / 2.0).floor(),
            'R' => min_col = ((max_col + min_col) / 2.0).ceil(),
            _ => println!("Weird key: {}", c)
        }
        //println!("minr: {}, maxr: {}, minc {}, maxc {}", min_row, max_row, min_col, max_col);
    }
    return (max_row*8.0+max_col) as usize;

}