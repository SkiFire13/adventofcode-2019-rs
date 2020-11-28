#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<((i32, i32), u32)>;

pub fn input_generator(input: &str) -> Input {
    let mut paths = Vec::new();

    for line in input.lines() {
        let mut current = (0i32, 0i32);
        let mut points = HashMap::new();
        let mut distance = 0u32;

        for segment in line.split(',') {
            let delta = match segment.chars().next() {
                Some('R') => (1, 0),
                Some('L') => (-1, 0),
                Some('U') => (0, 1),
                Some('D') => (0, -1),
                _ => panic!("Invalid input")
            };
            let length: usize = segment[1..].parse().expect("Cannot parse number in input");
            for _ in 0..length {
                distance += 1;
                current = (current.0 + delta.0, current.1 + delta.1);
                points.entry(current).or_insert(distance);
            }
        }
        paths.push(points);
    }
    
    if paths.len() != 2 { panic!("Input has a wrong number of paths"); }
    
    let second_path = paths.remove(1);
    let first_path = paths.remove(0);

    first_path.into_iter()
        .filter_map(move |(k, v1)| second_path.get(&k).map(|v2| (k, v1 + v2)))
        .collect()
}

pub fn part1(input: &Input) -> u32 {
    let intersections = input;
    intersections.iter()
        .map(|((x, y), _)| (x.abs() + y.abs()) as u32)
        .min()
        .expect("There are no intersections in the paths")
}

pub fn part2(input: &Input) -> u32 {
    let intersections = input;
    intersections.iter()
        .map(|(_, v)| *v)
        .min()
        .expect("There are no intersections in the paths")
}
