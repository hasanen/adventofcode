use std::env;
use std::fs;
use std::collections::HashMap;
use regex::Regex;

const EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];
    let mut valid_passports: usize = 0;

    let contents = fs::read_to_string(filename)
        .expect("Something  went wrong reading the file");

    let mut current_group: HashMap::<&str, &str> = HashMap::new();
    for (_index, line) in contents.lines().enumerate() {
        if line.len() > 0 {
            for key_value in line.trim().split_whitespace(){
                let key_value_arr: Vec<&str> = key_value.split(":").collect();
                match current_group.insert(key_value_arr[0].trim(), key_value_arr[1].trim()) {
                    Some(_) => (),
                    None => ()
                }
            }
        } else {
            if valid_passport(current_group) { valid_passports += 1; }            
            current_group = HashMap::new();
        }
    }
    println!("Found {} valid passports", valid_passports);
}

fn valid_passport(fields: HashMap<&str, &str>) -> bool {
    let valid_passport: bool = in_range(1920, 2002, fields.get("byr").unwrap_or(&"0")) &&
    in_range(2010, 2020, fields.get("iyr").unwrap_or(&"0")) &&
    in_range(2020, 2030, fields.get("eyr").unwrap_or(&"0")) &&
    valid_height(fields.get("hgt").unwrap_or(&"")) &&
    valid_hair_color(fields.get("hcl").unwrap_or(&"")) &&
    valid_eye_color(fields.get("ecl").unwrap_or(&"")) &&
    valid_pid(fields.get("pid").unwrap_or(&""));

    // println!("{:?} is valid: {}", fields, valid_passport);
    return valid_passport;
}
fn in_range(start: usize, end: usize, value: &str) -> bool {
    let result: bool = (start..end + 1).contains(&value.parse::<usize>().unwrap_or(0));
    // println!("{}-{}, {} is in range: {}", start, end, value, result);

    return result; 
}
fn valid_height(value: &str) -> bool {    
    if value.trim().len() == 0 { return false } 

    let height_capture_regex: Regex = Regex::new(r"^(\d+)\D*$").unwrap();
    let caps = height_capture_regex.captures(value).unwrap();
    // println!("{} {}", value, &caps[1]);

    if value.contains("cm") {
        return in_range(150, 193, &caps[1]);
    }
    if value.contains("in") {
        return in_range(59, 76, &caps[1]);
    }
    return false;
}
fn valid_hair_color(hair_color: &str) -> bool {
    let hair_color_regex: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    // println!("{}: {}", hair_color, hair_color_regex.is_match(hair_color));
    return hair_color_regex.is_match(hair_color);
}
fn valid_eye_color(eye_color: &str) -> bool {
    // println!("{}: {}", eye_color, EYE_COLORS.contains(&eye_color));
    return EYE_COLORS.contains(&eye_color);
}
fn valid_pid( value: &str) -> bool {
    if value.len() != 9  {
        // println!("{}: {}", value, false);
        return false;
    }
    let result = match value.parse::<usize>() {
        Ok(_) => true,
        _ => false
    };

    // println!("{}: {}", value, result);
    return result;
}