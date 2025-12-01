// Advent of Code - Day 1: Secret Entrance

const INPUT: &str = include_str!("./input.txt");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
struct Rotation {
    direction: Direction,
    amount: u32,
}

struct Dial {
    position: u32, // always 0-99
}

impl Dial {
    const POSITIONS: u32 = 100;

    fn new(start: u32) -> Self {
        Self {
            position: start % Self::POSITIONS,
        }
    }

    fn calculate_new_position(&self, rotation: Rotation) -> u32 {
        let delta = rotation.amount % Self::POSITIONS;
        match rotation.direction {
            Direction::Left => (self.position + Self::POSITIONS - delta) % Self::POSITIONS,
            Direction::Right => (self.position + delta) % Self::POSITIONS,
        }
    }

    fn turn(&mut self, rotation: Rotation) {
        self.position = self.calculate_new_position(rotation);
    }

    fn count_zero_crossings(&self, rotation: Rotation) -> u32 {
        let delta = rotation.amount % Self::POSITIONS;

        let passes_zero = match rotation.direction {
            Direction::Left => self.position != 0 && delta > self.position,
            Direction::Right => self.position + delta > Self::POSITIONS,
        };

        rotation.amount / Self::POSITIONS + u32::from(passes_zero || self.calculate_new_position(rotation) == 0)
    }
}

fn parse_input(input: &str) -> Vec<Rotation> {
    input
        .lines()
        .filter_map(|line| {
            let (dir, amount) = line.split_at(1);
            let amount: u32 = amount.parse().ok()?;
            let direction = match dir {
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => unreachable!("Only 'L' or 'R' expected"),
            };
            Some(Rotation { direction, amount })
        })
        .collect()
}

fn solve_part_one(input: &str) -> u32 {
    let instructions = parse_input(input);
    let mut dial = Dial::new(50);
    instructions
        .iter()
        .filter(|&turn| {
            dial.turn(*turn);
            dial.position == 0
        })
        .count().try_into().unwrap()
}

fn solve_part_two(input: &str) -> u32 {
    let instructions = parse_input(input);
    let mut dial = Dial::new(50);

    instructions
        .iter()
        .map(|&turn| {
            let count = dial.count_zero_crossings(turn);
            dial.turn(turn);
            count
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
        let example_input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 3);
    }

    #[test]
    fn part1_large_rotations() {
        let example_input = "L68
L30
R48
L5
R60
L55
R542
L42
L1
L99
R14
L101
L82
R1";
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 4);
    }

    #[test]
    fn part2() {
        let example_input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 6);
    }

    #[test]
    fn part2_large_rotations() {
        let example_input = "L68
L30
R48
L5
R60
L55
R542
L1
L99
R14
L101
L82
R1";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 12);
    }
}
