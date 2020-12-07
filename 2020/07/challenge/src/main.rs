use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs;

#[derive(Clone)]
struct Bag {
    color: String,
    contents: HashMap<String, u8>,
}

impl Bag {
    fn parse(contents: String) -> Vec<Bag> {
        let mut bags: Vec<Bag> = Vec::new();
        for line in contents.lines() {
            let parts: Vec<&str> = line.split("bags contain").collect();
            let color: String = parts[0].trim().to_string();
            let mut contents: HashMap<String, u8> = HashMap::new();
            for content in parts[1].split(",") {
                if content.trim() != "no other bags." {
                    let split: Vec<&str> = content.split(" ").collect();
                    let count: u8 = split[1].trim().parse::<u8>().unwrap();
                    let colorrr: String = split[2..split.len() - 1].join(" ").trim().to_string();
                    contents.insert(colorrr, count);
                }
            }
            bags.push(Bag { color, contents });
        }
        bags
    }
}

impl fmt::Display for Bag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Color: {}", self.color)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents = fs::read_to_string(filename).expect("Something  went wrong reading the file");

    let desired_color = "shiny gold";
    let bags: Vec<Bag> = Bag::parse(contents);
    let mut bag_count: u16 = 0;
    for bag in bags.iter() {
        if can_hold_bag(&bags, &bag.color, &desired_color) {
            bag_count += 1;
        }
    }
    println!(
        "{} bags can eventually contain {} bag",
        bag_count - 1,
        desired_color
    );
    println!(
        "{} can hold on total of {} bags",
        desired_color,
        total_bag_count_for_color(&bags, desired_color) - 1
    );
}

fn can_hold_bag(bags: &Vec<Bag>, color: &str, desired_color: &str) -> bool {
    if desired_color == color {
        true
    } else {
        if let Some(bag) = bags.iter().find(|b| b.color == color) {
            match bag
                .contents
                .keys()
                .find(|color| can_hold_bag(bags, &color, desired_color))
            {
                Some(_) => true,
                None => false,
            }
        } else {
            false
        }
    }
}
fn total_bag_count_for_color(bags: &Vec<Bag>, desired_color: &str) -> usize {
    let bag: &Bag = bags.iter().find(|b| b.color == desired_color).unwrap();
    let mut total: usize = 1;
    for (color, amount) in bag.contents.iter() {
        let sum = *amount as usize * total_bag_count_for_color(bags, &color);
        total += sum;
    }
    total
}
