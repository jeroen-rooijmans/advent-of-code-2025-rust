// Advent of Code - Day 4: Printing Department
use aoc::coord::Coordinate;
use aoc::grid::Grid;

const INPUT: &str = include_str!("./input.txt");

fn parse_input(input: &str) -> Grid<char> {
    Grid::construct(input, &|c| c)
}

fn solve_part_one(input: &str) -> usize {
    let grid = parse_input(input);
    let mut accessable_rolls = 0;
    for r in 0..grid.num_rows {
        for c in 0..grid.num_columns {
            let coord = Coordinate { x: r, y: c };
            if grid.get(&coord) == Some('@')
                && grid
                    .surrounding(coord)
                    .iter()
                    .filter_map(|sur| sur.map(|(_, char)| *char))
                    .filter(|c| *c == '@')
                    .count()
                    < 4
            {
                accessable_rolls += 1;
            }
        }
    }
    accessable_rolls
}

fn solve_part_two(input: &str) -> usize {
    let mut grid = parse_input(input);
    let mut removed_rolls = 0;
    loop {
        let mut accessable_rolls = vec![];
        for r in 0..grid.num_rows {
            for c in 0..grid.num_columns {
                let coord = Coordinate { x: r, y: c };
                if grid.get(&coord) == Some('@')
                    && grid
                        .surrounding(coord)
                        .iter()
                        .filter_map(|sur| sur.map(|(_, char)| *char))
                        .filter(|c| *c == '@')
                        .count()
                        < 4
                {
                    accessable_rolls.push(coord);
                }
            }
        }
        if accessable_rolls.is_empty() {
            break;
        }
        for coord in &accessable_rolls {
            grid.set(*coord, 'x');
            removed_rolls += 1;
        }
    }
    removed_rolls
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
        let example_input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 13);
    }

    #[test]
    fn part2() {
        let example_input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 43);
    }
}
