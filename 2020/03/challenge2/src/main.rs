use std::env;
use std::fs;
use std::str::FromStr;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Something  went wrong reading the file");

    println!("Found {} trees", 
        count_trees(&contents, 1, 1) *
        count_trees(&contents, 1, 3) *
        count_trees(&contents, 1, 5) *
        count_trees(&contents, 1, 7) *
        count_trees(&contents, 2, 1) 
        );
}

fn count_trees(contents: &str, slope_down: usize, slope_right: usize) -> usize {
    let tree: char = char::from_str("#").unwrap();
    let mut y: usize = 0;
    let mut tree_count: usize = 0;
    for (_index, line) in contents.lines().enumerate().step_by(slope_down) {
        let chars = line.as_bytes();
        if chars[y] as char == tree {
            tree_count += 1;
        }
        y += slope_right;
        if y >= chars.len() {
            y -= chars.len();
        }
    }
    return tree_count;
}
