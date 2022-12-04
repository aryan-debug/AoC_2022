use std::fs;
use std::time::Instant;

struct Elf {
    start: i32,
    end: i32,
}

impl Elf {
    fn new(section: String) -> Elf {
        let mut range = section.split("-");
        Elf {
            start: range.next().unwrap().parse().unwrap(),
            end: range.next().unwrap().parse().unwrap(),
        }
    }

    fn subset(&self, other: &Elf) -> bool {
        self.start >= other.start && self.end <= other.end
            || other.start >= self.start && other.end <= self.end
    }

    fn intersect(&self, other: &Elf) -> bool {
        !((self.start < other.start && self.start < other.end)
            && (self.end < other.start && self.end < other.end))
    }
}

fn main() {
    let start = Instant::now();
    let section_assignments = fs::read_to_string("input.txt").expect("Error opening the file.");
    let (subsets, intersections) = find_subset_and_intersections(section_assignments);
    println!("Part 1 solution: {subsets}");
    println!("Part 2 solution: {intersections}");
    println!("Time taken: {:?}", start.elapsed());
}

fn find_subset_and_intersections(section_assignments: String) -> (i32, i32) {
    let mut subsets = 0;
    let mut intersections = 0;
    for section in section_assignments.lines() {
        let mut sections = section.split(",");
        let first_elf = Elf::new(sections.next().unwrap().to_string());
        let second_elf = Elf::new(sections.next().unwrap().to_string());

        if first_elf.subset(&second_elf) {
            subsets += 1;
        }
        if first_elf.intersect(&second_elf) && second_elf.intersect(&first_elf) {
            intersections += 1;
        }
    }
    (subsets, intersections)
}
