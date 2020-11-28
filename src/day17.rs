#[allow(unused_imports)]
use super::prelude::*;
use super::intcode::IntcodeDevice;
type Input = (HashMap<(isize, isize), i64>, IntcodeDevice);

fn parse_map(output: Vec<i64>) -> HashMap<(isize, isize), i64> {
    output
        .split(|&chr| chr == 10)
        .enumerate()
        .flat_map(|(y, slice)| slice.iter()
            .enumerate()
            .map(move |(x, &chr)| ((x as isize, y as isize), chr))
        )
        .collect()
}

pub fn input_generator(input: &str) -> Input {
    let mut device: IntcodeDevice = input.parse().expect("Error parsing the IntcodeDevice");
    device.memory[0] = 2;
    device.execute();

    let map = parse_map(device.output.drain(..).collect());

    (map, device)
}

pub fn part1(input: &Input) -> isize {
    let (map, _) = input;
    map.iter()
        .filter(|&(_, &v)| v == 35)
        .map(|(&k, _)| k)
        .filter(|(x, y)| 
            [(1, 0), (-1, 0), (0, 1), (0, -1)].iter()
                .all(|(dx, dy)| map.get(&(x + dx, y + dy)) == Some(&35))
        )
        .map(|(x, y)| x * y)
        .sum()
}

pub fn part2(input: &Input) -> i64 {
    let (map, device) = input;
    let (mut robot_pos, robot_dir) = map.iter()
        .map(|(&k, &v)| (k, v))
        .filter(|&(_, v)| v == 94 || v == 118 || v == 60 || v == 62) // ^, v, <, >
        .next()
        .expect("No robot found");
    let robot_dir = match robot_dir {
        94 => (0, -1),
        118 => (0, 1),
        60 => (-1, 0),
        62 => (1, 0),
        _ => unreachable!(),
    };

    let mut direction = robot_dir;
    let mut moves: Vec<(char, usize)> = Vec::new();
    // Calculates the needed moves and saves them in `moves`
    loop {
        if map.get(&(robot_pos.0 + direction.0, robot_pos.1 + direction.1)) == Some(&35) {
            moves.last_mut().unwrap().1 += 1;
        } else {
            let new_dir = [(1, 0), (-1, 0), (0, 1), (0, -1)].iter().cloned()
                .filter(|&(dx, dy)| dx != -direction.0 && dy != -direction.1) // Can't go back
                .filter(|&(dx, dy)| map.get(&(robot_pos.0 + dx, robot_pos.1 + dy)) == Some(&35))
                .next();
            if let Some(new_dir) = new_dir {
                let ((x1, y1), (x2, y2)) = (new_dir, direction);
                let dir_char = match (x1 * x2 + y1 * y2, x1 * y2 - y1 * x2) {
                    (0, 1) => 'L',
                    (0, -1) => 'R',
                    _ => panic!("Invalid robot position"),
                };
                moves.push((dir_char, 1));
                direction = new_dir;
            } else {
                break;
            }
        }
        robot_pos = (robot_pos.0 + direction.0, robot_pos.1 + direction.1);
    }

    let mut min_idx = 0;
    let mut idxs_already_taken = Vec::new();
    let mut functions: Vec<&[(char, usize)]> = Vec::new();
    let mut main_routine = Vec::new();
    // Finds the functions
    'outer: while min_idx < moves.len() {
        // Match an already existing functions
        for (function_id, &function) in functions.iter().enumerate() {
            if &moves[min_idx..min_idx+function.len()] == function {
                main_routine.push(function_id);
                min_idx += function.len();
                continue 'outer;
            }
        }
        let max_idx = idxs_already_taken.iter().cloned().filter(|&idx| idx >= min_idx).min().unwrap_or(moves.len());
        let mut new_function = max_function(&moves[min_idx..max_idx]);
        while repeating_idxs(&moves[min_idx..], &new_function).count() < 2 {
            new_function = &new_function[..new_function.len()-1];
        }
        main_routine.push(functions.len());
        functions.push(new_function);
        min_idx += new_function.len();
        idxs_already_taken.extend(repeating_idxs(&moves[min_idx..], &new_function).map(|rel_idx| rel_idx + min_idx));
    }

    if functions.len() != 3 { panic!("Can't find the 3 functions") }

    let mut answer: String = main_routine.into_iter()
        .map(|function_id| format!("{}", (65 + function_id) as u8 as char))
        .collect::<Vec<_>>()
        .join(",")
        + "\n";
    answer += &functions.into_iter()
        .map(|function| function.iter().map(|(dir, len)| format!("{},{}", dir, len)).collect::<Vec<_>>().join(",")).collect::<Vec<_>>()
        .join("\n");
    answer += "\nn\n";

    let mut device: IntcodeDevice = device.clone();
    device.input.extend(answer.chars().map(|c| c as i64));
    device.execute();

    device.output.pop_back().expect("No output from device")
}

fn max_function(moves: &[(char, usize)]) -> &[(char, usize)] {
    let mut function_len = 1;
    let mut ascii_len = move_len(moves[0]);
    while function_len < moves.len() {
        let next_move_len = move_len(moves[function_len]);
        ascii_len += next_move_len + 1;
        if ascii_len > 20 {
            break;
        } else {
            function_len += 1;
        }
    }
    &moves[..function_len]
}

fn move_len(mut mov: (char, usize)) -> usize {
    let mut len = 2;
    while mov.1 > 0 {
        mov.1 /= 10;
        len += 1;
    }
    len
}

fn repeating_idxs<'a>(moves: &'a[(char, usize)], function: &'a[(char, usize)]) -> impl Iterator<Item = usize> + 'a {
    moves
        .windows(function.len())
        .enumerate()
        .filter(move |&(_, w)| w == function)
        .map(|(rel_idx, _)| rel_idx)
}
