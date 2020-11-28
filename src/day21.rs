#[allow(unused_imports)]
use super::prelude::*;
use super::intcode::IntcodeDevice;
type Input = IntcodeDevice;

pub fn input_generator(input: &str) -> IntcodeDevice {
    input.parse().expect("Error parsing the IntcodeDevice")
}

pub fn part1(input: &Input) -> i64 {
    let mut device = input.clone();

    device.input.extend(
        "\
        NOT A J\n\
        NOT B T\n\
        OR T J\n\
        NOT C T\n\
        OR T J\n\
        AND D J\n\
        WALK\n\
        ".bytes().map(|b| b as i64) 
    );

    device.execute();

    match device.output.pop_back() {
        Some(answer) if answer > 255 => answer,
        Some(_) => panic!("Didn't make it across"),
        None => panic!("No output from device"),
    }
}

pub fn part2(input: &Input) -> i64 {
    let mut device = input.clone();

    device.input.extend(
        "\
        NOT A J\n\
        NOT B T\n\
        OR T J\n\
        NOT C T\n\
        OR T J\n\
        AND D J\n\
        NOT E T\n\
        NOT T T\n\
        OR H T\n\
        AND T J\n\
        RUN\n\
        ".bytes().map(|b| b as i64) 
    );

    device.execute();

    match device.output.pop_back() {
        Some(answer) if answer > 255 => answer,
        Some(_) => panic!("Didn't make it across"),
        None => panic!("No output from device"),
    }
}
