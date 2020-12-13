use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents = fs::read_to_string(filename).expect("Something  went wrong reading the file");
    let mut lines = contents.lines().into_iter();

    let current_time = lines.next().unwrap().parse::<usize>().unwrap();
    let (next_bus_time, next_bus_id) = lines
        .next()
        .unwrap()
        .split(",")
        .filter(|b| b != &"x")
        .map(|b| b.parse::<usize>().unwrap())
        .map(|b| ((((current_time / b) + 1) * b), b))
        .filter(|(time, _)| time > &current_time)
        .min_by(|(a_time, _), (b_time, _)| a_time.cmp(b_time))
        .unwrap();

    println!(
        "Minute to wait: {}",
        (next_bus_time - current_time) * next_bus_id
    );
}
