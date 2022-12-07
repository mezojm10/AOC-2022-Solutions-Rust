use std::fs;

pub fn main() {
    let input = fs::read_to_string("aoc1.text").expect("Error reading file");

    println!("Day 1 Part 1 solution: {}", part1(&input));
    println!("Day 1 Part 2 solution: {}\n", part2(&input));
}

fn part1(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|num| num.parse::<u32>().unwrap_or_default())
                .sum::<u32>()
        })
        .max()
        .unwrap()
}

fn part2(input: &str) -> u32 {
    let mut calories: Vec<_> = input
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|num| num.parse::<u32>().unwrap_or_default())
                .sum::<u32>()
        })
        .collect();
    calories.sort();
    calories.iter().rev().take(3).sum::<u32>()
}
