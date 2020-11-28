#[allow(unused_imports)]
use super::prelude::*;
use super::intcode::IntcodeDevice;
type Input = IntcodeDevice;

pub fn input_generator(input: &str) -> Input {
    input.parse().expect("Error parsing the IntcodeDevice")
}

pub fn part1(input: &Input) -> usize {
    let mut device = input.clone();

    let mut blocks_count = 0;

    device.execute();
    while !device.output.is_empty() {
        let _ = device.output.pop_front().expect("No output from IntcodeDevice");
        let _ = device.output.pop_front().expect("No output from IntcodeDevice");
        match device.output.pop_front().expect("No output from IntcodeDevice") {
            0 /* Empty */ | 1 /* Wall */ | 3 /* Paddle */ | 4 /* Ball */ => {},
            2 /* Block */ => blocks_count += 1,
            _ => panic!("Invalid tile type")
        };
    }

    blocks_count
}

pub fn part2(input: &Input) -> i64 {
    let mut device = input.clone();
    device.memory[0] = 2;

    let mut blocks_count = 0;
    let mut first = true;

    let mut ball_x = 0;
    let mut paddle_x = 0;

    let mut score = 0;

    loop {
        device.execute();
        while !device.output.is_empty() {
            let x = device.output.pop_front().expect("No output from IntcodeDevice");
            let y = device.output.pop_front().expect("No output from IntcodeDevice");
            if x == -1 && y == 0 {
                score = device.output.pop_front().expect("No output from IntcodeDevice");
            } else {
                match device.output.pop_front().expect("No output from IntcodeDevice") {
                    0 /* Empty */ => if !first { blocks_count -= 1; },
                    1 /* Wall */ => {},
                    2 /* Block */ => if first { blocks_count += 1; },
                    3 /* Paddle */ => paddle_x = x,
                    4 /* Ball */ => {
                        if !first { blocks_count += 2; }
                        ball_x = x;
                    },
                    _ => panic!("Invalid tile type")
                }
            }
        }

        device.input.push_back(match ball_x.cmp(&paddle_x) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        });

        if blocks_count == 0 {
            return score
        }
        first = false;
    }
}
