// Advent of Code - Day 7: Laboratories

use std::collections::{HashMap, HashSet, VecDeque};

use aoc::coord::Coordinate;
use aoc::direction::Direction;
use aoc::grid::Grid;

const INPUT: &str = include_str!("./input.txt");

/// Finds position(s) of a tachyon beam after taking a single step in time
fn beam_step(grid: &Grid<char>, position: Coordinate<usize>) -> Vec<Coordinate<usize>> {
    let mut next_positions = Vec::new();
    if let Some(position_south) = grid.step(position, Direction::South, 1)
        && let Some(next_space) = grid.get(&position_south) {
            if next_space == '.' {
                // Tachyon beams pass freely through empty space (.)
                next_positions.push(position_south);
            } else {
                // Tachyon beam encounters a splitter (^), a new tachyon beam continues from the
                //  immediate left and from the immediate right of the splitter
                if let Some(sw_position) = grid.step(position, Direction::SouthWest, 1) {
                    next_positions.push(sw_position);
                }
                if let Some(se_position) = grid.step(position, Direction::SouthEast, 1) {
                    next_positions.push(se_position);
                }
            }
        }

    next_positions
}

fn solve_part_one(input: &str) -> usize {
    let grid = Grid::construct(input, &|c| c);
    let starting_position = *grid.search(&'S').first().unwrap();
    let mut tachyon_beam_splits = 0;
    let mut queue = VecDeque::from([starting_position]);
    let mut visited = HashSet::from([starting_position]);
    while let Some(position) = queue.pop_front() {
        let encountered_splitter = grid
            .step(position, Direction::South, 1)
            .and_then(|pos| grid.get(&pos))
            .is_some_and(|space| space == '^');
        if encountered_splitter {
            tachyon_beam_splits += 1;
        }

        for next_position in beam_step(&grid, position) {
            if visited.insert(next_position) {
                queue.push_back(next_position);
            }
        }
    }

    tachyon_beam_splits
}

fn solve_part_two(input: &str) -> usize {
    let grid = Grid::construct(input, &|c| c);
    let starting_position = *grid.search(&'S').first().unwrap();
    // stores how many timelines visit each coordinate
    let mut timeline_counts = HashMap::from([(starting_position, 1)]);

    for y in 0..grid.num_rows - 1 {
        for x in 0..grid.num_columns {
            let position = Coordinate { x, y };

            let Some(&count) = timeline_counts.get(&position) else {
                // No tachyon beam in any timeline ever visits this position
                continue;
            };

            // All timelines flow to the next positions
            for next_position in beam_step(&grid, position) {
                *timeline_counts.entry(next_position).or_insert(0) += count;
            }
        }
    }

    // sum number of timelines that reached bottom of grid
    timeline_counts
        .iter()
        .filter_map(|(coord, &count)| {
            if coord.y == grid.num_rows - 1 {
                Some(count)
            } else {
                None
            }
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
        let example_input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 21);
    }

    #[test]
    fn part2() {
        let example_input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 40);
    }
}
