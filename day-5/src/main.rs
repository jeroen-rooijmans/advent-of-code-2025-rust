// Advent of Code - Day 5: Cafeteria

use std::ops::RangeInclusive;

const INPUT: &str = include_str!("./input.txt");

fn parse_input(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let (raw_ranges, ingredients) = input.trim().split_once("\n\n").unwrap();
    let ranges: Vec<RangeInclusive<u64>> = raw_ranges
        .lines()
        .map(|l| {
            let (start_str, end_str) = l.split_once('-').unwrap();
            let start = start_str.parse().unwrap();
            let end = end_str.parse().unwrap();
            start..=end
        })
        .collect();
    (
        ranges,
        ingredients.lines().map(|l| l.parse().unwrap()).collect(),
    )
}

fn solve_part_one(input: &str) -> usize {
    let (ranges, ingredients) = parse_input(input);
    ingredients
        .iter()
        .filter(|&ingredient| ranges.iter().any(|range| range.contains(ingredient)))
        .count()
}

fn solve_part_two(input: &str) -> usize {
    let (mut ranges, _) = parse_input(input);
    ranges.sort_by(|a, b| a.start().cmp(b.start()));
    let first_range = ranges[0].clone();
    let merged_ranges: Vec<RangeInclusive<u64>> =
        ranges
            .into_iter()
            .fold(vec![first_range], |mut acc, range| {
                let prev_range = acc.last_mut().unwrap();
                if prev_range.end() >= range.start() {
                    let new_end = (*prev_range.end()).max(*range.end());
                    *prev_range = *prev_range.start()..=new_end;
                } else {
                    acc.push(range);
                }
                acc
            });
    merged_ranges.iter().map(|r| r.clone().count()).sum()
}

fn main() {
    let part_one_answer = solve_part_one(INPUT);
    println!("Part one:\n{part_one_answer}");
    let part_two_answer = solve_part_two(INPUT);
    println!("Part two:\n{part_two_answer}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let example_input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 3);
    }

    #[test]
    fn part2() {
        let example_input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 14);
    }
}
