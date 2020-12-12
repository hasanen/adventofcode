use std::env;
use std::fs;

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

#[derive(Debug, Clone, Copy)]
struct Instruction {
    direction: Direction,
    value: usize,
}

#[derive(Debug)]
struct Ship {
    x: isize,
    y: isize,
    waypoint_x: isize,
    waypoint_y: isize,
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
    fn sail(&mut self, instruction: Instruction) {
        match instruction.direction {
            Direction::NORTH | Direction::EAST | Direction::SOUTH | Direction::WEST => {
                self.move_waypoint(instruction.direction, instruction.value as isize)
            }
            Direction::RIGHT | Direction::LEFT => {
                self.turn_ship(instruction.direction, instruction.value);
            }
            Direction::FORWARD => self.move_ship(instruction.value as isize),
            Direction::NONE => (),
        }
        println!(
            "Ship' x: {}, y: {}, waypoint_x: {}, waypoint_y: {}",
            self.x, self.y, self.waypoint_x, self.waypoint_y
        );
    }

    fn turn_ship(&mut self, direction: Direction, value: usize) {
        let steps = value / 90;
        let mut arr = [
            if self.waypoint_x > 0 {
                self.waypoint_x as usize
            } else {
                0
            },
            if self.waypoint_y > 0 {
                self.waypoint_y as usize
            } else {
                0
            },
            if self.waypoint_x < 0 {
                self.waypoint_x.abs() as usize
            } else {
                0
            },
            if self.waypoint_y < 0 {
                self.waypoint_y.abs() as usize
            } else {
                0
            },
        ];
        match direction {
            Direction::LEFT => arr.rotate_left(steps),
            Direction::RIGHT => arr.rotate_right(steps),
            _ => {}
        };
        self.waypoint_x = arr[0] as isize - arr[2] as isize;
        self.waypoint_y = arr[1] as isize - arr[3] as isize;
    }

    fn move_ship(&mut self, value: isize) {
        if self.waypoint_x > 0 {
            self.x += self.waypoint_x * value;
        } else {
            self.x += self.waypoint_x * value;
        }
        if self.waypoint_y > 0 {
            self.y += self.waypoint_y * value;
        } else {
            self.y += self.waypoint_y * value;
        }
    }

    fn move_waypoint(&mut self, direction: Direction, value: isize) {
        match direction {
            Direction::NORTH => self.waypoint_x += value,
            Direction::EAST => self.waypoint_y += value,
            Direction::SOUTH => self.waypoint_x -= value,
            Direction::WEST => self.waypoint_y -= value,
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
        waypoint_x: 1,
        waypoint_y: 10,
    };
    for instruction in instructions {
        ship.sail(instruction);
    }
    println!(
        "Ship's distance between origin and current manhattan distance is {}",
        ship.manhattan_distance()
    );
}
