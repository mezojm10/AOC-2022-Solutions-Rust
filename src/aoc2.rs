use std::fs;

pub fn main() {
    let input = fs::read_to_string("aoc2.text").expect("Error reading file");

    println!("Day 2 Part 1 solution: {}", part1(&input));
    println!("Day 2 Part 2 solution: {}\n", part2(&input));
}

fn part1(input: &str) -> u32 {
    input.lines().fold(0, |score, round| {
        if let [p1, p2] = round.split_whitespace().collect::<Vec<_>>()[..] {
            let round_score = match (p1, p2) {
                ("A", "X") => 4,
                ("A", "Y") => 8,
                ("A", "Z") => 3,
                ("B", "X") => 1,
                ("B", "Y") => 5,
                ("B", "Z") => 9,
                ("C", "X") => 7,
                ("C", "Y") => 2,
                ("C", "Z") => 6,
                _ => 0,
            };
            score + round_score
        } else {
            score
        }
    })
}

fn part2(input: &str) -> u32 {
    input.lines().fold(0, |score, round| {
        if let [p1, p2] = round.split_whitespace().collect::<Vec<_>>()[..] {
            let round_score = match (p1, p2) {
                ("A", "X") => 3,
                ("A", "Y") => 4,
                ("A", "Z") => 8,
                ("B", "X") => 1,
                ("B", "Y") => 5,
                ("B", "Z") => 9,
                ("C", "X") => 2,
                ("C", "Y") => 6,
                ("C", "Z") => 7,
                _ => 0,
            };
            score + round_score
        } else {
            score
        }
    })
}
