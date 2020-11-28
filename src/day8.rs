#[allow(unused_imports)]
use super::prelude::*;
type Input = String;

pub fn input_generator(input: &str) -> Input { input.to_string() }

pub fn part1(input: &Input) -> usize {
    let (_, o, t) = input
        .as_bytes()
        .chunks(25 * 6)
        .map(|sub_str| (
            sub_str.iter().filter(|&&b| b == b'0').count(),
            sub_str.iter().filter(|&&b| b == b'1').count(),
            sub_str.iter().filter(|&&b| b == b'2').count()
        ))
        .min()
        .expect("Input can't be blank");

    o * t
}

pub fn part2(input: &Input) -> String {
    let input = input.as_bytes();
    let mut output = String::with_capacity(5);
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

    for nchar in 0..5 {
        let mut acc = 0u32;
        for height in 0..6 {
            for width in 0..5 {
                let pos = height * 25 + nchar * 5 + width;
                let pixel = get_pixel(&input, pos);
                let pixel = match pixel {
                    b'0' => 0,
                    b'1' => 1,
                    _ => unreachable!(),
                };
                acc <<= 1;
                acc ^= pixel;
            }
        }

        let c = CHARMAP.iter().position(|&encoded| encoded == acc).expect("Failed to decode char");
        let c = (c as u8 + b'A') as char;
        output.push(c);
    }

    output
}

fn get_pixel(input: &[u8], pos: usize) -> u8 {
    match input.get(pos) {
        None => b'2',
        Some(b'2') => get_pixel(&input[25*6..], pos),
        Some(&b) => b,
    }
}
