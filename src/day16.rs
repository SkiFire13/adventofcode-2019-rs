#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<u32>;

pub fn input_generator(input: &str) -> Input {
    input.chars()
        .map(|c| c.to_digit(10).expect("Invalid character"))
        .collect()
}

pub fn part1(input: &Input) -> u32 {
    let mut input = input.clone();
    let mut temp_vec = Vec::new();

    for _ in 0..100 {
        temp_vec.clear();
        temp_vec.extend(
            (1..=input.len()).map(|i|
                input.iter()
                    .zip(pattern_iter(i).skip(1))
                    .map(|(d, p)| (*d as i32) * p)
                    .sum::<i32>().abs() as u32
                    % 10
            )
        );
        std::mem::swap(&mut input, &mut temp_vec);
    }

    input[..8].iter().fold(0, |sum, n| 10 * sum + n)
}

pub fn part2(input: &Input) -> u32 {
    let message_offset = input[..7].iter().fold(0, |sum, n| 10 * sum + n) as usize;
    if 2 * message_offset - 1 < input.len() { panic!("This is impossible in human time"); }

    let mut numbers = input.iter().cloned()
        .cycle()
        .skip(message_offset % input.len())
        .take(10000 * input.len() - message_offset)
        .collect::<Vec<_>>();

    for _ in 0..100 {
        for i in (0..numbers.len()-1).rev() {
            numbers[i] = (numbers[i + 1] + numbers[i]) % 10;
        }
    }

    numbers[..8].iter().fold(0, |sum, n| 10 * sum + n)
}

fn pattern_iter(n: usize) -> impl Iterator<Item = i32> {
    [0, 1, 0, -1].iter()
        .flat_map(move |d| std::iter::repeat(d).take(n))
        .cloned()
        .cycle()
}
