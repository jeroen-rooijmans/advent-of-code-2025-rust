// Advent of Code - Day 6: Trash Compactor

use std::iter::IntoIterator;

const INPUT: &str = include_str!("./input.txt");

fn transpose2d<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(IntoIterator::into_iter).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn solve_part_one(input: &str) -> usize {
    let lines: Vec<&str> = input.trim().lines().collect();
    let (operator_line, problems) = lines.split_last().unwrap();
    let operators: Vec<char> = operator_line
        .split_whitespace()
        .flat_map(|s| s.chars())
        .collect();
    let mut numbers: Vec<Vec<usize>> = problems
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    numbers = transpose2d(numbers);
    numbers
        .into_iter()
        .zip(operators)
        .map(|(problem, operator)| match operator {
            '+' => problem.iter().sum(),
            '*' => problem.iter().product::<usize>(),
            _ => panic!("Invalid operator encountered"),
        })
        .sum()
}

fn solve_part_two(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let (operator_line, problems) = lines.split_last().unwrap();
    let operators: Vec<char> = operator_line
        .split_whitespace()
        .flat_map(|s| s.chars())
        .collect();
    let mut chars: Vec<Vec<char>> = problems.iter().map(|line| line.chars().collect()).collect();
    chars = transpose2d(chars);
    let numbers: Vec<Vec<usize>> = chars
        .into_iter()
        .map(|col| col.into_iter().filter(|&c| c != ' ').collect())
        .collect::<Vec<_>>()
        .split(String::is_empty)
        .map(|group| {
            group
                .iter()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();
    numbers
        .into_iter()
        .zip(operators)
        .map(|(problem, operator)| match operator {
            '+' => problem.iter().sum(),
            '*' => problem.iter().product::<usize>(),
            _ => panic!("Invalid operator encountered"),
        })
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
        let example_input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n *   +   *   +  ";
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 4277556);
    }

    #[test]
    fn part2() {
        let example_input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n *   +   *   +  ";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 3263827);
    }
}
