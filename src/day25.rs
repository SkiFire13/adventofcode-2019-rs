#[allow(unused_imports)]
use super::prelude::*;
use super::intcode::{DeviceStatus, IntcodeDevice};
type Input = IntcodeDevice;

fn analyze_output(output: &str) -> (Vec<&str>, Vec<&str>) {
    (
        output.lines()
            .skip(4)
            .take_while(|line| line.starts_with("- "))
            .map(|line| &line[2..])
            .collect(),
        output.lines()
            .skip(4)
            .skip_while(|line| line.starts_with("- "))
            .skip(2)
            .take_while(|line| line.starts_with("- "))
            .map(|line| &line[2..])
            .collect()
    )
}

fn opposite_dir(dir: &str) -> &'static str {
    match dir {
        "north" => "south",
        "south" => "north",
        "east" => "west",
        "west" => "east",
        _ => unreachable!(),
    }
}


pub fn input_generator(input: &str) -> Input {
    input.parse().expect("Error parsing the IntcodeDevice")
}

pub fn part1(input: &Input) -> u32 {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back((input.clone(), ""));

    while let Some((mut device, no_dir)) = queue.pop_front() {
        if !seen.insert(device.clone()) { continue; }
        let execute_result = device.execute();
        let output: String = device.output.drain(..).skip_while(|&i| i == 10).map(|i| i as u8 as char).collect();

        if execute_result == DeviceStatus::Halt && output.starts_with("== Pressure-Sensitive Floor ==") {
            if let Some(typing_idx) = output.find("by typing") {
                return output[typing_idx + 10..].split_whitespace().next()
                    .expect("Incorret format for answer!")
                    .parse::<u32>()
                    .expect("Can't parse answer to u32")
            }
        }

        let (directions, items) = analyze_output(&output);
        for item in items.into_iter().filter(|&item| item != "infinite loop") {
            for &dir in directions.iter() {
                let mut new_device = device.clone();

                new_device.write_str(&format!("take {}\n", item));
                new_device.execute();
                new_device.output.clear();
                new_device.write_str(&format!("{}\n", dir));

                queue.push_back((new_device, opposite_dir(dir)));
            }
        }
        for dir in directions.into_iter().filter(|&dir| dir != no_dir) {
            let mut new_device = device.clone();
            new_device.write_str(&format!("{}\n", dir));
            queue.push_back((new_device, opposite_dir(dir)));
        }
    }

    panic!("Can't get the password");
}
