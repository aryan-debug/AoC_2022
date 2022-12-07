use std::collections::HashSet;
use std::fs;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();
    let datastream = fs::read_to_string("input.txt").expect("Error reading file.");
    println!("Part 1 solution: {:?}", get_start_of_packet(&datastream, 4));
    println!(
        "Part 2 solution: {:?}",
        get_start_of_packet(&datastream, 14)
    );
    println!("Time taken: {:?}", start_time.elapsed());
}

fn get_start_of_packet(datastream: &String, unique_chars: usize) -> usize {
    for i in 0..datastream.len() {
        let buffer = datastream[i..i + unique_chars]
            .chars()
            .collect::<HashSet<_>>();
        if buffer.len() == unique_chars {
            return i + unique_chars;
        }
    }
    return 0;
}
