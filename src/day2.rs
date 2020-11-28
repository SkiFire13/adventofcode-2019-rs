#[allow(unused_imports)]
use super::prelude::*;
use super::intcode::IntcodeDevice;
type Input = IntcodeDevice;

pub fn input_generator(input: &str) -> Input {
    input.parse().expect("Error parsing the IntcodeDevice")
}

pub fn part1(input: &Input) -> i64 {
    let mut device = input.clone();
    device.memory[1] = 12;
    device.memory[2] = 2;
    device.execute();
    device.memory[0]
}

pub fn part2(input: &Input) -> i64 {
    let device = input;
    for noun in 0..99 {
        for verb in 0..99 {
            let mut device = device.clone();
            device.memory[1] = noun;
            device.memory[2] = verb;
            device.execute();
            if device.memory[0] == 19_690_720 {
                return 100 * noun + verb;
            }
        }
    }
    panic!("Cannot find valid noun-verb pair");
}
