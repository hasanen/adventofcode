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
    value: usize,
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
                value: splits[1].trim().parse::<usize>().unwrap(),
            })
        }
    }
}

impl Operation {
    fn unmasked_value(&self, mask: &Mask) -> String {
        let mut result = "".to_string();
        let address_as_bits = format!("{:b}", self.address);
        println!("{}", address_as_bits);
        println!("{}", mask.value);
        // for (i, bit) in mask.value.chars().enumerate() {
        //     match bit {
        //         'X' => {
        //             if i + self.value.len() < mask.value.len() {
        //                 result.push('0')
        //             } else {
        //                 result.push(
        //                     self.address
        //                         .chars()
        //                         .nth((i + self.value.len()) - mask.value.len())
        //                         .unwrap_or('0'),
        //                 )
        //             }
        //         }
        //         '0' | '1' => result.push(bit),
        //         _ => (),
        //     }
        // }
        result.to_string()
    }
}
impl Memory {
    fn write(&mut self, operation: &Operation, mask: &Mask) {
        // println!("----------------");
        for address in self.unmasked_addresses(operation.address, &mask.value) {
            // println!(
            //     "{} is {}",
            //     address,
            //     usize::from_str_radix(&address, 2).unwrap()
            // );
            self.memory.insert(
                usize::from_str_radix(&address, 2).unwrap() as u64,
                operation.value,
            );
        }
    }
    fn total_values(&self) -> usize {
        self.memory.values().sum()
    }
    fn unmasked_addresses(&self, address: u64, mask: &str) -> Vec<String> {
        let address_as_bits = format!("{:b}", address);
        let mut result = "".to_string();
        for (i, bit) in mask.chars().enumerate() {
            match bit {
                '0' => {
                    if i + address_as_bits.len() < mask.len() {
                        result.push('0')
                    } else {
                        result.push(
                            address_as_bits
                                .chars()
                                .nth((i + address_as_bits.len()) - mask.len())
                                .unwrap_or('0'),
                        )
                    }
                }
                '1' | 'X' => result.push(bit),
                _ => (),
            }
        }
        let mut addresses: Vec<String> = Vec::new();
        let count_of_x: u32 = result.matches("X").count() as u32;
        for num in 0..(2 as usize).pow(count_of_x) {
            let mut real_address = "".to_string();
            let num_as_bits = format!(
                "{:0>length$}",
                format!("{:b}", num),
                length = count_of_x as usize
            );
            let mut x = 0;
            for (i, bit) in result.chars().enumerate() {
                match bit {
                    'X' => {
                        real_address.push(num_as_bits.chars().nth(x).unwrap_or('0'));
                        x += 1;
                    }
                    '1' | '0' => real_address.push(bit),
                    _ => (),
                }
            }
            addresses.push(real_address.to_string());
        }
        addresses
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
            Instruction::Operation(op) => memory.write(op, &current_mask),
        }
    }
    println!("Sum of all values is {}", memory.total_values());
}
