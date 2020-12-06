use std::env;
use std::fs;

struct AnswerGroup {
    answers: Vec<String>
}

impl AnswerGroup {
    fn from_str(answers: &str) -> AnswerGroup {
        AnswerGroup {
            answers: answers.lines().map(|line| String::from(line)).collect()
        }
    }

    fn uniq_yes_answers(&self) -> usize {
        let mut uniq: Vec<char> = Vec::new();
        for letter in self.answers.join("").chars() {
            if !uniq.contains(&letter) { uniq.push(letter) }
        }
        uniq.len()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Something  went wrong reading the file");

    let groups: Vec<AnswerGroup> = contents.split("\n\n").map (|group_text| AnswerGroup::from_str(group_text)).collect();

    println!("Total count: {}", groups.iter().map(|ag| ag.uniq_yes_answers()).sum::<usize>());
}