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

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";
        assert_eq!(24000, part1(input));
    }

    #[test]
    fn test_part2() {
        let input = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";
        assert_eq!(45000, part2(input));
    }
}
