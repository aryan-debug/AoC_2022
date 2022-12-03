use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let content = fs::read_to_string("input.txt").expect("Error opening file");
    let part1_scores = HashMap::from([
        ("A X", 4),
        ("A Y", 8),
        ("A Z", 3),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 7),
        ("C Y", 2),
        ("C Z", 6),
    ]);
    let part2_scores = HashMap::from([
        ("A X", 3),
        ("A Y", 4),
        ("A Z", 8),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 2),
        ("C Y", 6),
        ("C Z", 7),
    ]);
    let rounds = content.lines().into_iter();
    let (part1_score, part2_score): (i32, i32) = rounds
        .map(|round| {
            (
                *part1_scores.get(round).unwrap(),
                *part2_scores.get(round).unwrap(),
            )
        })
        .fold((0, 0), |(part1_score, part2_score), (x, y)| {
            (part1_score + x, part2_score + y)
        });
    println!("{} {}", part1_score, part2_score);
    println!("{:?}", start.elapsed());
}
