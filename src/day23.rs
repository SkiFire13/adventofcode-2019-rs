#[allow(unused_imports)]
use super::prelude::*;
use super::intcode::IntcodeDevice;
type Input = IntcodeDevice;

pub fn input_generator(input: &str) -> Input {
    input.parse().expect("Error parsing the IntcodeDevice")
}

pub fn part1(input: &Input) -> i64 {
    let mut devices = (0..50).map(|ip| {
        let mut device = input.clone();
        device.input.push_back(ip);
        device
    }).collect::<Vec<_>>();
    for ip in (0..devices.len()).cycle() {
        let device = &mut devices[ip];
        if device.input.len() == 0 {
            device.input.push_back(-1);
        }
        device.execute();
        let output = device.output.drain(..).collect::<Vec<_>>();
        for o in output.chunks(3) {
            if o.len() != 3 {
                panic!("Invalid packet size")
            } else if o[0] == 255 {
                return o[2]
            } else if o[0] >= 0 && o[0] < 50 {
                devices[o[0] as usize].input.extend(&o[1..])
            } else {
                panic!("Invalid ip")
            }
        }
    }
    unreachable!();
}

pub fn part2(input: &Input) -> i64 {
    let mut devices = (0..50).map(|ip| {
        let mut device = input.clone();
        device.input.push_back(ip);
        device
    }).collect::<Vec<_>>();
    let mut sent = true;
    let mut nat = [0, 0];
    let mut nat_seen = std::collections::HashSet::new();
    loop {
        if !sent {
            if nat_seen.insert(nat[1]) {
                devices[0].input.extend(&nat);
            } else {
                return nat[1]
            }
        }
        sent = false;
        for ip in 0..devices.len() {
            let device = &mut devices[ip];
            if device.input.len() == 0 {
                device.input.push_back(-1);
            }
            device.execute();
            let output = devices[ip].output.drain(..).collect::<Vec<_>>();
            for packet in output.chunks(3) {
                if packet.len() != 3 {
                    panic!("Invalid packet size")
                } else if packet[0] == 255 {
                    nat = [packet[1], packet[2]];
                } else if packet[0] >= 0 && packet[0] < 50 {
                    devices[packet[0] as usize].input.extend(&packet[1..])
                } else {
                    panic!("Invalid ip")
                }
                sent = true;
            }
        }
    }
}
