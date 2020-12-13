use std::env;
use std::fs;

const SEAT_EMPTY: char = 'L';
const SEAT_TAKEN: char = '#';

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
            } else if place == &SEAT_TAKEN && adjacent_seats_taken > 3 {
                state[r_i][p_i] = SEAT_EMPTY;
            }
        }
    }
}
fn adjacent_seats_taken(row: usize, column: usize, state: &Vec<Vec<char>>) -> usize {
    let mut seats = 0;
    if row > 0 {
        //seats above
        if column > 0 && state[row - 1][column - 1] == SEAT_TAKEN {
            seats += 1
        }
        if state[row - 1][column] == SEAT_TAKEN {
            seats += 1
        }
        if column + 1 < state[row - 1].len() && state[row - 1][column + 1] == SEAT_TAKEN {
            seats += 1
        }
    }

    if column > 0 && state[row][column - 1] == SEAT_TAKEN {
        seats += 1
    }
    if column + 1 < state[row].len() && state[row][column + 1] == SEAT_TAKEN {
        seats += 1
    }

    if row + 1 < state.len() {
        //seats below
        if column > 0 && state[row + 1][column - 1] == SEAT_TAKEN {
            seats += 1
        }
        if state[row + 1][column] == SEAT_TAKEN {
            seats += 1
        }
        if column + 1 < state[row + 1].len() && state[row + 1][column + 1] == SEAT_TAKEN {
            seats += 1
        }
    }
    seats
}
