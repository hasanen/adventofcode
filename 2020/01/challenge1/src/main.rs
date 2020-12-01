use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents = fs::read_to_string(filename)
                .expect("Something went wrong reading the file");

    let mut numbers = Vec::new();
    for (_index, line) in contents.lines().enumerate() {
        numbers.push(line.parse::<u32>().unwrap_or(0))
    }

    let mut number_one: &u32 = &0;
    let mut number_two: &u32 = &0;
    'outer: for x in &numbers {
        for y in &numbers {
            if x + y == 2020 {
                println!("Match! {} + {}", x, y);
                number_one = x;
                number_two = y;
                break 'outer;
            }
        }
    }

    println!("Correct answer is: {}", number_one * number_two)
}
