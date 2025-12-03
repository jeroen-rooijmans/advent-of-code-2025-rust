// Advent of Code - Day 2: Gift Shop

use std::ops::RangeInclusive;

const INPUT: &str = include_str!("./input.txt");

fn parse_input(input: &str) -> Vec<RangeInclusive<u64>> {
    input
        .trim()
        .split(',')
        .filter_map(|id_range| {
            let (start_str, end_str) = id_range.split_once('-')?;
            let start = start_str.parse().ok()?;
            let end = end_str.parse().ok()?;
            Some(start..=end)
        })
        .collect()
}

fn is_doubled_sequence(id: u64) -> bool {
    let sequence = id.to_string();
    let pattern_length = sequence.len() / 2;
    sequence.len().is_multiple_of(2) && sequence[..pattern_length] == sequence[pattern_length..]
}

fn is_repeated_sequence(id: u64) -> bool {
    let sequence = id.to_string();
    let sequence_length = sequence.len();

    (1..=sequence_length / 2)
        .filter(|&pattern_length| sequence_length.is_multiple_of(pattern_length))
        .any(|pattern_length| {
            let pattern = &sequence[..pattern_length];
            sequence
                .as_bytes()
                .chunks_exact(pattern_length)
                .all(|chunk| chunk == pattern.as_bytes())
        })
}

fn solve_part_one(input: &str) -> u64 {
    let ranges = parse_input(input);
    ranges
        .into_iter()
        .flatten()
        .filter(|&id| is_doubled_sequence(id))
        .sum()
}

fn solve_part_two(input: &str) -> u64 {
    let ranges = parse_input(input);
    ranges
        .into_iter()
        .flatten()
        .filter(|&id| is_repeated_sequence(id))
        .sum()
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
        let example_input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 1227775554);
    }

    #[test]
    fn part2() {
        let example_input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 4174379265);
    }
}
