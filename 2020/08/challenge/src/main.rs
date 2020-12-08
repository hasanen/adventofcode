use std::env;
use std::fs;

#[derive(PartialEq, Debug)]
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

    println!("Value after first round is {}", challenge1(&instructions));
    println!("Value of fixed boot sequence {}", challenge2(&instructions));
}

fn challenge1(instructions: &Vec<Instruction>) -> isize {
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
    value
}

fn challenge2(instructions: &Vec<Instruction>) -> isize {
    let mut value: isize = 0;
    for (_nop_jmp_index, nop_jmp_index) in instructions
        .iter()
        .enumerate()
        .map(|(index, i)| {
            if i.operation == Operation::NOP || i.operation == Operation::JMP {
                index as isize
            } else {
                -1
            }
        })
        .filter(|v| v > &0)
        .enumerate()
    {
        let mut index: usize = 0;
        let mut instructions_ran: Vec<usize> = Vec::new();
        while index < instructions.len() {
            if instructions_ran.contains(&index) {
                value = 0;
                instructions_ran.clear();
                break;
            }
            instructions_ran.push(index);
            let instruction = instructions.get(index).unwrap();
            let mut op: &Operation = &instruction.operation;
            if nop_jmp_index as usize == index && op == &Operation::NOP {
                op = &Operation::JMP
            } else if nop_jmp_index as usize == index && op == &Operation::JMP {
                op = &Operation::NOP
            }
            match &op {
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
        if value > 0 {
            println!("Change needed to be made for line {}", nop_jmp_index + 1);
            break;
        };
    }
    value
}
