// Advent of Code - Day 3: Lobby

const INPUT: &str = include_str!("./input.txt");

#[derive(Debug)]
struct BatteryBank {
    ratings: Vec<u8>,
}

impl BatteryBank {
    /// Create a `BatteryBank` from a string of joltage ratings.
    fn new(s: &str) -> Self {
        let ratings = s
            .trim()
            .chars()
            .map(|c| u8::try_from(c.to_digit(10).unwrap()).unwrap())
            .collect();
        Self { ratings }
    }

    fn max_joltage(&self, num_batteries: usize) -> u64 {
        // Push ratings to the stack, but remove smaller values (than current rating) while we have skips available
        let mut skips = self.ratings.len() - num_batteries;
        let mut stack: Vec<u8> = Vec::with_capacity(num_batteries);
        for &rating in &self.ratings {
            while let Some(&top) = stack.last() {
                if rating > top && skips > 0 {
                    stack.pop();
                    skips -= 1;
                } else {
                    break;
                }
            }
            stack.push(rating);
        }
        stack.truncate(num_batteries); // Ensure stack has correct length

        // convert stack into joltage value
        stack
            .iter()
            .fold(0u64, |acc, &rating| acc * 10 + u64::from(rating))
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = BatteryBank> {
    input.trim().lines().map(BatteryBank::new)
}

fn solve_part_one(input: &str) -> u64 {
    parse_input(input).map(|bank| bank.max_joltage(2)).sum()
}

fn solve_part_two(input: &str) -> u64 {
    parse_input(input).map(|bank| bank.max_joltage(12)).sum()
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
        let example_input = "987654321111111
811111111111119
234234234234278
818181911112111";
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 357);
    }

    #[test]
    fn part2() {
        let example_input = "987654321111111
811111111111119
234234234234278
818181911112111";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 3121910778619);
    }
}
