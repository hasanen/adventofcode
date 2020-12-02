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
        
        let pos1: &usize = &caps[1].parse::<usize>().unwrap_or(0);
        let pos2: &usize = &caps[2].parse::<usize>().unwrap_or(0);
        let pw_chars = &caps[4].as_bytes();
        let letter = &caps[3].chars().nth(0).unwrap();
        let char_pos1 = pw_chars[*pos1 - 1] as char;
        let char_pos2 = pw_chars[*pos2 - 1] as char;

        if char_pos1 != char_pos2 && (*letter == char_pos1 || *letter == char_pos2) {
            valid_passwords += 1;
        }
    }
    println!("Found {} valid passwords", valid_passwords);
}
