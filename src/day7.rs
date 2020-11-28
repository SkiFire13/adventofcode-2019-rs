#[allow(unused_imports)]
use super::prelude::*;
use super::intcode::{DeviceStatus, IntcodeDevice};
type Input = IntcodeDevice;

pub fn input_generator(input: &str) -> Input {
    input.parse().expect("Error parsing the IntcodeDevice")
}

pub fn part1(input: &Input) -> i64 {
    let device = input;
    let mut max = std::i64::MIN;
    for (a,b,c,d,e) in permutations_iter() {
        let mut input = 0;
        for n in &[a,b,c,d,e] {
            let mut device = device.clone();
            device.input.push_back(*n);
            device.input.push_back(input);
            device.execute();
            input = device.output.pop_back().expect("No output from IntcodeDevice");
        }
        max = if input > max { input } else { max };
    }

    max
}

pub fn part2(input: &Input) -> i64 {
    let device = input;
    let mut max = std::i64::MIN;

    for (a,b,c,d,e) in permutations_iter() {
        let mut devices = [device.clone(), device.clone(), device.clone(), device.clone(), device.clone()];
        for (device, phase) in devices.iter_mut().zip([a,b,c,d,e].iter()) {
            device.input.push_back(phase+5);
        }
        let mut buffer = vec![0];
        let output = 'output: loop {
            for (i, device) in devices.iter_mut().enumerate() {
                device.input.append(&mut buffer.into());
                let execute_result = device.execute();
                buffer = device.output.drain(..).collect();
                if let DeviceStatus::Halt = execute_result {
                    if i == 4 {
                        break 'output buffer.get(0).cloned().expect("No output from IntcodeDevice");
                    }
                }
            }
        };
        max = if output > max { output } else { max };
    }

    max
}

fn permutations_iter() -> impl Iterator<Item = (i64, i64, i64, i64, i64)> {
    (0..5)
        .flat_map(|a| (0..5).filter(move|&b| b != a).map(move|b| (a,b)))
        .flat_map(|(a,b)| (0..5).filter(move|&c| c != b && c != a).map(move|c|(a,b,c)))
        .flat_map(|(a,b,c)| (0..5).filter(move|&d| d != a && d != b && d != c).map(move|d|(a,b,c,d)))
        .flat_map(|(a,b,c,d)| (0..5).filter(move|&e| e != a && e != b && e != c && e != d).map(move|e|(a,b,c,d,e)))
}
