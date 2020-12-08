use std::env;
use std::fs;

enum Operation {
    NOP,
    ACC,
    JMP,
}
struct Instruction {
    operation: Operation,
    argument: isize,
}

impl Instruction {
    fn from_str(line: &str) -> Instruction {
        let commands: Vec<&str> = line.split_whitespace().collect();
        Instruction {
            operation: match commands[0] {
                "acc" => Operation::ACC,
                "jmp" => Operation::JMP,
                _ => Operation::NOP,
            },
            argument: match commands[1].parse::<isize>() {
                Ok(v) => v,
                Err(_) => 0,
            },
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let instructions: Vec<Instruction> = fs::read_to_string(filename)
        .expect("Something  went wrong reading the file")
        .lines()
        .map(|l| Instruction::from_str(l))
        .collect();

    let mut index: usize = 0;
    let mut instructions_ran: Vec<usize> = Vec::new();
    let mut value: isize = 0;
    while !instructions_ran.contains(&index) {
        instructions_ran.push(index);
        let instruction = instructions.get(index).unwrap();
        match &instruction.operation {
            Operation::ACC => {
                value += instruction.argument;
                index += 1;
            }
            Operation::JMP => {
                index = (index as isize + instruction.argument) as usize;
            }
            Operation::NOP => index += 1,
        }
    }
    println!("Value after first round is {}", value);
}
