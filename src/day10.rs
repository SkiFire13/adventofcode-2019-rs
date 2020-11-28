#[allow(unused_imports)]
use super::prelude::*;
type Input = HashSet<Asteroid>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Asteroid { x: isize, y: isize}
impl Asteroid {
    fn dist(&self) -> isize {
        self.x * self.x + self.y * self.y
    }
}
impl std::ops::Sub for Asteroid {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self { x: self.x - other.x, y: self.y - other.y }
    }
}

pub fn input_generator(input: &str) -> Input {
    input.lines()
        .enumerate()
        .flat_map(|(y, line)|
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(move |(x, _)| Asteroid { x: x as isize, y: y as isize })
        )
        .collect()
}

pub fn part1(input: &Input) -> usize {
    let asteroids = input;
    asteroids
        .iter()
        .map(|&asteroid| seen_by(&asteroids, asteroid).count())
        .max()
        .expect("No asteroids")
}

pub fn part2(input: &Input) -> isize {
    let mut asteroids = input.clone();
    let station = *asteroids
        .iter()
        .max_by_key(|&&asteroid| seen_by(&asteroids, asteroid).count())
        .expect("No asteroids");
    asteroids.remove(&station);
    
    let mut last_removed_angle = std::f64::consts::FRAC_PI_2;
    let mut count = 0;

    let first = asteroids.iter()
        .map(|&asteroid| station - asteroid)
        .filter(|&Asteroid { x, y }| x == 0 && y > 0)
        .min_by_key(|&asteroid| asteroid.y);
    if let Some(first) = first {
        let target = station - first;
        asteroids.remove(&target);
        count += 1;
    }

    loop {
        let (target, target_angle) = asteroids.iter()
            .map(|&asteroid| station - asteroid)
            .map(|asteroid| (asteroid, (asteroid.y as f64).atan2(asteroid.x as f64)))
            .min_by(|&(ast1, mut a1), &(ast2, mut a2)| {
                while a1 <= last_removed_angle { a1 += 2.0 * std::f64::consts::PI; }
                while a2 <= last_removed_angle { a2 += 2.0 * std::f64::consts::PI; }
                a1.partial_cmp(&a2).unwrap_or(std::cmp::Ordering::Equal)
                    .then(ast1.dist().cmp(&ast2.dist()))
            })
            .expect("There are less than 200 asteroids");
        
        let target = station - target;
        
        last_removed_angle = target_angle;
        asteroids.remove(&target);
        count += 1;

        if count == 200 {
            return 100 * target.x + target.y
        }
    }
}

fn seen_by(asteroids: &HashSet<Asteroid>, station: Asteroid) -> impl Iterator<Item = Asteroid> + '_ {
    asteroids.iter()
        .filter(move |&&a| a != station)
        .map(move |&asteroid| asteroid - station)
        .filter(move |&Asteroid { mut x, mut y }| {
            let gcd = gcd(x.abs(), y.abs());
            x = x / gcd;
            y = y / gcd;
            (1..gcd).all(|gcd| !asteroids.contains(&Asteroid { x: station.x + x * gcd, y: station.y + y * gcd }))
        })
}

fn gcd(mut x: isize, mut y: isize) -> isize {
    if x == 0 { return y }
    if y == 0 { return x }
    while y != 0 {
        std::mem::swap(&mut x, &mut y);
        y = y % x;
    }
    return x
}
