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
    set_of_paths.insert(adapters);
    let num_of_paths = try_to_remove_one(&set_of_paths) + 1;
    println!("Challenge 2: {}, took {:?}", num_of_paths, start.elapsed());
}

fn try_to_remove_one(paths_to_check: &HashSet<Vec<usize>>) -> usize {
    let mut new_paths = HashSet::new();
    for path in paths_to_check.iter() {
        let last_index = path.len() - 1;
        for (i, _) in path.iter().enumerate() {
            if i == 0 || i == last_index {
                continue;
            }
            let numbers = [&path[..i], &path[i + 1..]].concat();
            if valid_path(&numbers) {
                new_paths.insert(numbers);
            }
        }
    }
    if new_paths.len() > 0 {
        return new_paths.len() + try_to_remove_one(&new_paths);
    } else {
        return 0;
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
