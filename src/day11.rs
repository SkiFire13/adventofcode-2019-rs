#[allow(unused_imports)]
use super::prelude::*;
use super::intcode::{DeviceStatus, IntcodeDevice};
type Input = IntcodeDevice;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Color { Black, White }

pub fn input_generator(input: &str) -> Input {
    input.parse().expect("Error parsing the IntcodeDevice")
}

pub fn part1(input: &Input) -> usize {
    let mut device = input.clone();

    let mut panels = HashMap::new();
    let mut position = (0isize, 0isize);
    let mut direction = (0isize, 1isize);

    loop { 
        device.input.push_back( if let Some(Color::White) = panels.get(&position) { 1 } else { 0 });
        match device.execute() {
            DeviceStatus::Halt => return panels.len(),
            DeviceStatus::WaitingInput => {
                direction = match device.output.pop_back() {
                    // Complex point rotation; multiply by i * delta
                    Some(0) => (-direction.1, direction.0),
                    Some(1) => (direction.1, -direction.0),
                    _ => panic!("Invalid direction output from VM")
                };
                let new_color = match device.output.pop_back() {
                    Some(0) => Color::Black,
                    Some(1) => Color::White,
                    _ => panic!("Invalid direction output from VM")
                };
                panels.insert(position, new_color);
                position = (position.0 + direction.0, position.1 + direction.1);
            }
        }
    }
}

pub fn part2(input: &Input) -> String {
    let mut device = input.clone();

    let mut panels = HashMap::new();
    let mut position = (0isize, 0isize);
    let mut direction = (0isize, 1isize);

    panels.insert(position, Color::White);

    loop { 
        device.input.push_back( if let Some(Color::White) = panels.get(&position) { 1 } else { 0 });
        match device.execute() {
            DeviceStatus::Halt => break,
            DeviceStatus::WaitingInput => {
                direction = match device.output.pop_back() {
                    // Complex point rotation; multiply by i * delta
                    Some(0) => (-direction.1, direction.0),
                    Some(1) => (direction.1, -direction.0),
                    _ => panic!("Invalid direction output from VM")
                };
                let new_color = match device.output.pop_back() {
                    Some(0) => Color::Black,
                    Some(1) => Color::White,
                    _ => panic!("Invalid direction output from VM")
                };
                panels.insert(position, new_color);
                position = (position.0 + direction.0, position.1 + direction.1);
            }
        }
    }

    let (minx, maxx, miny, maxy) = panels.keys()
        .fold((None, None, None, None), |(minx, maxx, miny, maxy), &(x, y)|
            (
                Some(std::cmp::min(minx.unwrap_or(x), x)),
                Some(std::cmp::max(maxx.unwrap_or(x), x)),
                Some(std::cmp::min(miny.unwrap_or(y), y)),
                Some(std::cmp::max(maxy.unwrap_or(y), y)),
            )
        );
    
    let minx = minx.expect("No panels");
    let maxx = maxx.expect("No panels");
    let miny = miny.expect("No panels");
    let maxy = maxy.expect("No panels");

    let mut output = String::with_capacity(8);
    const CHARMAP: [u32; 26] = [
        0b_01100_10010_10010_11110_10010_10010, // A
        0b_11100_10010_11100_10010_10010_11100, // B
        0b_01100_10010_10000_10000_10010_01100, // C
        u32::MAX                              , // D
        0b_11110_10000_11100_10000_10000_11110, // E
        0b_11110_10000_11100_10000_10000_10000, // F
        0b_01100_10010_10000_10110_10010_01110, // G
        0b_10010_10010_11110_10010_10010_10010, // H
        u32::MAX                              , // I
        0b_00110_00010_00010_00010_10010_01100, // J
        0b_10010_10100_11000_10100_10100_10010, // K
        0b_10000_10000_10000_10000_10000_11110, // L
        u32::MAX                              , // M
        u32::MAX                              , // N
        0b_01100_10010_10010_10010_10010_10010, // O
        0b_11100_10010_10010_11100_10000_10000, // P
        u32::MAX                              , // Q
        0b_11100_10010_10010_11100_10100_10010, // R
        0b_01110_10000_10000_01100_00010_11100, // S
        u32::MAX                              , // T
        0b_10010_10010_10010_10010_10010_01100, // U
        u32::MAX                              , // V
        u32::MAX                              , // W
        u32::MAX                              , // X
        0b_10001_10001_01010_00100_00100_00100, // Y
        0b_11110_00010_00100_01000_10000_11110, // Z
    ];

    for nchar in 0..8 {
        let mut acc = 0u32;
        for y in (miny..=maxy).rev() {
            for x_offset in 0..5 {
                let x = 1 + minx + nchar * 5 + x_offset;
                assert!(x <= maxx);
                let pixel = panels.get(&(x, y)) == Some(&Color::White);
                acc <<= 1;
                acc ^= pixel as u32;
            }
        }
        let c = CHARMAP.iter().position(|&encoded| encoded == acc).expect("Failed to decode char");
        let c = (c as u8 + b'A') as char;
        output.push(c)
    }
    
    output
}
