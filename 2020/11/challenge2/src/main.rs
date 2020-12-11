use std::env;
use std::fs;

const SEAT_EMPTY: char = 'L';
const SEAT_TAKEN: char = '#';

#[derive(Debug)]
struct Movement {
    x: isize,
    y: isize,
}

impl Movement {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
        }
    }
}

const UP: Movement = Movement { x: -1, y: 0 };
const UP_RIGHT: Movement = Movement { x: -1, y: 1 };
const RIGHT: Movement = Movement { x: 0, y: 1 };
const DOWN_RIGHT: Movement = Movement { x: 1, y: 1 };
const DOWN: Movement = Movement { x: 1, y: 0 };
const DOWN_LEFT: Movement = Movement { x: 1, y: -1 };
const LEFT: Movement = Movement { x: 0, y: -1 };
const UP_LEFT: Movement = Movement { x: -1, y: -1 };

const MOVEMENTS: [Movement; 8] = [
    UP, UP_RIGHT, RIGHT, DOWN_RIGHT, DOWN, DOWN_LEFT, LEFT, UP_LEFT,
];

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];
    let mut map: Vec<Vec<char>> = fs::read_to_string(filename)
        .expect("Something  went wrong reading the file")
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let mut seats_take_on_last_round = 0;
    let mut rounds = 0;
    loop {
        apply_round(&mut map);
        rounds += 1;
        let seats_taken = seats_taken(&map);
        if seats_take_on_last_round == seats_taken {
            break;
        }
        // print_state(&map);
        seats_take_on_last_round = seats_taken;
    }

    println!(
        "After {} rounds, {} seats have been taken",
        rounds,
        seats_taken(&map)
    );
}

fn seats_taken(state: &Vec<Vec<char>>) -> usize {
    state
        .iter()
        .map(|row| row.iter().filter(|c| c == &&SEAT_TAKEN).count())
        .sum()
}
// fn print_state(state: &Vec<Vec<char>>) {
//     println!("---------------");
//     for row in state.iter() {
//         println!("{:?}", row);
//     }
// }
fn apply_round(state: &mut Vec<Vec<char>>) {
    let clone: Vec<Vec<char>> = state.clone();
    for (r_i, row) in clone.iter().enumerate() {
        for (p_i, place) in row.iter().enumerate() {
            let adjacent_seats_taken = adjacent_seats_taken(r_i, p_i, &clone);
            if place == &SEAT_EMPTY && adjacent_seats_taken == 0 {
                state[r_i][p_i] = SEAT_TAKEN;
            } else if place == &SEAT_TAKEN && adjacent_seats_taken > 4 {
                state[r_i][p_i] = SEAT_EMPTY;
            }
        }
    }
}

fn adjacent_seats_taken(row: usize, column: usize, state: &Vec<Vec<char>>) -> usize {
    let mut seats = 0;

    for direction in MOVEMENTS.iter() {
        if next_seat_to_direction_is_taken(row, column, direction.clone(), state) {
            seats += 1;
        }
    }
    seats
}

fn next_seat_to_direction_is_taken(
    row: usize,
    column: usize,
    direction: Movement,
    state: &Vec<Vec<char>>,
) -> bool {
    let next_x: isize = row as isize + direction.x;
    let next_y: isize = column as isize + direction.y;
    if 0 <= next_x
        && next_x < state.len() as isize
        && 0 <= next_y
        && next_y < state[row].len() as isize
    {
        match state[next_x as usize][next_y as usize] {
            SEAT_EMPTY => {
                return false;
            }
            SEAT_TAKEN => {
                return true;
            }
            _ => {
                return next_seat_to_direction_is_taken(
                    next_x as usize,
                    next_y as usize,
                    direction,
                    state,
                );
            }
        }
    } else {
        false
    }
}
