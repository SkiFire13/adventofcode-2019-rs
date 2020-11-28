#[allow(unused_imports)]
use super::prelude::*;
use super::intcode::IntcodeDevice;
type Input = IntcodeDevice;

pub fn input_generator(input: &str) -> Input {
    input.parse().expect("Error parsing the IntcodeDevice")
}

pub fn part1(input: &Input) -> i64 {
    let mut device: IntcodeDevice = input.clone();
    device.input.push_back(1);
    device.execute();
    if device.output.iter().rev().skip(1).all(|&i| i == 0) {
        return device.output.pop_back().expect("No output from device")
    }

    panic!("Some checks failed: {:?}", device.output)
}

pub fn part2(input: &Input) -> i64 {
    let mut device: IntcodeDevice = input.clone();
    device.input.push_back(5);
    device.execute();
    if device.output.iter().rev().skip(1).all(|&i| i == 0) {
        return device.output.pop_back().expect("No output from the IntcodeDevice")
    }

    panic!("Some checks failed: {:?}", device.output)
}