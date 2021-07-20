use std::fs;
use std::ops::Not;

pub fn one() -> usize {
    let input = fs::read_to_string("src/day10/input").expect("Unable to read file");
    let mut adapters: Vec<usize> = input.split("\n").map(|x| x.parse::<usize>().unwrap()).collect();

    adapters.sort();

    let mut index: usize = 0;
    let mut current_value: usize = 0;
    let mut difference_1: usize = 0;
    let mut difference_2: usize = 0;
    let mut difference_3: usize = 0;

    loop {
        if index > adapters.len() - 1 {
            break
        }
        let difference = adapters[index] - current_value;
        match difference {
            1 => difference_1 += 1,
            2 => difference_2 += 1,
            3 => difference_3 += 1,
            _ => {
                println!("invalid something");
                continue;
            }
        }
        current_value = adapters[index];

        index += 1;
    }


    return difference_1 * (difference_3+1);

}


pub fn two() -> u64 {
    let input = fs::read_to_string("src/day10/input").expect("Unable to read file");
    let mut adapters: Vec<u64> = input.split("\n").map(|x| x.parse::<u64>().unwrap()).collect();
    adapters.sort();
    adapters.insert(0, 0);
    adapters.push(adapters.last().unwrap() + 3);
    let mut consecutive_ones = vec![];
    let mut ones = vec![];

    // generate slices of consecutive 1-diff elements, for example:
    // input:  [0, 1, 4, 5, 6, 9]
    // result: [[0, 1], [4, 5, 6], [9]]
    for window in adapters.windows(2) {
        match window[1] - window[0] {
            1 => ones.push(window[0]),
            3 => {
                ones.push(window[0]);
                consecutive_ones.push(ones);
                ones = vec![];
            }
            _ => (),
        }
    }

    consecutive_ones.iter().map(
        |ones| match ones.len() {
            1 => 1,
            2 => 1,
            3 => 2,
            4 => 4,
            5 => 7,
            _ => panic!("Weird slice {:?}", ones),
        }
    ).product()

}
