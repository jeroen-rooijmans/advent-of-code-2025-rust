// Advent of Code - Day 10: Factory

const INPUT: &str = include_str!("./input.txt");

#[derive(Debug)]
struct Machine {
    // diagram of required state of machine to start, where light i corresponds the i-th bit
    // e.g. [.##.] -> 0110_2 -> 6
    diagram: usize,
    // button[button_idx] = [light_idx or counter_idx, ...]
    buttons: Vec<Vec<usize>>,
    // button represented by bitmask where a set bit means the button toggles that light
    // e.g. (1,3) -> 1010_2 -> 10
    button_masks: Vec<usize>,
    // number of lights == number of counters
    num_lights: usize,
    // joltage target values
    joltages: Vec<usize>,
    // captures the indices of buttons that increase a specific counter
    // counter_dependencies[counter_idx] = [button_idx, button_idx, ...]
    counter_dependencies: Vec<Vec<usize>>,
}

impl Machine {
    fn new(
        diagram: usize,
        button_masks: Vec<usize>,
        num_lights: usize,
        joltages: Vec<usize>,
    ) -> Self {
        let mut counter_dependencies = vec![Vec::new(); num_lights];
        let mut buttons = vec![Vec::new(); button_masks.len()];

        for (button_idx, &mask) in button_masks.iter().enumerate() {
            for counter_idx in 0..num_lights {
                if (mask & (1 << counter_idx)) != 0 {
                    buttons[button_idx].push(counter_idx);
                    counter_dependencies[counter_idx].push(button_idx);
                }
            }
        }
        Self {
            diagram,
            buttons,
            button_masks,
            num_lights,
            joltages,
            counter_dependencies,
        }
    }

    // Solves the system of linear equations: (Ax = t) over field F_2
    // field F_2 has two element {0, 1}, where addition is XOR and multiplication is AND.
    //
    // Each indicator light state is the sum of presses of all the buttons that affect it,
    // which can be simplified whether or not the button is pressed an odd or even number of times.
    //
    // Let N be the number of indicator lights, and M be the number of buttons.
    // Let x_j be the parity of the number of times button j is pressed (0: odd, 1: even).
    // let t_i be the target state for light i (0: off, 1: on)
    // let A_ij be the matrix representing the wiring schematics, where
    // A_ij = 1 if button j toggles light i, and A_ij = 0 otherwise.
    //
    // We need to find x1,x2...,xM such that for each light i:
    // A_i1_x1 XOR A_i2_x2 XOR ... XOR A_iM_xM = t_i (Ax = t)
    // This will give us all possible sets of parities x
    // For each parity vector x, we then calculate the minimum total presses by summing up x_j for j=1 to M.
    fn solve_part_one(&self) -> usize {
        // Create Augmented matrix, by augmenting A_ij with the target states
        // each row represents an equation for a light: [B_M-1, ..., B_0 | t]
        // t being the target state, and B_0 to B_M-1 are bits for each light.
        // The matrix is stored as a vector of ints
        let mut aug_matrix: Vec<usize> = Vec::with_capacity(self.num_lights);
        for i in 0..self.num_lights {
            let mut row_val = 0;
            if (self.diagram & (1 << i)) != 0 {
                row_val |= 1;
            }
            for (j, &button) in self.button_masks.iter().enumerate() {
                if (button & (1 << i)) != 0 {
                    row_val |= 1 << (j + 1); // button j is at bit j+1
                }
            }
            aug_matrix.push(row_val);
        }

        // Perform Gaussian elimination on augmented matrix
        let mut pivot = 0;
        for col in (1..=self.button_masks.len()).rev() {
            if pivot < self.num_lights {
                let mut i = pivot;
                // find row with '1' at current pivot column
                while i < self.num_lights && (aug_matrix[i] & (1 << col)) == 0 {
                    i += 1;
                }
                if i < self.num_lights {
                    // swap rows to bring pivot to current position
                    aug_matrix.swap(pivot, i);
                    // eliminate variable from all other rows
                    for j in 0..self.num_lights {
                        if j != pivot && (aug_matrix[j] & (1 << col)) != 0 {
                            aug_matrix[j] ^= aug_matrix[pivot]; // XOR for addition
                        }
                    }
                    pivot += 1;
                }
            }
        }

        // Find minimum weight solution
        let mut pivot_cols: Vec<usize> = Vec::new();
        let mut pivot_map: Vec<usize> = vec![usize::MAX; self.button_masks.len() + 1]; // map col -> row

        // only need to check up to last pivot
        for i in 0..pivot {
            // find first pivot
            for col in (1..=self.button_masks.len()).rev() {
                if (aug_matrix[i] & (1 << col)) != 0 {
                    if pivot_map[col] == usize::MAX {
                        pivot_map[col] = i;
                        pivot_cols.push(col);
                        break;
                    }
                }
            }
        }
        pivot_cols.sort();
        let free_cols: Vec<usize> = (1..=self.button_masks.len())
            .filter(|&col| !pivot_cols.contains(&col))
            .collect();

        let mut min_presses = usize::MAX;
        for i in 0..(1usize << free_cols.len()) {
            let mut current_presses = 0;
            let mut x: Vec<u8> = vec![0; self.button_masks.len() + 1];
            for (k, &col) in free_cols.iter().enumerate() {
                let val = (i >> k) & 1;
                x[col] = val as u8;
                current_presses += val;
            }

            for &col in pivot_cols.iter() {
                if let Some(&row) = pivot_map.get(col).filter(|&&r| r != usize::MAX) {
                    // x[col] + sum(A_row_j * x[j]) = t_row
                    let mut rhs = (aug_matrix[row] & 1) as u8;
                    for j in 1..self.button_masks.len() + 1 {
                        if j != col && (aug_matrix[row] & (1 << j)) != 0 {
                            rhs ^= x[j]
                        }
                    }
                    x[col] = rhs;
                    current_presses += rhs as usize;
                }
            }
            min_presses = min_presses.min(current_presses);
        }

        min_presses
    }

