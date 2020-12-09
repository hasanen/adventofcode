use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];
    let preamble: &usize = &args[2].parse::<usize>().unwrap();

    let numbers: Vec<usize> = fs::read_to_string(filename)
        .expect("Something  went wrong reading the file")
        .lines()
        .map(|l| l.parse::<usize>().unwrap() )
        .collect();

    let number: usize = **numbers_not_sum_in_last_n_numbers(&numbers, *preamble).first().unwrap();
    println!("First number with preamble {} that is not sum of two numbers is {:?}", preamble, number);
    println!("Sum of min and max is {}", sum_of_min_and_max_in_contiguous_set_for(&numbers, number));
}
fn numbers_not_sum_in_last_n_numbers(numbers: &Vec<usize>, preamble: usize ) -> Vec<&usize>  {
    let mut numbers_not_sum: Vec<&usize> = Vec::new();
    for i in preamble..numbers.len() {
        let number: &usize = numbers.get(i).unwrap();
        let set_of_numbers: &[usize] = &numbers.as_slice()[i-preamble..i];
        if !is_sum(set_of_numbers, *number) {
            numbers_not_sum.push(number);
        }
    }
    numbers_not_sum
}
fn is_sum(numbers: &[usize], number: usize) -> bool{
    for number1 in numbers.iter() {
        for number2 in numbers.iter() {
            if number1 == number2 {
                continue
            }
            if number1 + number2 == number {
                return true
            }
        }
    }
    false
}
fn sum_of_min_and_max_in_contiguous_set_for(numbers: &Vec<usize>, number: usize ) -> usize {
    for start_index in 0..numbers.len() {
        let mut nums: Vec<usize> = Vec::new();
        for i in start_index..numbers.len() {
            let v: usize = numbers[i];
            nums.push(v);
            if i > 2 {
                let sum: usize = nums.iter().sum();
                if sum == number {
                    return nums.iter().min().unwrap() + nums.iter().max().unwrap();
                } else if sum > number{
                    break;
                }
            }
        }
    }
    0
}