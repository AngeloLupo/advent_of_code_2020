use std::fs;


pub fn one(instruction_index: usize) -> (bool, i32) {
    let input = fs::read_to_string("src/day8/input").expect("Unable to read file");
    let instructions: Vec<&str> = input.split("\n").collect();

    let mut index: usize = 0;
    let mut accumulator: i32 = 0;

    let mut visited_indexes: Vec<usize> = Vec::new();

    loop{

        if index > instructions.len()-1 {
            return (true, accumulator);
        }

        if visited_indexes.contains(&index) {
            return (false, accumulator);
        }
        visited_indexes.push(index);

        let split_instruction: Vec<&str> = instructions[index].split(" ").collect();
        let mut instruction = split_instruction[0];

        let operator: char = split_instruction[1].as_bytes()[0] as char;
        let operand_str = &split_instruction[1][1..];
        let operand: i32 = operand_str.parse().unwrap();


        if index == instruction_index {
            if instruction == "nop" {
                println!("changing instruction {}", index);
                instruction = "jmp";
            }
            if instruction == "jmp" {
                println!("changing instruction {}", index);
                instruction = "nop";
            }
        }

        match instruction {
            "nop" => {
                index += 1
            },
            "acc" => {
                match operator {
                    '+' => accumulator += operand,
                    '-' => accumulator -= operand,
                    _ => println!("[ACC] Weird operator: {} at index {}", operator, index)
                }
                index += 1;
            }
            "jmp" => {
                match operator {
                    '+' => index += operand as usize,
                    '-' => index -= operand as usize,
                    _ => println!("[JMP] Weird operator: {} at index {}", operator, index)
                }
            },
            _ => println!("Weird instruction: {}", operator)
        }
    }
}


pub fn two() -> i32 {
    let input = fs::read_to_string("src/day8/input").expect("Unable to read file");
    let instructions: Vec<&str> = input.split("\n").collect();

    let mut index: usize = 0;
    let mut indexes: Vec<usize> = Vec::new();

    loop {
        if index > instructions.len()-1 {
            break;
        }
        let split_instruction: Vec<&str> = instructions[index].split(" ").collect();
        let instruction = split_instruction[0];

        if (instruction == "nop") | (instruction == "jmp") {
            indexes.push(index)
        }
        index += 1;
    }

    for index_to_change in indexes {
        println!("trying index {}", index_to_change);
        let result: (bool, i32) = one(index_to_change);
        if result.0 == true{
            return result.1;
        }
    }
    return 0;
}