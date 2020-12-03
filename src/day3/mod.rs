use std::fs;


pub fn one(column_move: usize, row_move: usize, ) -> usize {
    let input = fs::read_to_string("src/day3/input").expect("Unable to read file");
    let map: Vec<&str> = input.split("\n").collect();

    let mut row: usize = 0;
    let mut column: usize = 0;

    let mut trees: usize = 0;

    let max_col: usize = map[0].len() - 1;
    let max_row: usize = map.len() - 1;

    let mut position: char = 'a';

    while row < max_row {
        row += row_move;
        if row > max_row {
            break;
        }
        column += column_move;

        if column > max_col {
            column -= max_col;
            column -= 1;
        }
        position = map[row].chars().nth(column).unwrap();
        if position == '#' {
            trees += 1;
        }
    }

    return trees;
}

pub fn two() -> usize {
    return one(1, 1) * one(3, 1) * one(5, 1) * one(7, 1) * one(1, 2);
}
