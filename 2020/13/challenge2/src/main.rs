use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents = fs::read_to_string(filename).expect("Something  went wrong reading the file");
    let buses: Vec<usize> = contents
        .lines()
        .into_iter()
        .nth(1)
        .unwrap()
        .split(",")
        .map(|b| match b.parse::<usize>() {
            Ok(v) => v,
            Err(_) => 0,
        })
        .collect();
    let start = Instant::now();
    let biggest_bus_id = buses.iter().max().unwrap();
    let biggest_bus_id_index = buses.iter().position(|x| x == biggest_bus_id).unwrap();
    let earliest_timestamp: usize;
    let mut time = (100000000000000 / biggest_bus_id) * biggest_bus_id;
    'outer: loop {
        time += biggest_bus_id;

        for (i, id) in buses.iter().enumerate() {
            if id != &0
                && ((time as isize + (i as isize - biggest_bus_id_index as isize)) as usize)
                    % buses[i]
                    > 0
            {
                continue 'outer;
            }
        }
        earliest_timestamp = time - biggest_bus_id_index;
        break;
    }

    println!(
        "Earliest timestamp: {}, took {:?}",
        earliest_timestamp,
        start.elapsed()
    );
}
