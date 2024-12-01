use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> (Vec<u32>, Vec<u32>) {
    let nr_lines = input.lines().count();
    let mut numbers1 = Vec::<u32>::with_capacity(nr_lines);
    let mut numbers2 = Vec::<u32>::with_capacity(nr_lines);

    for line in input.lines() {
        let mut line_iter = line.split_ascii_whitespace();
        numbers1.push(line_iter.next().unwrap().parse::<u32>().unwrap());
        numbers2.push(line_iter.next().unwrap().parse::<u32>().unwrap());
    }
    (numbers1, numbers2)
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    let mut numbers1 = input.0.clone();
    let mut numbers2 = input.1.clone();
    assert_eq!(numbers1.len(), numbers1.len());
    numbers1.sort();
    numbers2.sort();
    let mut total = 0;
    for (n1, n2) in numbers1.iter().zip(numbers2.iter()) {
        total += n1.abs_diff(*n2);
    }
    total
}

fn hist(inp: &[u32]) -> HashMap<u32, u32> {
    let hm = HashMap::<u32, u32>::with_capacity(inp.len());
    inp.iter().fold(hm, |mut acc, x| {
        acc.entry(*x).and_modify(|v| *v += 1).or_insert(1);
        acc
    })
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    let numbers1 = hist(&input.0);
    let numbers2 = hist(&input.1);
    let mut total = 0;
    for (nr, cnt) in numbers1 {
        total += nr * numbers2.get(&nr).unwrap_or(&0) * cnt;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let ex_input = r"3   4
4   3
2   5
1   3
3   9
3   3";
        let input_parsed = input_generator(ex_input);
        assert_eq!(solve_part1(&input_parsed), 11);
    }

    #[test]
    fn part2_example() {
        let ex_input = r"3   4
4   3
2   5
1   3
3   9
3   3";
        let input_parsed = input_generator(ex_input);
        assert_eq!(solve_part2(&input_parsed), 31);
    }
}
