use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug)]
struct Mask {
    value: String,
}
#[derive(Debug)]
struct Operation {
    address: u64,
    value: String,
}
#[derive(Debug)]
enum Instruction {
    Mask(Mask),
    Operation(Operation),
}
#[derive(Debug)]
struct Memory {
    memory: HashMap<u64, usize>,
}
impl Instruction {
    fn parse(string: &str) -> Self {
        if string.starts_with("mask") {
            Self::Mask(Mask {
                value: string.split_at(6).1.trim().to_string(),
            })
        } else {
            let splits: Vec<_> = string.split("=").collect();
            let address: Vec<_> = splits[0].split(|c| c == '[' || c == ']').collect();
            Self::Operation(Operation {
                address: address[1].parse::<u64>().unwrap(),
                value: format!("{:b}", splits[1].trim().parse::<usize>().unwrap()),
            })
        }
    }
}

impl Operation {
    fn unmasked_value(&self, mask: &Mask) -> String {
        let mut result = "".to_string();
        for (i, bit) in mask.value.chars().enumerate() {
            match bit {
                'X' => {
                    if i + self.value.len() < mask.value.len() {
                        result.push('0')
                    } else {
                        result.push(
                            self.value
                                .chars()
                                .nth((i + self.value.len()) - mask.value.len())
                                .unwrap_or('0'),
                        )
                    }
                }
                '0' | '1' => result.push(bit),
                _ => (),
            }
        }
        result.to_string()
    }
}
impl Memory {
    fn write(&mut self, address: u64, value: &str) {
        self.memory
            .insert(address, usize::from_str_radix(value, 2).unwrap());
    }
    fn total_values(&self) -> usize {
        self.memory.values().sum()
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let instructions: Vec<Instruction> = fs::read_to_string(filename)
        .expect("Something  went wrong reading the file")
        .lines()
        .map(|l| Instruction::parse(l))
        .collect();

    let mut memory = Memory {
        memory: HashMap::new(),
    };
    let mut current_mask = &Mask {
        value: "".to_string(),
    };
    for instruction in instructions.iter() {
        match instruction {
            Instruction::Mask(mask) => current_mask = &mask,
            Instruction::Operation(op) => {
                memory.write(op.address, &op.unmasked_value(&current_mask))
            }
        }
    }
    println!("Sum of all values is {}", memory.total_values());
}
