use std::env;
use std::fs;

fn main() {
    let mut tree_count: usize = 0;
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];
    let slope_right: usize = 3;
    let tree: char = "#".chars().next().unwrap();

    let contents = fs::read_to_string(filename)
        .expect("Somethingd  went wrong reading the file");

    let mut y: usize = 0;
    for (_index, line) in contents.lines().enumerate() {
        let chars = line.as_bytes();
        if chars[y] as char == tree {
            tree_count += 1;
        }
        y += slope_right;
        if y >= chars.len() {
            y -= chars.len();
        }
    }
    println!("Found {} trees", tree_count);
}
