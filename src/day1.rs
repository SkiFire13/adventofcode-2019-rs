#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<u32>;

pub fn input_generator(input: &str) -> Input {
    input.lines()
        .map(|l| l.parse::<u32>().expect("Input line is not a positive integer"))
        .collect()
}

pub fn part1(input: &Input) -> u32 {
    input.iter().map(|mass| (mass / 3).saturating_sub(2)).sum()
}

pub fn part2(input: &Input) -> u32 {
    input.iter().map(|&mass|
        std::iter::successors(Some(mass), |&mass| (mass / 3).checked_sub(2))
            .skip(1)
            .sum::<u32>()
    ).sum()
}
