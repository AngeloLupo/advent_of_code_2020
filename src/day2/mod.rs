use std::fs;

// fn read_file() -> Vec<&'static str> {
//     let input = fs::read_to_string("src/day2/input").expect("Unable to read file");
//     let strings: Vec<&'static str> = input.split("\n").collect();
//
//     strings
// }
//
// fn split_data(strings: Vec<&str>) -> Vec<Vec<&str>> {
//     let passwords: Vec<Vec<&str>> = (strings.iter().map(|x: &&str| {x.split(": ").collect()})).collect();
//     println!("{} -- {}", passwords[0][0], passwords[0][1]);
//     return passwords;
// }

fn is_valid_one(password: &str, policy: &str) -> bool {
    let policy_str: Vec<&str> = policy.split(" ").collect();
    let letter = policy_str[1];
    let min_max: Vec<&str> = policy_str[0].split("-").collect();
    let min: usize = min_max[0].parse().unwrap();
    let max :usize = min_max[1].parse().unwrap();

    let letter_count = password.matches(letter).count();
    if letter_count >= min && letter_count <= max {
       return true;
    }
    return false;
}

fn is_valid_two(password: &str, policy: &str) -> bool {
    let password_vec: Vec<char> = password.chars().collect();
    let policy_str: Vec<&str> = policy.split(" ").collect();
    let letter: char = policy_str[1].parse().unwrap();
    let min_max: Vec<&str> = policy_str[0].split("-").collect();
    let p1: usize = min_max[0].parse().unwrap();
    let p2 :usize = min_max[1].parse().unwrap();

    let mut counter = 0;
    if password_vec[p1-1] == letter {
        counter += 1;
    }
    if password_vec[p2-1] == letter {
        counter += 1;
    }

    match counter {
        1 => true,
        _ => false
    }
}

pub fn one() -> i32 {
    let input = fs::read_to_string("src/day2/input").expect("Unable to read file");
    let strings: Vec<& str> = input.split("\n").collect();
    let passwords: Vec<Vec<&str>> = (strings.iter().map(|x: &&str| {x.split(": ").collect()})).collect();

    let mut counter: i32 = 0;
    for pass in passwords {
        if is_valid_one(pass[1], pass[0]) {
            counter += 1;
        }
    }

    return counter ;
}

pub fn two() -> i32 {

    let input = fs::read_to_string("src/day2/input").expect("Unable to read file");
    let strings: Vec<& str> = input.split("\n").collect();
    let passwords: Vec<Vec<&str>> = (strings.iter().map(|x: &&str| {x.split(": ").collect()})).collect();

    let mut counter: i32 = 0;
    for pass in passwords {
        if is_valid_two(pass[1], pass[0]) {
            counter += 1;
        }
    }

    return counter ;
}
