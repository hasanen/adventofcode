use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
    RIGHT,
    LEFT,
    FORWARD,
    NONE,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    value: usize,
}

#[derive(Debug)]
struct Ship {
    x: isize,
    y: isize,
    direction: Direction,
}

impl Instruction {
    fn from_str(str: &str) -> Self {
        let (d, v) = str.split_at(1);
        let direction = match d {
            "N" => Direction::NORTH,
            "E" => Direction::EAST,
            "S" => Direction::SOUTH,
            "W" => Direction::WEST,
            "R" => Direction::RIGHT,
            "L" => Direction::LEFT,
            "F" => Direction::FORWARD,
            &_ => Direction::NONE,
        };
        let value = v.parse::<usize>().unwrap();

        Self { direction, value }
    }
}

impl Ship {
    fn manhattan_distance(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }
    fn sail_to(&mut self, instruction: Instruction) {
        match instruction.direction {
            Direction::NORTH => self.x += instruction.value as isize,
            Direction::EAST => self.y += instruction.value as isize,
            Direction::SOUTH => self.x -= instruction.value as isize,
            Direction::WEST => self.y -= instruction.value as isize,
            Direction::RIGHT => (),
            Direction::LEFT => (),
            Direction::FORWARD => match self.direction {
                Direction::NORTH => self.x += instruction.value as isize,
                Direction::EAST => self.y += instruction.value as isize,
                Direction::SOUTH => self.x -= instruction.value as isize,
                Direction::WEST => self.y -= instruction.value as isize,
                _ => (),
            },
            Direction::NONE => (),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let instructions: Vec<_> = fs::read_to_string(filename)
        .expect("Something  went wrong reading the file")
        .lines()
        .map(|l| Instruction::from_str(l))
        .collect();

    let mut ship = Ship {
        x: 0,
        y: 0,
        direction: Direction::EAST,
    };
    for instruction in instructions {
        ship.sail_to(instruction);
    }
    println!("Ship's manhattan distance is {}", ship.manhattan_distance());
}
