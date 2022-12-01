use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let sum_of_calories = sum_of_calories(&content);
    part1(&sum_of_calories);
    part2(&sum_of_calories[(sum_of_calories.len() - 3)..])
}

fn sum_of_calories(content: &String) -> Vec<i32> {
    let mut calories: Vec<i32> = vec![];
    let mut current_calories = 0;
    for line in content.lines() {
        if line.is_empty() {
            calories.push(current_calories);
            current_calories = 0;
        } else {
            current_calories += line.parse::<i32>().unwrap();
        }
    }
    calories.sort();
    calories
}

fn part1(calories: &Vec<i32>) {
    println!("First part solution: {:?}", calories[calories.len() - 1]);
}

fn part2(calories: &[i32]) {
    let mut sum = 0;
    for calorie in calories {
        sum += calorie;
    }
    println!("Second part solution: {sum}");
}
