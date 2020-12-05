use std::env;
use std::fs;

mod seat;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Something  went wrong reading the file");

    let seats: Vec<seat::Seat> = contents.lines().map(|line| seat::Seat::from_str(line)).collect();
    println!("Seat with highest ID: {}", seats.iter().max().unwrap());
    println!("My seat ID: {}", my_seat(seats).unwrap());

}
fn my_seat(seats: Vec<seat::Seat>) -> Option<u32> {
    let mut seat_id: Option<u32> = None;
    let mut clone: Vec<seat::Seat> = seats;
    clone.sort();
    let mut index = 1;
    let mut last_seat: &seat::Seat = clone.get(0).unwrap();
    while seat_id == None {
        let current_seat: &seat::Seat = clone.get(index).unwrap();
        if current_seat.id - last_seat.id == 2 {
            seat_id = Some(current_seat.id - 1);
        }
        index += 1;
        last_seat = current_seat;
    }
    return seat_id;
}