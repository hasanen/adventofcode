use std::env;
use std::fs;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut valid_passwords: u16 = 0;
    let pw_line_regex: Regex = Regex::new(r"(\d+)-(\d+)\s(\D):\s(\w+)").unwrap();
    for (_index, line) in contents.lines().enumerate() {
        let caps = pw_line_regex.captures(line).unwrap();
        
        let min: &usize = &caps[1].parse::<usize>().unwrap_or(0);
        let max: &usize = &caps[2].parse::<usize>().unwrap_or(0);
        let count = &caps[4].matches(&caps[3]).count();

        if min <= count && count <= max {
            valid_passwords += 1;
        }
    }
    println!("Found {} valid passwords", valid_passwords);
}
