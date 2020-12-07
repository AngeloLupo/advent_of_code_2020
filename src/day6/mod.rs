use std::fs;
use std::collections::HashSet;

pub fn one() -> usize {
    let input = fs::read_to_string("src/day6/input").expect("Unable to read file");
    let answer_groups: Vec<&str> = input.split("\n\n").collect();
    let mut total:usize = 0;
    for answer_group in answer_groups {
        let mut set = HashSet::new();
        let mut answers_vec: Vec<&str> = answer_group.split("\n").collect();
        let answers = answers_vec.join("");

        for answer in answers.chars(){
            set.insert(answer);
        }
        total += set.len();
    }

    return total;
}

pub fn two() -> usize {
    let input = fs::read_to_string("src/day6/input").expect("Unable to read file");
    let answer_groups: Vec<&str> = input.split("\n\n").collect();
    let mut total:usize = 0;
    for answer_group in answer_groups {
        let mut set = HashSet::new();
        let mut answers_vec: Vec<&str> = answer_group.split("\n").collect();
        let people = answers_vec.len();

        let answers = answers_vec.join("");

        for answer in answers.chars(){
            set.insert(answer);
        }

        for char in set {
            if answers.matches(char).count() == people {
                total += 1;
            }
        }
    }

    return total;
}