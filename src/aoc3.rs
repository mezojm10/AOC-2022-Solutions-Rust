use std::fs;

pub fn main() {
    let input = fs::read_to_string("aoc3.text").expect("Error reading file");

    println!("Day 3 Part 1 solution: {}", part1(&input));
    println!("Day 3 Part 2 solution: {}\n", part2(&input));
}

fn part1(input: &str) -> u32 {
    input.lines().fold(0, |priority_sum, rucksack| {
        let letter_scores = ('a'..='z').chain('A'..='Z');
        let (comp1, comp2) = rucksack.split_at(rucksack.len() / 2);
        for chr in comp1.chars() {
            if comp2.contains(chr) {
                return letter_scores
                    .enumerate()
                    .find_map(|(idx, c)| {
                        if c == chr {
                            Some(priority_sum + idx as u32 + 1)
                        } else {
                            None
                        }
                    })
                    .unwrap();
            }
        }
        priority_sum
    })
}

fn part2(input: &str) -> u32 {
    let mut iter = input.lines();

    let mut priority_sum: u32 = 0;
    while let Some(elf1) = iter.next() {
        let elf2 = iter.next().unwrap();
        let elf3 = iter.next().unwrap();

        let letter_scores = ('a'..='z').chain('A'..='Z');
        for chr in elf1.chars() {
            if elf2.contains(chr) && elf3.contains(chr) {
                priority_sum += letter_scores
                    .enumerate()
                    .find_map(
                        |(idx, c)| {
                            if c == chr {
                                Some(idx as u32 + 1)
                            } else {
                                None
                            }
                        },
                    )
                    .unwrap();
                break;
            }
        }
    }
    priority_sum
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";
        assert_eq!(157, part1(input));
    }

    #[test]
    fn test_part2() {
        let input = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";
        assert_eq!(70, part2(input));
    }
}
