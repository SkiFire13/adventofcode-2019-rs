#[allow(unused_imports)]
use super::prelude::*;
use super::intcode::IntcodeDevice;
type Input = IntcodeDevice;

pub fn input_generator(input: &str) -> Input {
    input.parse().expect("Error parsing the IntcodeDevice")
}

pub fn part1(input: &Input) -> u32 {
    let base_device = input;
    let mut device = input.clone();
    (0..50)
        .flat_map(|y| (0..50).map(move |x| (x,y)))
        .map(|(x,y)| is_attracted(&mut device, base_device, x, y) as u32)
        .sum()
}

pub fn part2(input: &Input) -> i64 {
    let base_device = input;
    let mut device = input.clone();
    let (mut x, mut y) = (0, 0);
    loop {
        if !is_attracted(&mut device, &base_device, x, y + 99) {
            x += 1;
            continue;
        }
        if !is_attracted(&mut device, &base_device, x + 99, y) {
            y += 1;
            continue;
        }
        return 10000 * x + y
    }
}

fn is_attracted(device: &mut IntcodeDevice, base_device: &IntcodeDevice, x: i64, y: i64) -> bool {
    device.reset(&base_device);
    device.input.push_back(x);
    device.input.push_back(y);
    device.execute();
    match device.output.pop_back().expect("Intcode device didn't report anything") {
        0 => false,
        1 => true,
        _ => panic!("Invalid report from intcode device")
    }
}
