#[allow(unused_imports)]
use super::prelude::*;
type Input = (HashSet<Point>, HashMap<Point, Point>, Point, Point);

type Point = (isize, isize);

pub fn input_generator(input: &str) -> Input {
    let mut walls = HashSet::new();
    let mut routes = HashSet::new();
    let mut chars = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let (x, y) = (x as isize, y as isize);
            match c {
                '#' => { walls.insert((x, y)); },
                '.' => { routes.insert((x, y)); }
                c if c.is_ascii_uppercase() => { 
                    walls.insert((x, y));
                    chars.insert((x, y), c);
                },
                _ => {}
            }
        }
    }

    let mut start = None;
    let mut target = None;
    let mut portals_seen = HashMap::new();
    let mut portals = HashMap::new();

    for (&point, &c1) in chars.iter() {
        let real_portal = [(0, 1), (0, -1), (1, 0), (-1, 0)].iter().cloned()
            .map(|(dx, dy)| (point.0 + dx, point.1 + dy))
            .filter(|point| routes.contains(point))
            .next();
        if let Some(real_portal) = real_portal {
            let (new_point, c2) = [(0, 1), (0, -1), (1, 0), (-1, 0)].iter().cloned()
                .map(|(dx, dy)| (point.0 + dx, point.1 + dy))
                .filter_map(|new_point| chars.get(&new_point).cloned().map(|c2| (new_point, c2)))
                .next()
                .expect("One letter portals are invalid");
            match (c1, c2) {
                ('A', 'A') => start = Some(start.xor(Some(real_portal))
                    .expect("Multiple starting points found")),
                ('Z', 'Z') => target = Some(target.xor(Some(real_portal))
                    .expect("Multiple ending points found")),
                (c1, c2) => {
                    let (c1, c2) = if point < new_point { (c1, c2) } else { (c2, c1) };
                    if let Some(other_portal) = portals_seen.insert((c1, c2), real_portal) {
                        portals.insert(real_portal, other_portal);
                        portals.insert(other_portal, real_portal);
                    }
                }
            }
        }
    }

    (walls, portals, start.expect("No start point found"), target.expect("No target point found"))
}

pub fn part1(input: &Input) -> u32 {
    let &(ref walls,ref  portals, start, target) = input;
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    queue.push_back((0, start));

    while let Some((distance, point)) = queue.pop_front() {
        if seen.insert(point) {
            if point == target {
                return distance;
            }
            queue.extend(
                [(0, 1), (0, -1), (1, 0), (-1, 0)].iter().cloned()
                    .map(|(dx, dy)| (point.0 + dx, point.1 + dy))
                    .filter(|new_point| !walls.contains(new_point))
                    .chain(portals.get(&point).cloned().into_iter())
                    .map(|new_point| (distance + 1, new_point))
            );
        }
    }

    panic!("Can't reach target")
}

pub fn part2(input: &Input) -> i32 {
    let &(ref walls,ref  portals, start, target) = input;
    let (minx, maxx, miny, maxy) = portals.keys()
        .fold((std::isize::MAX, 0, std::isize::MAX, 0), |(minx, maxx, miny, maxy), &(x, y)| {
            (
                std::cmp::min(minx, x),
                std::cmp::max(maxx, x),
                std::cmp::min(miny, y),
                std::cmp::max(maxy, y),
            )
        });
    
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    queue.push_back((0, start, 0));

    while let Some((distance, point, depth)) = queue.pop_front() {
        if seen.insert((point, depth)) {
            if point == target && depth == 0 {
                return distance;
            }
            queue.extend(
                [(0, 1), (0, -1), (1, 0), (-1, 0)].iter().cloned()
                    .map(|(dx, dy)| (point.0 + dx, point.1 + dy))
                    .filter(|new_point| !walls.contains(new_point))
                    .map(|new_point| (distance + 1, new_point, depth))
                    .chain(
                        portals.get(&point).cloned().into_iter()
                            .map(|new_point| {
                                let new_depth = depth + if 
                                    point.0 == minx ||
                                    point.0 == maxx ||
                                    point.1 == miny ||
                                    point.1 == maxy
                                    { -1 } else { 1 };
                                (distance + 1, new_point, new_depth)
                            })
                            .filter(|&(_, _, new_depth)| new_depth >= 0)
                    )
            );
        }
    }

    panic!("Can't reach target")
}
