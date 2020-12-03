use std::fs;

fn base() -> Vec<i32> {
    let input = fs::read_to_string("src/day1/input").expect("Unable to read file");
    let expenses: Vec<i32> = input.split("\n").map(|x| x.parse::<i32>().unwrap()).collect();

    expenses
}

pub fn one() -> i32 {

    let expenses = base();

    for i in &expenses {
        for j in &expenses {
            if i + j == 2020 {
                return i*j;
            }
        }
    }

    return 0;
}


pub fn two() -> i32 {
    let expenses = base();

    for i in &expenses {
        for j in &expenses {
            for l in &expenses {
                if i + j + l == 2020 {
                    return i * j * l;
                }
            }
        }
    }
    return 0;
}