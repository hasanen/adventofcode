use std::env;
use std::fs;
use std::collections::HashMap;

const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];
    let mut valid_passports: usize = 0;

    let contents = fs::read_to_string(filename)
        .expect("Something  went wrong reading the file");

    let mut current_group: HashMap::<&str, &str> = HashMap::new();
    for (_index, line) in contents.lines().enumerate() {
        if line.len() > 0 {
            for key_value in line.split_whitespace(){
                let key_value_arr: Vec<&str> = key_value.split(":").collect();
                match current_group.insert(key_value_arr[0].trim(), key_value_arr[1].trim()) {
                    Some(_) => (),
                    None => ()
                }
            }
        } else {
            let mut valid: bool = true;
            for key in REQUIRED_FIELDS.iter() {
              if !current_group.contains_key(key) {
                  valid = false;
                  continue;
              }
            }
            if valid { valid_passports += 1; }            
            current_group = HashMap::new();
        }
    }
    println!("Found {} valid passports", valid_passports);
}
