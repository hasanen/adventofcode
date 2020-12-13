use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

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

    println!(
        "challenge 1: {} * {} = {}",
        difference_of_one,
        difference_of_three,
        difference_of_one * difference_of_three
    );
}
fn challenge2(mut adapters: Vec<usize>) {
    let start = Instant::now();
    adapters.push(adapters.iter().last().unwrap() + 3);
    adapters.push(0);
    adapters.sort();

    let mut set_of_paths = HashSet::new();
    set_of_paths.insert(adapters.to_vec());
    let clone = set_of_paths.clone();
    try_to_remove_one(&mut set_of_paths, &clone);
    println!(
        "Challenge 2: {}, took {:?}",
        set_of_paths.len(),
        start.elapsed()
    );
}

fn try_to_remove_one(set_of_paths: &mut HashSet<Vec<usize>>, paths_to_check: &HashSet<Vec<usize>>) {
    let mut new_paths = HashSet::new();
    for path in paths_to_check.clone().iter() {
        for (i, x) in path.iter().enumerate() {
            if i == 0 || i == path.len() - 1 {
                continue;
            }
            let numbers: Vec<_> = path.iter().filter(|n| n != &x).map(|n| *n).collect();
            if !set_of_paths.contains(&numbers) && valid_path(&numbers) {
                new_paths.insert(numbers.to_vec());
            }
        }
    }
    if new_paths.len() > 0 {
        set_of_paths.extend(new_paths.clone());
        try_to_remove_one(set_of_paths, &new_paths);
    }
}

fn valid_path(adapters: &Vec<usize>) -> bool {
    let mut last: usize = 0;
    for number in adapters.iter() {
        if number - last > 3 {
            return false;
        }
        last = *number;
    }
    // println!("{:?}: {}", adapters, true);
    true
}
