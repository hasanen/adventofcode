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

    fn everyone_answered_yes(&self) -> usize {
        let mut clone = self.answers.clone();
        clone.sort_by(|a, b| a.len().cmp(&b.len()));
        clone
            .first().unwrap()
            .chars()
            .filter(|c| self.answers.iter().filter(|a| a.contains(&*c.to_string())).count() == self.answers.len())
            .count()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Something  went wrong reading the file");

    let groups: Vec<AnswerGroup> = contents.split("\n\n").map (|group_text| AnswerGroup::from_str(group_text)).collect();

    println!("Count all yes answers: {}", groups.iter().map(|ag| ag.uniq_yes_answers()).sum::<usize>());
    println!("Count answers which everyone answered yes: {}", groups.iter().map(|ag| ag.everyone_answered_yes()).sum::<usize>());
}