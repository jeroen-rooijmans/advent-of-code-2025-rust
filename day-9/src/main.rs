// Advent of Code - Day 9: Movie Theater

use aoc::coord::Coordinate;

const INPUT: &str = include_str!("./input.txt");

fn parse_input(input: &str) -> Vec<Coordinate<isize>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_once(',')
                .map(|(x, y)| Coordinate::new(x.parse().unwrap(), y.parse().unwrap()))
                .unwrap()
        })
        .collect()
}

fn is_valid_rectangle(
    coord_a: &Coordinate<isize>,
    coord_b: &Coordinate<isize>,
    edges: &[(&Coordinate<isize>, &Coordinate<isize>)],
) -> bool {
    let x_min = coord_a.x.min(coord_b.x);
    let x_max = coord_a.x.max(coord_b.x);
    let y_min = coord_a.y.min(coord_b.y);
    let y_max = coord_a.y.max(coord_b.y);

    // zero width/height rectangles are not useful
    if x_min == x_max || y_min == y_max {
        return false;
    }

    for (poly_a, poly_b) in edges {
        if poly_a.x == poly_b.x {
            // vertical edge
            if poly_a.x > x_min && poly_a.x < x_max {
                // edge x position inside rectangle's width
                let overlap_min = poly_a.y.min(poly_b.y).max(y_min);
                let overlap_max = poly_a.y.max(poly_b.y).min(y_max);
                if overlap_min < overlap_max {
                    // overlap between edge and rectangle
                    return false;
                }
            }
        } else if poly_a.y == poly_b.y {
            // horizontal edge
            if poly_a.y > y_min && poly_a.y < y_max {
                // edge y position inside rectangle's width
                let overlap_min = poly_a.x.min(poly_b.x).max(x_min);
                let overlap_max = poly_a.x.max(poly_b.x).min(x_max);
                if overlap_min < overlap_max {
                    // overlap between edge and rectangle
                    return false;
                }
            }
        }
    }
    true
}

fn solve_part_one(input: &str) -> isize {
    let tiles = parse_input(input);
    let mut max_area = 0;
    for (i, coord_a) in tiles.iter().enumerate() {
        for coord_b in tiles.iter().skip(i + 1) {
            let dx = coord_a.x - coord_b.x;
            let dy = coord_a.y - coord_b.y;
            let area = (dx.abs() + 1) * (dy.abs() + 1);
            if area > max_area {
                max_area = area;
            }
        }
    }
    max_area
}

fn solve_part_two(input: &str) -> isize {
    let red_tiles = parse_input(input);
    let n = red_tiles.len();
    // create edges of polygon loop defined by red tiles
    let mut edges = Vec::new();
    for i in 0..n {
        edges.push((&red_tiles[i], &red_tiles[(i + 1) % n]));
    }

    let mut max_area = 0;
    for (i, coord_a) in red_tiles.iter().enumerate() {
        for coord_b in red_tiles.iter().skip(i + 1) {
            let dx = coord_a.x - coord_b.x;
            let dy = coord_a.y - coord_b.y;
            let area = (dx.abs() + 1) * (dy.abs() + 1);
            if area <= max_area {
                // skip rectangles smaller than current best
                continue;
            }
            if is_valid_rectangle(coord_a, coord_b, &edges) {
                max_area = area;
            }
        }
    }
    max_area
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
        let example_input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 50);
    }

    #[test]
    fn part2() {
        let example_input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 24);
    }
}
