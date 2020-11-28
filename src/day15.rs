#[allow(unused_imports)]
use super::prelude::*;
use super::intcode::IntcodeDevice;
type Input = (HashMap<(i32, i32), MapStatus>, (i32, i32));

#[derive(PartialEq, Eq)]
pub enum MapStatus { Wall, Empty, Oxygen }

pub fn input_generator(input: &str) -> Input {
    let mut device: IntcodeDevice = input.parse().expect("Error parsing the IntcodeDevice");
    let mut map = HashMap::new();
    let mut path = Vec::new();

    path.push((0, 0));
    map.insert((0, 0), MapStatus::Empty);

    let mut oxygen_pos = None;

    loop {
        let pos = *path.last().unwrap();
        
        let dest = near(pos, &map).filter(|p| map.get(p) == None).next().or_else(|| {
            path.pop(); // Go back
            path.pop()
        });
        let dest = if let Some(dest) = dest { dest }
            else { return (map, oxygen_pos.expect("No oxygen found")) };

        device.input.push_back(match (dest.0 - pos.0, dest.1 - pos.1) {
            (0, 1) => 1,
            (0, -1) => 2,
            (1, 0) => 3,
            (-1, 0) => 4,
            _ => unreachable!(),
        });
        device.execute();
        let map_type = match device.output.pop_front() {
            Some(0) => MapStatus::Wall,
            Some(1) => MapStatus::Empty,
            Some(2) => MapStatus::Oxygen,
            _ => panic!("Invalid IntcodeDevice output")
        };
        if map_type == MapStatus::Oxygen { oxygen_pos = Some(dest); }
        if map_type != MapStatus::Wall { path.push(dest); }
        map.insert(dest, map_type);
    }
}

pub fn part1(input: &Input) -> u32 {
    let (map, oxygen_pos) = input;
    let mut queue = BinaryHeap::new();
    let mut seen = HashSet::new();

    queue.push(Reverse((0, (0, 0))));
    loop {
        let Reverse((dist, pos)) = queue.pop().unwrap();
        if seen.insert(pos) {
            if pos == *oxygen_pos {
                return dist
            }
            for near_pos in near(pos, map) {
                queue.push(Reverse((dist+1, near_pos)));
            }
        }
    }
}

pub fn part2(input: &Input) -> u32 {
    let (map, oxygen_pos) = input;
    let mut queue = BinaryHeap::new();
    let mut seen = HashSet::new();

    queue.push(Reverse((0, *oxygen_pos)));
    
    let mut max = 0;

    loop {
        if let Some(Reverse((dist, pos))) = queue.pop() {
            if seen.insert(pos) {
                max = dist;
                for near_pos in near(pos, &map) {
                    queue.push(Reverse((dist+1, near_pos)));
                }
            }
        } else {
            return max
        }
    }
}

fn near(pos: (i32, i32), map: &HashMap<(i32, i32), MapStatus>) -> impl Iterator<Item = (i32, i32)> + '_ {
    [(1, 0), (-1, 0), (0, 1), (0, -1)].iter()
        .map(move |(dx, dy)| (pos.0 + dx, pos.1 + dy))
        .filter(move |new_pos| map.get(new_pos) != Some(&MapStatus::Wall))
}
