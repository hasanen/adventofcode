use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let mut adapters: Vec<usize> = fs::read_to_string(filename)
        .expect("Something  went wrong reading the file")
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect();

    adapters.sort();

    let outlet_joltage = 0;
    let device_joltage = adapters.iter().last().unwrap() + 3;
    let mut difference_of_one = 0;
    let mut difference_of_three = 1;
    let mut last_adapater = outlet_joltage;

    for number in adapters.iter() {
        let result = number - last_adapater;
        if result == 1 {
            difference_of_one += 1;
        } else if result == 3 {
            difference_of_three += 1;
        }
        last_adapater = *number;
    }

    println!("{} + {} = {}", difference_of_one, difference_of_three, difference_of_one * difference_of_three);
}
