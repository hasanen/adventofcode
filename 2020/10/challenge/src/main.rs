use std::env;
use std::fs;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let mut adapters: Vec<usize> = fs::read_to_string(filename)
        .expect("Something  went wrong reading the file")
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect();

    adapters.sort();
    challenge1(&adapters);
    challenge2(adapters.to_vec());
}

fn challenge1(adapters: &Vec<usize>) {
    let outlet_joltage = 0;
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

    println!("challenge 1: {} * {} = {}", difference_of_one, difference_of_three, difference_of_one * difference_of_three);
}
fn challenge2(mut adapters: Vec<usize>) {
    adapters.push(adapters.iter().last().unwrap() + 3);
    adapters.push(0);
    adapters.sort();

    let mut set_of_paths  = HashSet::new();
    set_of_paths.insert(adapters.to_vec());
    try_to_remove_one(&mut set_of_paths, &adapters, 1);
    println!("Challenge 2: {}", set_of_paths.len());
}

fn try_to_remove_one(set_of_paths: &mut HashSet<Vec<usize>>, list: &Vec<usize>, level: usize) {
    for (i, x) in list.iter().enumerate() {
        if level == 1 {
            println!("Starting to work on letter {}/{}", i + 1, list.len())
        }
        if i == 0 || i == list.len() -1 {
            continue;
        }
        let numbers = list
            .iter()
            .filter(|n| n != &x  )
            .map(|n| *n)
            .collect();

        if valid_path(&numbers) {
            set_of_paths.insert(numbers.to_vec());
            try_to_remove_one(set_of_paths, &numbers, level + 1) ;
        }
        if level == 1 {
            println!("Done with letter {}/{}", i + 1, list.len())
        }
    }
}

fn valid_path(adapters: &Vec<usize>) -> bool {
    let mut last: usize = 0;
    for number in adapters.iter() {
        if number - last > 3 {
            return false
        }
        last = *number;
    }
    // println!("{:?}: {}", adapters, true);
    true
}