#[allow(unused_imports)]
use super::prelude::*;
type Input = Layout;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Layout { layout: u32 }

impl Layout {
    fn new() -> Self { Self { layout: 0 } }
    fn get(&self, x: usize, y: usize) -> Tile {
        if self.layout & (1 << (x + 5 * y)) != 0 { Tile::Bug } else { Tile::Empty }
    }
    fn set(&mut self, x: usize, y: usize, new_tile: Tile) {
        match new_tile {
            Tile::Bug => self.layout = self.layout | (1 << (x + 5 * y)),
            Tile::Empty => self.layout = self.layout & !(1 << (x + 5 * y)),
        }
    }
    fn count_near_bugs(&self, x: usize, y: usize) -> usize {
        [(1, 0), (-1, 0), (0, 1), (0, -1)].iter().cloned()
            .map(|(dx, dy)| (x as isize + dx, y as isize + dy))
            .filter(|&(nx, ny)| nx >= 0 && nx < 5 && ny >= 0 && ny < 5)
            .map(|(nx, ny)| (nx as usize, ny as usize))
            .filter(|&(nx, ny)| self.get(nx, ny) == Tile::Bug)
            .count()
    }
    fn biodiversity(&self) -> u32 { self.layout }
    fn is_empty(&self) -> bool {  self.layout == 0 }
    fn count(&self) -> u32 { self.layout.count_ones() }
}

#[derive(PartialEq, Eq)]
enum Tile { Bug, Empty }

pub fn input_generator(input: &str) -> Input {
    let mut layout = Layout::new();
    for (y, line) in input.lines().enumerate() {
        if y >= 5 { panic!("Too much lines in input") }
        for (x, c) in line.chars().enumerate() {
            if x >= 5 { panic!("Too much chars in line") }
            layout.set(x, y, match c {
                '#' => Tile::Bug,
                '.' => Tile::Empty,
                _ => panic!("Invalid char in input")
            })
        }
    }
    layout
}

pub fn part1(input: &Input) -> u32 {
    let mut layout = *input;
    let mut seen = HashSet::new();

    while seen.insert(layout) {
        let mut new_layout = Layout::new();
        for (x, y) in (0..5).flat_map(|x| (0..5).map(move |y| (x,y))) {
            let count_near = layout.count_near_bugs(x, y);
            match layout.get(x, y) {
                Tile::Bug if count_near == 1 =>  new_layout.set(x, y, Tile::Bug),
                Tile::Empty if count_near == 1 || count_near == 2 => new_layout.set(x, y, Tile::Bug),
                _ => {}
            }
        }
        layout = new_layout;
    }

    layout.biodiversity()
}

pub fn part2(input: &Input) -> u32 {
    let layout = *input;
    let mut layouts = VecDeque::with_capacity(403);
    let mut new_layouts = VecDeque::with_capacity(403);
    layouts.push_back(layout);

    layouts.push_front(Layout::new());
    layouts.push_back(Layout::new());

    for _ in 0..200 {
        if !layouts[1].is_empty() { layouts.push_front(Layout::new()); }
        if !layouts[layouts.len() - 2].is_empty() { layouts.push_back(Layout::new()); }

        new_layouts.clear();
        new_layouts.extend((0..layouts.len()).map(|_| Layout::new()));

        for (x, y, depth) in (0..5)
            .flat_map(|x| (0..5).map(move |y| (x, y)))
            .flat_map(|(x, y)| (1..layouts.len()-1).map(move |d| (x, y, d)))
            .filter(|&(x, y, _)| !(x == 2 && y == 2))
        {
            let count_near = layouts[depth].count_near_bugs(x, y)
                + if x == 0 { (layouts[depth - 1].get(1, 2) == Tile::Bug) as usize } else { 0 }
                + if x == 4 { (layouts[depth - 1].get(3, 2) == Tile::Bug) as usize } else { 0 }
                + if y == 0 { (layouts[depth - 1].get(2, 1) == Tile::Bug) as usize } else { 0 }
                + if y == 4 { (layouts[depth - 1].get(2, 3) == Tile::Bug) as usize } else { 0 }
                + match (x, y) {
                    (1, 2) => (0..5).filter(|&y| layouts[depth + 1].get(0, y) == Tile::Bug).count(),
                    (3, 2) => (0..5).filter(|&y| layouts[depth + 1].get(4, y) == Tile::Bug).count(),
                    (2, 1) => (0..5).filter(|&x| layouts[depth + 1].get(x, 0) == Tile::Bug).count(),
                    (2, 3) => (0..5).filter(|&x| layouts[depth + 1].get(x, 4) == Tile::Bug).count(),
                    _ => 0,
                };

            match layouts[depth].get(x, y) {
                Tile::Bug if count_near == 1 => new_layouts[depth].set(x, y, Tile::Bug),
                Tile::Empty if count_near == 1 || count_near == 2 => new_layouts[depth].set(x, y, Tile::Bug),
                _ => {},
            }
        }

        std::mem::swap(&mut layouts, &mut new_layouts);
    }

    layouts.iter().map(|layout| layout.count()).sum()
}
