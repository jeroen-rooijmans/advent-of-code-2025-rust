// Advent of Code - Day 11: Reactor

use std::collections::HashMap;

type Connections = HashMap<String, Vec<String>>;
type CountCache = HashMap<String, usize>;

const INPUT: &str = include_str!("./input.txt");

fn parse_input(input: &str) -> Connections {
    input
        .trim()
        .lines()
        .filter_map(|line| {
            let (from, to) = line.split_once(": ").unwrap();
            Some((
                from.to_string(),
                to.trim()
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect(),
            ))
        })
        .collect()
}

/// Count number of paths recursively from `from` node to `to` node.
fn count_paths(from: &str, to: &str, connections: &Connections, cache: &mut CountCache) -> usize {
    // check if we're at `to` node
    if from == to {
        return 1;
    }

    // check if we've been here before
    if let Some(&count) = cache.get(from) {
        return count;
    }

    // recursively follow connections
    let mut total_paths = 0;
    if let Some(outputs) = connections.get(from) {
        for next_device in outputs {
            total_paths += count_paths(next_device, to, connections, cache)
        }
    }

    // add to cache
    cache.insert(from.to_string(), total_paths);

    total_paths
}

fn solve_part_one(input: &str) -> usize {
    let connections = parse_input(input);
    let start_node = "you";
    let end_node = "out";
    let mut cache = CountCache::new();
    count_paths(start_node, end_node, &connections, &mut cache)
}

fn solve_part_two(input: &str) -> usize {
    let connections = parse_input(input);
    let start_node = "svr";
    let dac_node = "dac";
    let fft_node = "fft";
    let end_node = "out";

    // option 1: 'svr' -> 'dac' -> 'fft' -> 'out'
    let mut cache = CountCache::new();
    let svr_to_dac = count_paths(start_node, dac_node, &connections, &mut cache);
    let mut cache = CountCache::new();
    let dac_to_fft = count_paths(dac_node, fft_node, &connections, &mut cache);
    let mut cache = CountCache::new();
    let fft_to_out = count_paths(fft_node, end_node, &connections, &mut cache);

    // option 2: 'svr' -> 'fft' -> 'dac' -> 'out'
    let mut cache = CountCache::new();
    let svr_to_fft = count_paths(start_node, fft_node, &connections, &mut cache);
    let mut cache = CountCache::new();
    let fft_to_dac = count_paths(fft_node, dac_node, &connections, &mut cache);
    let mut cache = CountCache::new();
    let dac_to_out = count_paths(dac_node, end_node, &connections, &mut cache);

    (svr_to_dac * dac_to_fft * fft_to_out) + (svr_to_fft * fft_to_dac * dac_to_out)
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
        let example_input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 5);
    }

    #[test]
    fn part2() {
        let example_input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 2);
    }
}
