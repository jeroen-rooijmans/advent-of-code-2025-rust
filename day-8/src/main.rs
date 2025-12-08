// Advent of Code - Day 8: Playground

const INPUT: &str = include_str!("./input.txt");

struct JunctionBox {
    x: isize,
    y: isize,
    z: isize,
}

impl JunctionBox {
    fn euclidean_distance_sq(&self, other: &JunctionBox) -> usize {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx.pow(2) + dy.pow(2) + dz.pow(2)) as usize
    }
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, idx: usize) -> usize {
        // find root idx
        let mut root_idx = idx;
        while root_idx != self.parent[root_idx] {
            root_idx = self.parent[root_idx];
        }
        // path compression
        let mut i = idx;
        while i != root_idx {
            let parent_idx = self.parent[i];
            self.parent[parent_idx] = root_idx;
            i = parent_idx;
        }
        root_idx
    }

    fn union(&mut self, idx_a: usize, idx_b: usize) -> bool {
        let root_a = self.find(idx_a);
        let root_b = self.find(idx_b);
        if root_a == root_b {
            // already in same set
            return false;
        }
        // merge smaller set into larger one
        if self.size[root_a] < self.size[root_b] {
            self.parent[root_a] = root_b;
            self.size[root_b] += self.size[root_a];
        } else {
            self.parent[root_b] = root_a;
            self.size[root_a] += self.size[root_b];
        }
        true
    }

    fn get_sizes(&mut self) -> Vec<usize> {
        let mut sizes = Vec::new();
        for i in 0..self.parent.len() {
            let root_idx = self.find(i);
            if root_idx == i {
                sizes.push(self.size[root_idx]);
            }
        }
        sizes
    }
}

fn parse_input(input: &str) -> Vec<JunctionBox> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split(',').map(|s| s.parse().unwrap());
            JunctionBox {
                x: parts.next().unwrap(),
                y: parts.next().unwrap(),
                z: parts.next().unwrap(),
            }
        })
        .collect()
}

fn solve_part_one(input: &str, num_connections: usize) -> usize {
    let junction_boxes = parse_input(input);
    let mut union_find = UnionFind::new(junction_boxes.len());
    let mut distance_pairs: Vec<(usize, usize, usize)> = junction_boxes
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            junction_boxes
                .iter()
                .enumerate()
                .skip(i + 1)
                .map(move |(j, b)| {
                    let dist_sq = a.euclidean_distance_sq(b);
                    (i, j, dist_sq)
                })
        })
        .collect();
    distance_pairs.sort_by_key(|&(_, _, distance)| distance);

    // Build Union-Find data structure
    for &(i, j, _distance) in distance_pairs.iter().take(num_connections) {
        union_find.union(i, j);
    }
    let mut circuit_sizes = union_find.get_sizes();
    circuit_sizes.sort_by(|a, b| b.cmp(a));

    circuit_sizes.iter().take(3).product()
}

fn solve_part_two(input: &str) -> isize {
    let junction_boxes = parse_input(input);
    let mut union_find = UnionFind::new(junction_boxes.len());
    let mut distance_pairs: Vec<(usize, usize, usize)> = junction_boxes
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            junction_boxes
                .iter()
                .enumerate()
                .skip(i + 1)
                .map(move |(j, b)| {
                    let dist_sq = a.euclidean_distance_sq(b);
                    (i, j, dist_sq)
                })
        })
        .collect();
    distance_pairs.sort_by_key(|&(_, _, distance)| distance);

    let mut last_connection_idxs: Option<(usize, usize)> = None;
    // Build Union-Find data structure until all junction boxes are in a single circuit
    for &(i, j, _distance) in &distance_pairs {
        if union_find.union(i, j) {
            last_connection_idxs = Some((i, j));
            if union_find.get_sizes().len() == 1 {
                break;
            }
        }
    }
    if let Some((idx_a, idx_b)) = last_connection_idxs {
        junction_boxes[idx_a].x * junction_boxes[idx_b].x
    } else {
        panic!("Could not connect all junction boxes!");
    }
}

fn main() {
    let part_one_answer = solve_part_one(INPUT, 1000);
    println!("Part one:\n{part_one_answer}");
    let part_two_answer = solve_part_two(INPUT);
    println!("Part two:\n{part_two_answer}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let example_input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        let answer = crate::solve_part_one(example_input, 10);
        assert_eq!(answer, 40);
    }

    #[test]
    fn part2() {
        let example_input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 25272);
    }
}