    fn solve_part_two(&self) -> usize {
        0
    }
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .trim()
        .lines()
        .map(|l| {
            let line = l.trim();
            // find delimiters
            let diagram_end = line.find(']').unwrap();
            let joltage_start = line.find('{').unwrap();
            // parse indicator light diagram
            let diagram_str = line.get(1..diagram_end).unwrap();
            let num_lights = diagram_str.len();
            let mut diagram = 0;
            for (i, c) in diagram_str.chars().enumerate() {
                if c == '#' {
                    diagram |= 1 << i; // set the i-th bit
                }
            }
            // parse button wiring schematics
            let button_str = line.get(diagram_end + 1..joltage_start).unwrap().trim();
            let mut button_masks = Vec::new();
            for schematic in button_str.split_whitespace() {
                let indices_str = schematic.trim_matches(|c| c == '(' || c == ')').trim();
                let mut button_mask = 0;
                if !indices_str.is_empty() {
                    for s in indices_str.split(',') {
                        if let Ok(i) = s.parse::<usize>() {
                            if i < num_lights {
                                button_mask |= 1 << i; // set the i-th bit
                            }
                        }
                    }
                }
                button_masks.push(button_mask);
            }
            // parse joltage requirements
            let joltage_str = line
                .get(joltage_start..)
                .unwrap()
                .trim_matches(|c| c == '{' || c == '}');
            let joltages: Vec<usize> = joltage_str
                .split(',')
                .filter_map(|s| s.parse::<usize>().ok())
                .collect();
            Machine::new(diagram, button_masks, num_lights, joltages)
        })
        .collect()
}

fn solve_part_one(input: &str) -> usize {
    let machines = parse_input(input);
    machines
        .iter()
        .map(|machine| machine.solve_part_one())
        .sum()
}

fn solve_part_two(input: &str) -> usize {
    let machines = parse_input(input);
    machines
        .iter()
        .map(|machine| machine.solve_part_two())
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
        let example_input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 7);
    }

    #[test]
    fn part2() {
        let example_input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 33);
    }
}
