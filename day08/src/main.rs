use std::{collections::HashSet, io::BufRead};

enum Operation {
    Acc,
    Jmp,
    Nop,
}

struct Instruction {
    operation: Operation,
    operand: i32,
}

impl Instruction {
    fn from(line: &str) -> Instruction {
        let mut iter = line.split(" ");
        let operation = match iter.next().unwrap() {
            "acc" => Operation::Acc,
            "jmp" => Operation::Jmp,
            "nop" => Operation::Nop,
            _ => unreachable!(),
        };
        let operand = iter.next().unwrap().parse::<i32>().unwrap();

        Instruction {
            operation,
            operand,
        }
    }
}

fn run(program: &Vec<Instruction>) -> (bool, i32) {
    let mut program_counter = 0;
    let mut accumulator = 0;
    let mut history = HashSet::new();
    while program_counter < program.len() {
        if history.contains(&program_counter) {
            return (false, accumulator);
        }

        history.insert(program_counter);

        match program[program_counter].operation {
            Operation::Acc => {
                accumulator += program[program_counter].operand;
                program_counter += 1;
            },
            Operation::Jmp => {
                program_counter = ((program_counter as i32) + program[program_counter].operand) as usize;
            },
            Operation::Nop => {
                program_counter += 1;
            },
        }
    }

    (true, accumulator)
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut program = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        program.push(Instruction::from(&line));
    }

    println!("{}", run(&program).1);

    for i in 00..program.len() {
        match program[i].operation {
            Operation::Acc => {}
            Operation::Jmp => {
                program[i].operation = Operation::Nop;
                let (terminated, accumulator) = run(&program);
                program[i].operation = Operation::Jmp;
                if terminated {
                    println!("{}", accumulator);
                    break; 
                }
            }
            Operation::Nop => {
                program[i].operation = Operation::Jmp;
                let (terminated, accumulator) = run(&program);
                program[i].operation = Operation::Nop;
                if terminated {
                    println!("{}", accumulator);
                    break; 
                }
            }
        }
    }
}
