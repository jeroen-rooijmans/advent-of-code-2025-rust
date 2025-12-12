// Advent of Code - Day 12: Christmas Tree Farm

use std::collections::HashSet;
use std::error::Error;
use std::fmt;

const INPUT: &str = include_str!("./input.txt");
const SIZE: usize = 3;

#[derive(Debug)]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid shape string format.")
    }
}
impl Error for ParseError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GridCell {
    Empty,
    Covered(usize),
}

#[derive(Debug, Clone)]
struct Region {
    width: usize,
    height: usize,
    wishlist: Vec<usize>,
    grid: Vec<Vec<GridCell>>,
}

impl Region {
    fn new(width: usize, height: usize, wishlist: Vec<usize>) -> Self {
        let grid = vec![vec![GridCell::Empty; width]; height];
        Self {
            width,
            height,
            wishlist,
            grid,
        }
    }

    /// Checks if a `Present` can be placed at without overlap.
    /// Topleft of the present 3x3 shape will be placed at (r, c) on the grid.
    fn can_place(&self, present: &Present, row_idx: usize, col_idx: usize) -> bool {
        // check bounds
        if row_idx + SIZE > self.height || col_idx + SIZE > self.width {
            return false;
        }
        // check overlap: if shape contains '#' and grid is not empty
        let shape = present.get_shape();
        for (dr, row) in shape.iter().enumerate() {
            for (dc, &cell) in row.iter().enumerate() {
                if cell && self.grid[row_idx + dr][col_idx + dc] != GridCell::Empty {
                    return false;
                }
            }
        }

        true
    }

    fn place_mut(&mut self, present: &Present, row_idx: usize, col_idx: usize) {
        let shape = present.get_shape();
        for (dr, row) in shape.iter().enumerate() {
            for (dc, &cell) in row.iter().enumerate() {
                if cell {
                    self.grid[row_idx + dr][col_idx + dc] = GridCell::Covered(present.id);
                }
            }
        }
    }

    fn unplace_mut(&mut self, present: &Present, row_idx: usize, col_idx: usize) {
        let shape = present.get_shape();
        for (dr, row) in shape.iter().enumerate() {
            for (dc, &cell) in row.iter().enumerate() {
                if cell {
                    self.grid[row_idx + dr][col_idx + dc] = GridCell::Empty;
                }
            }
        }
    }

    fn count_empty_cells(&self) -> usize {
        self.grid
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&cell| cell == GridCell::Empty)
            .count()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Present {
    shape: [[bool; SIZE]; SIZE],
    id: usize,
}

impl TryFrom<&str> for Present {
    type Error = ParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut shape = [[false; SIZE]; SIZE];
        let mut row_idx = 0;
        let mut lines = s.trim().lines().peekable(); // Use peekable to check for extra lines

        // Grab id
        let id: usize = lines
            .next()
            .unwrap()
            .trim()
            .split_once(':')
            .unwrap()
            .0
            .parse()
            .unwrap();

        // Ensure we don't process more than SIZE lines
        while row_idx < SIZE {
            match lines.next() {
                Some(line) => {
                    let line = line.trim();
                    if line.len() != SIZE {
                        return Err(ParseError);
                    }

                    for (col_idx, char) in line.chars().enumerate() {
                        match char {
                            '#' => shape[row_idx][col_idx] = true,
                            '.' => shape[row_idx][col_idx] = false,
                            _ => return Err(ParseError),
                        }
                    }
                    row_idx += 1;
                }
                None => break, // Ran out of lines
            }
        }

        // Must have exactly SIZE rows and no more lines left
        if row_idx != SIZE || lines.peek().is_some() {
            return Err(ParseError);
        }

        Ok(Present::new(shape, id))
    }
}

impl Present {
    fn new(shape: [[bool; SIZE]; SIZE], id: usize) -> Self {
        Present { shape, id }
    }

    fn get_shape(&self) -> &[[bool; SIZE]; SIZE] {
        &self.shape
    }

    // Returns shape rotated 90 degrees in clockwise direction.
    fn rotate_cw(&self) -> Self {
        let s = self.shape;
        let new_shape = [
            [s[2][0], s[1][0], s[0][0]],
            [s[2][1], s[1][1], s[0][1]],
            [s[2][2], s[1][2], s[0][2]],
        ];
        Present::new(new_shape, self.id)
    }

