use std::fs;
use std::ops::Not;

pub fn one() -> usize {
    let input = fs::read_to_string("src/day9/input").expect("Unable to read file");
    let numbers: Vec<usize> = input.split("\n").map(|x| x.parse::<usize>().unwrap()).collect();

    let mut index = 0;
    let mut sums: Vec<usize> = Vec::new();
    loop {
        if index <= 24 {
            index += 1;
            continue;
        }
        if index == numbers.len(){
            break;
        }
        sums = Vec::new();
        let number = numbers[index];
        for i in index-25..index {
            let first = numbers[i];
            for j in index-25..index {
                let second = numbers[j];
                sums.push(first+second);
            }
        }
        if sums.contains(&number).not() {
            return number
        }
        index += 1;
    }
    return 0;

}


pub fn two() -> usize {
    let input = fs::read_to_string("src/day9/input").expect("Unable to read file");
    let numbers: Vec<usize> = input.split("\n").map(|x| x.parse::<usize>().unwrap()).collect();

    let to_find = 1721308972;

    let mut index = 0;
    let mut sum: usize = 0;
    let mut sums: Vec<usize> = Vec::new();
    let mut subarray: Vec<usize> = Vec::new();
    loop {
        if index == numbers.len() {
            break;
        }
        sums = Vec::new();
        if numbers[index] == to_find {
            index += 1;
            continue;
        }
        sum = numbers[index];
        for j in index+1..numbers.len() {
            if sum == to_find {
                for x in index..j {
                    subarray.push(numbers[x]);
                }
                let min = subarray.iter().min().unwrap();
                let max = subarray.iter().max().unwrap();

                return min + max;
            }
            if sum > 1721308972 {
                break;
            }
            sum += numbers[j]
        }
        index += 1;
    }
    return 0;

}
