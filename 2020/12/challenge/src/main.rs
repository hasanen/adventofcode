use std::env;
use std::fs;

const DIRECTIONS: [Direction; 4] = [
    Direction::NORTH,
    Direction::EAST,
    Direction::SOUTH,
    Direction::WEST,
];
#[derive(Debug, Clone, Copy)]
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
            Direction::NORTH | Direction::EAST | Direction::SOUTH | Direction::WEST => {
                self.movemove(instruction.direction, instruction.value as isize)
            }
            Direction::RIGHT | Direction::LEFT => {
                self.turn_ship(instruction.direction, instruction.value);
            }
            Direction::FORWARD => self.movemove(self.direction, instruction.value as isize),
            Direction::NONE => (),
        }
        // println!(
        //     "Ship' x: {}, Y: {}, dir: {:?}",
        //     self.x, self.y, self.direction
        // );
    }

    fn turn_ship(&mut self, direction: Direction, value: usize) {
        let steps: isize = value as isize / 90;
        let mut index: isize = match self.direction {
            Direction::NORTH => 0,
            Direction::EAST => 1,
            Direction::SOUTH => 2,
            Direction::WEST => 3,
            _ => 0,
        };
        match direction {
            Direction::LEFT => index -= steps,
            Direction::RIGHT => index += steps,
            _ => {}
        };

        if index < 0 {
            index += 4
        }
        if index > 3 {
            index -= 4
        }
        self.direction = DIRECTIONS[index as usize];
    }

    fn movemove(&mut self, direction: Direction, value: isize) {
        match direction {
            Direction::NORTH => self.x += value,
            Direction::EAST => self.y += value,
            Direction::SOUTH => self.x -= value,
            Direction::WEST => self.y -= value,
            _ => (),
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
