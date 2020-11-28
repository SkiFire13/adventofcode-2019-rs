#[allow(unused_imports)]
use super::prelude::*;
type Input = (Point, HashSet<Point>, HashMap<Point, KeySet>, HashMap<Point, DoorSet>);

type Point = (isize, isize);
type KeySet = u32;
type DoorSet = u32;

pub fn input_generator(input: &str) -> Input {
    let mut start = None;
    let mut walls = HashSet::new();
    let mut keys = HashMap::new();
    let mut doors = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let (x, y) = (x as isize, y as isize);
            match c {
                '.' => {},
                '#' => { walls.insert((x, y)); },
                '@' => start = Some(start.xor(Some((x, y))).expect("Multiple starting points found")),
                c if c.is_ascii_uppercase() => { doors.insert((x, y), 1 << (c.to_ascii_lowercase() as u8 - b'a')); },
                c if c.is_ascii_lowercase()=> { keys.insert((x, y), 1 << (c as u8 - b'a')); },
                _ => panic!("Found invalid char in input")
            }
        }
    }

    (start.expect("No starting point found"), walls, keys, doors)
}

pub fn part1(input: &Input) -> u32 {
    let &(start, ref walls, ref keys, ref doors) = input;
    solve(start, walls, keys, doors)
}

pub fn part2(input: &Input) -> u32 {
    let &((start_x, start_y), ref walls, ref keys, ref doors) = input;
    let mut walls = walls.clone();
    walls.extend([(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)].iter().map(|&(dx, dy)| (start_x + dx, start_y + dy)));

    let mut total = 0;
    for (robot_x, robot_y) in [(1, 1), (1, -1), (-1, 1), (-1, -1)].iter().map(|(dx, dy)| (start_x + dx, start_y + dy)) {
        let same_quadrant = |x: &isize, y: &isize| {
            x.cmp(&robot_x) != start_x.cmp(&robot_x) &&
            y.cmp(&robot_y) != start_y.cmp(&robot_y)
        };
        
        let keys = keys.iter().map(|(&k, &v)| (k, v))
            .filter(|((x, y), _)| same_quadrant(x, y))
            .collect::<HashMap<Point, KeySet>>();

        let keys_set = keys.values().fold(0, |acc, key| acc | key);
        let doors = doors.iter().map(|(&k, &v)| (k, v))
            .filter(|((x, y), _)| same_quadrant(x, y))
            .filter(|(_, k)| keys_set & k != 0)
            .collect::<HashMap<Point, DoorSet>>();
        
        total += solve((robot_x, robot_y), &walls, &keys, &doors);
    }

    total
}

fn solve(
    start: Point,
    walls: &HashSet<Point>,
    keys: &HashMap<Point, KeySet>,
    doors: &HashMap<Point, DoorSet>
) -> u32 {
    let mut reach_keys_cache = ReachKeysCache::new(walls, keys, doors);
    let keys_cache = keys.keys().chain(std::iter::once(&start))
        .map(|&point| (point, reach_keys(&mut reach_keys_cache, point)))
        .collect::<HashMap<Point, Vec<(u32, Point, KeySet, DoorSet)>>>();

    let mut queue = BinaryHeap::new();
    let mut seen = HashSet::new();
    queue.push(Reverse((0, start, 0)));

    let keys_set = keys.values().fold(0, |acc, key| acc | key);

    while let Some(Reverse((distance, robot, coll_keys))) = queue.pop() {
        if coll_keys == keys_set {
            return distance
        }
        if seen.insert((robot, coll_keys)) {
            let accessible_keys = keys_cache[&robot].iter()
                .filter(|(_, _, new_key, _)| coll_keys & new_key == 0)
                .filter(|(_, _, _, needed_doors)| needed_doors & !coll_keys == 0);

            for &(rel_distance, new_robot, new_key, _) in accessible_keys {
                queue.push(Reverse((distance + rel_distance, new_robot, coll_keys | new_key)));
            }
        }
    }

    panic!("There's no way to get all the keys")
}

struct ReachKeysCache<'a> {
    walls: &'a HashSet<Point>,
    keys: &'a HashMap<Point, KeySet>,
    doors: &'a HashMap<Point, DoorSet>,
    queue: VecDeque<(u32, Point, DoorSet)>,
    seen: HashSet<Point>
}

impl<'a> ReachKeysCache<'a> {
    fn new(walls: &'a HashSet<Point>, keys: &'a HashMap<Point, KeySet>, doors: &'a HashMap<Point, DoorSet>) -> Self {
        Self { walls, keys, doors, queue: VecDeque::new(), seen: HashSet::new() }
    }
}

fn reach_keys(cache: &mut ReachKeysCache, current_position: Point) -> Vec<(u32, Point, KeySet, DoorSet)> {
    let mut vec = Vec::new();
    let ReachKeysCache { queue, seen, walls, keys, doors } = cache;
    seen.clear();
    queue.clear();
    queue.push_back((0, current_position, 0));

    while let Some((distance, robot, doors_seen)) = queue.pop_front() {
        if seen.insert(robot) {
            if let Some(&key) = keys.get(&robot) {
                vec.push((distance, robot, key, doors_seen));
            }
            queue.extend([(1, 0), (-1, 0), (0, 1), (0, -1)].iter().cloned()
                .map(|(dx, dy)| (robot.0 + dx, robot.1 + dy))
                .filter(|new_robot| !walls.contains(&new_robot))
                .map(|new_robot| (distance + 1, new_robot, doors_seen | doors.get(&new_robot).unwrap_or(&0)))
            );
        }
    }

    vec
}
