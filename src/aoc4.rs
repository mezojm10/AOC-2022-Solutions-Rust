use std::fs;

pub fn main() {
    let input = fs::read_to_string("aoc4.text").expect("Error reading file");

    println!("Day 4 Part 1 solution: {}", part1(&input));
    println!("Day 4 Part 2 solution: {}\n", part2(&input));
}

fn part1(input: &str) -> u32 {
    input.lines().fold(0, |acc, pair| {
        if let [range1, range2] = pair.split(',').collect::<Vec<_>>()[..] {
            let range1 = &range1
                .split('-')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>()[..];

            let range2 = &range2
                .split('-')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>()[..];

            let (small_range, big_range) = if range1[1] - range1[0] > range2[1] - range2[0] {
                (range2, range1)
            } else {
                (range1, range2)
            };

            if small_range[0] >= big_range[0] && small_range[1] <= big_range[1] {
                acc + 1
            } else {
                acc
            }
        } else {
            acc
        }
    })
}

fn part2(input: &str) -> u32 {
    input.lines().fold(0, |acc, pair| {
        if let [range1, range2] = pair.split(',').collect::<Vec<_>>()[..] {
            let range1 = &range1
                .split('-')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>()[..];

            let range2 = &range2
                .split('-')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>()[..];

            let (small_range, big_range) = if range1[1] - range1[0] > range2[1] - range2[0] {
                (range2, range1)
            } else {
                (range1, range2)
            };

            if (small_range[0] >= big_range[0] && small_range[0] <= big_range[1])
                || (small_range[1] >= big_range[0] && small_range[1] <= big_range[1])
            {
                acc + 1
            } else {
                acc
            }
        } else {
            acc
        }
    })
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";
        assert_eq!(2, part1(input));
    }

    #[test]
    fn test_part2() {
        let input = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";
        assert_eq!(4, part2(input));
    }
}
