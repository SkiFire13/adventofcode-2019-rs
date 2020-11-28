#[allow(unused_imports)]
use super::prelude::*;
type Input = (Vec<String>, HashMap<usize, Vec<usize>>, HashMap<usize, Vec<usize>>);

pub fn input_generator(input: &str) -> Input {
    let mut names = Vec::new();
    let mut seen = HashMap::new();
    let mut depended_by = HashMap::new();
    let mut depends_on = HashMap::new();

    fn get_idx<'a>(dep: &'a str, names: &mut Vec<String>, seen: &mut HashMap<&'a str, usize>) -> usize {
        if let Some(&idx) = seen.get(dep) {
            idx
        } else {
            names.push(dep.to_string());
            seen.insert(dep, names.len() - 1);
            names.len() - 1
        }
    };

    for line in input.lines() {
        let mut split = line.split(')');
        let dependency = get_idx(split.next().expect("Invalid input line"), &mut names, &mut seen);
        let depend = get_idx(split.next().expect("Invalid input line"), &mut names, &mut seen);

        if split.next().is_some() { panic!("Additional line not expected"); }
        depended_by.entry(dependency).or_insert_with(Vec::new).push(depend);
        depends_on.entry(depend).or_insert_with(Vec::new).push(dependency);
    }
    (names, depended_by, depends_on)
}

pub fn part1(input: &Input) -> usize {
    let (names, _, depends_on) = input;
    let mut cache = vec![None; names.len()];
    fn num_orbits(node: usize, depends_on: &HashMap<usize, Vec<usize>>, cache: &mut Vec<Option<usize>>) -> usize {
        if let Some(cached) = cache[node] {
            return cached;
        }
        let sol = depends_on.get(&node).map_or(0, |node_deps|
            node_deps.len() + node_deps.iter().map(|&sub_node| num_orbits(sub_node, depends_on, cache)).sum::<usize>()
        );
        cache[node] = Some(sol);
        sol
    }
    depends_on.keys().map(|&k| num_orbits(k, &depends_on, &mut cache)).sum()
}

pub fn part2(input: &Input) -> i32 {
    let (names, depended_by, depends_on) = input;
    let mut seen = HashSet::new();
    let mut queue = BinaryHeap::new();

    let you_key = names.iter().position(|e| e == "YOU").expect("No YOU key");
    let san_key = names.iter().position(|e| e == "SAN").expect("No SAN key");

    queue.push(Reverse((0, you_key)));
    loop {
        let Reverse((dist, node)) = queue.pop().expect("There's no path from YOU to SAN");
        if seen.contains(&node) { continue; }
        if node == san_key { return dist - 2 }
        seen.insert(node);
        depended_by.get(&node).into_iter().flatten()
            .chain(depends_on.get(&node).into_iter().flatten())
            .for_each(|&sub_node| queue.push(Reverse((dist + 1, sub_node))))
    }
}
