#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Action>;

#[derive(FromStr)]
pub enum Action {
    #[display("cut {0}")]
    Cut(i128),
    #[display("deal into new stack")]
    Stack,
    #[display("deal with increment {0}")]
    Increment(i128)
}

pub fn input_generator(input: &str) -> Input {
    input.lines()
        .map(|line| line.parse().expect("Invalid input"))
        .collect()
}

pub fn part1(input: &Input) -> i128 {
    let actions = input;
    let size: i128 = 10007;
    let mut idx: i128 = 2019;

    for action in actions {
        idx = match action {
            Action::Cut(cut) => idx - cut,
            Action::Stack => -idx - 1,
            Action::Increment(increment) => increment * idx,
        };
        idx = idx.rem_euclid(size);
    }

    idx
}

pub fn part2(input: &Input) -> i128 {
    let actions = input;
    let size: i128 = 119315717514047;
    let n_iter: i128 = 101741582076661;
    let idx: i128 = 2020;

    let mut coeffs = (1, 0);
    for action in actions.iter().rev() {
        coeffs = match action {
            Action::Cut(cut) => (coeffs.0, coeffs.1 + cut),
            Action::Stack => (-coeffs.0, -coeffs.1-1),
            Action::Increment(increment) => {
                let inv = mod_inv(*increment, size);
                (coeffs.0 * inv, coeffs.1 * inv)
            }
        };
        coeffs = (coeffs.0.rem_euclid(size), coeffs.1.rem_euclid(size));
    }
    let (a, b) = coeffs;
    let an = mod_pow(a, n_iter, size);

    (an * idx % size + (an - 1) * mod_inv(a - 1, size) % size * b % size) % size
}

fn mod_pow(mut base: i128, mut exp: i128, modulus: i128) -> i128 {
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}

fn mod_inv(a: i128, module: i128) -> i128 {
    let mut mn = (module, a);
    let mut xy = (0, 1);
   
    while mn.1 != 0 {
        xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
        mn = (mn.1, mn.0 % mn.1);
    }
   
    while xy.0 < 0 {
        xy.0 += module;
    }
    
    xy.0
}
