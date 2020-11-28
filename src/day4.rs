#[allow(unused_imports)]
use super::prelude::*;
type Input = (u32, u32);

pub fn input_generator(input: &str) -> Input {
    let mut words = input.split('-');
    (
        words.next()
            .expect("Invalid input")
            .parse::<u32>()
            .expect("Cannot parse as a positive integer"),
        words.next()
            .expect("Invalid input")
            .parse::<u32>()
            .expect("Cannot parse as a positive integer")
    )
}

pub fn part1(input: &Input) -> usize {
    let &(min, max) = input;
    find_valid(min, max, |count| count >= 2)
}

pub fn part2(input: &Input) -> usize {
    let &(min, max) = input;
    find_valid(min, max, |count| count == 2)
}

fn find_valid(min: u32, max: u32, count_condition: impl Fn(usize) -> bool) -> usize {
    (std::cmp::max(100_000, min) ..= std::cmp::min(999_999, max))
        .filter(|&n| {
            (n % 111_111 == 0 && count_condition(6)) || 
            {
                let (double, incr) = count_digits_tupled(n)
                    .fold((false, true), |(double, incr), ((digit_1, count_1), (digit_2, count_2))|
                        (
                            double || count_condition(count_1) || count_condition(count_2),
                            incr && digit_1 <= digit_2
                        )
                    );
                double && incr
            }
        })
        .count()
}

fn count_digits_tupled(n: u32) -> impl Iterator<Item = ((u32, usize), (u32, usize))> {
    let mut digits = std::iter::successors(
        Some(n),
        |&prev| if prev >= 10 { Some(prev / 10) } else { None }
    )
    .map(|i| i % 10);

    let mut next_digit = digits.next();
    let mut next = move || {
        next_digit.take().map(|t| {
            let mut count = 1;
            loop {
                next_digit = digits.next();
                match next_digit {
                    Some(next_digit) if next_digit == t => count += 1,
                    _ => break (t, count),
                }
            }
        })
    };

    std::iter::successors(
        next().and_then(|v2| next().map(|v1| (v1, v2)) ),
        move |&(v2, _)| next().map(|v1| (v1, v2))
    )
}
