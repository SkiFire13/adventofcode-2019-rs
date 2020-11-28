#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Moon>;

#[derive(Clone, Copy, PartialEq, Eq, FromStr)]
#[display("<x={x}, y={y}, z={z}>")]
#[from_str(default_fields("vx", "vy", "vz"))]
pub struct Moon {
    x: i32, y: i32, z: i32,
    vx: i32, vy: i32, vz: i32,
}

fn step_to_repeat(mut parts: Vec<(i32, i32)>) -> u64 {
    let initial_parts = parts.clone();
    let mut step = 1;
    loop {
        for i in 0..parts.len() {
            parts[i].1 += parts.iter().map(|&(s, _)| cmp_i32(s, parts[i].0)).sum::<i32>();
        }
        for moon_part in parts.iter_mut() {
            moon_part.0 += moon_part.1;
        }
        if initial_parts == parts {
            return step
        }
        step += 1;
    }
}

fn cmp_i32(a: i32, b: i32) -> i32 {
    match a.cmp(&b) {
        std::cmp::Ordering::Greater => 1,
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
    }
}

fn mcm(a: u64, b: u64) -> u64 {
    let mut x = a;
    let mut y = b;
    while y != 0 {
        std::mem::swap(&mut x, &mut y);
        y = y % x;
    }

    a * b / x
}

pub fn input_generator(input: &str) -> Input {
    input.lines()
        .map(|line| line.parse().expect("Invalid input"))
        .collect()
}

pub fn part1(input: &Input) -> i32 {
    let mut moons = input.clone();

    for _ in 0..1000 {
        for i in 0..moons.len() {
            let current_moon = &moons[i];
            let (dx, dy, dz) = moons.iter().fold((0i32, 0i32, 0i32), |(dx, dy, dz), moon|
                (
                    dx + cmp_i32(moon.x, current_moon.x),
                    dy + cmp_i32(moon.y, current_moon.y),
                    dz + cmp_i32(moon.z, current_moon.z),
                )
            );
            let current_moon = &mut moons[i];
            current_moon.vx += dx;
            current_moon.vy += dy;
            current_moon.vz += dz;
        }
        for moon in moons.iter_mut() {
            moon.x += moon.vx;
            moon.y += moon.vy;
            moon.z += moon.vz;
        }
    }

    moons.iter()
        .map(|moon| (moon.x.abs() + moon.y.abs() + moon.z.abs()) * (moon.vx.abs() + moon.vy.abs() + moon.vz.abs()))
        .sum()
}

pub fn part2(input: &Input) -> u64 {
    let moons = input;

    let step_x = step_to_repeat(moons.iter().map(|&moon| (moon.x, moon.vx)).collect());
    let step_y = step_to_repeat(moons.iter().map(|&moon| (moon.y, moon.vy)).collect());
    let step_z = step_to_repeat(moons.iter().map(|&moon| (moon.z, moon.vz)).collect());

    mcm(step_x, mcm(step_y, step_z))
}