    // Returns shape flipped along vertical axis.
    fn flip_hor(&self) -> Self {
        let s = self.shape;
        let new_shape = [
            [s[0][2], s[0][1], s[0][0]],
            [s[1][2], s[1][1], s[1][0]],
            [s[2][2], s[2][1], s[2][0]],
        ];
        Present::new(new_shape, self.id)
    }

    /// Returns a set of all unique orientations of a `Present`.
    pub fn all_orientations(&self) -> HashSet<Present> {
        let mut orientations = HashSet::new(); // HashSet handles uniqueness
        // generate 4 rotations of present
        let mut present = *self;
        for _ in 0..4 {
            orientations.insert(present);
            present = present.rotate_cw();
        }
        // generate 4 rotations of horizontally flipped present
        present = self.flip_hor();
        for _ in 0..4 {
            orientations.insert(present);
            present = present.rotate_cw();
        }

        orientations
    }
}

fn parse_input(input: &str) -> (Vec<Present>, Vec<Region>) {
    let mut presents = vec![];
    let blocks: Vec<&str> = input.trim().split("\n\n").collect();
    for present_str in &blocks[..blocks.len() - 1] {
        presents.push(Present::try_from(*present_str).unwrap());
    }
    let mut regions = vec![];
    for region_str in blocks.last().unwrap().trim().lines() {
        if region_str.is_empty() {
            continue;
        }
        let (size_str, wishlist_str) = region_str.split_once(": ").unwrap();
        let (width, height) = size_str
            .split_once('x')
            .map(|(width_str, height_str)| {
                (width_str.parse().unwrap(), height_str.parse().unwrap())
            })
            .unwrap();
        let wishlist = wishlist_str
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        regions.push(Region::new(width, height, wishlist));
    }
    (presents, regions)
}

fn count_cells_needed(present_ids: &[usize], all_orientations: &[Vec<Present>]) -> usize {
    present_ids
        .iter()
        .map(|&id| {
            all_orientations[id]
                .first()
                .unwrap()
                .get_shape()
                .iter()
                .flat_map(|row| row.iter())
                .filter(|&&cell| cell)
                .count()
        })
        .sum()
}

fn try_fit_presents(
    region: &mut Region,
    present_ids: &mut Vec<usize>,
    all_orientations: &[Vec<Present>],
) -> bool {
    if present_ids.is_empty() {
        return true;
    }

    // Early pruning: check if we have enough empty cells
    let cells_needed = count_cells_needed(present_ids, all_orientations);
    if region.count_empty_cells() < cells_needed {
        return false;
    }

    let id_to_place = present_ids.pop().unwrap();
    let orientations = &all_orientations[id_to_place];

    for r in 0..=region.height.saturating_sub(SIZE) {
        for c in 0..=region.width.saturating_sub(SIZE) {
            for orientation in orientations {
                if region.can_place(orientation, r, c) {
                    region.place_mut(orientation, r, c);

                    if try_fit_presents(region, present_ids, all_orientations) {
                        return true;
                    }

                    region.unplace_mut(orientation, r, c);
                }
            }
        }
    }

    present_ids.push(id_to_place);
    false
}

fn solve(input: &str) -> usize {
    let (presents, regions) = parse_input(input);
    let all_orientations: Vec<Vec<Present>> = presents
        .iter()
        .map(|p| p.all_orientations().into_iter().collect())
        .collect();

    let mut count = 0;
    for region in regions {
        let mut region = region.clone();
        let mut present_ids: Vec<usize> = region
            .wishlist
            .iter()
            .enumerate()
            .flat_map(|(id, count)| std::iter::repeat_n(id, *count))
            .collect();

        let mut freq_map = std::collections::HashMap::new();
        for &id in &present_ids {
            *freq_map.entry(id).or_insert(0) += 1;
        }
        // Sort by frequency (most common first)
        present_ids.sort_by_key(|&id| std::cmp::Reverse(freq_map[&id]));

        if try_fit_presents(&mut region, &mut present_ids, &all_orientations) {
            count += 1;
        }
    }
    count
}

fn main() {
    let answer = solve(INPUT);
    println!("Answer:\n{answer}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let example_input = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";
        let answer = crate::solve(example_input);
        assert_eq!(answer, 2);
    }
}
