use std::collections::VecDeque;
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct IntcodeDevice {
    ip: usize,
    relative_offset: isize,
    pub memory: Vec<i64>,
    pub input: VecDeque<i64>,
    pub output: VecDeque<i64>
}

impl IntcodeDevice {
    pub fn execute(&mut self) -> DeviceStatus {
        loop {
            let opcode = {
                let opcode = self.memory.get(self.ip).cloned().expect("No more instructions");
                if opcode > 0 { opcode as usize } else { panic!("Negative opcode"); }
            };

            use ParamMode::*;
            match Self::parse_opcode(opcode) {
                (99, [Position, Position, Position]) => return DeviceStatus::Halt,
                (1, [mode1, mode2, mode3]) => {
                    let value1 = self.get_value(self.get_param(1), mode1);
                    let value2 = self.get_value(self.get_param(2), mode2);
                    let result_addr = self.get_address(self.get_param(3), mode3);
                    self.memory[result_addr] = value1 + value2;
                    self.ip += 4;
                },
                (2, [mode1, mode2, mode3]) => {
                    let value1 = self.get_value(self.get_param(1), mode1);
                    let value2 = self.get_value(self.get_param(2), mode2);
                    let result_addr = self.get_address(self.get_param(3), mode3);
                    self.memory[result_addr] = value1 * value2;
                    self.ip += 4;
                },
                (3, [mode1, Position, Position]) => {
                    let result_addr = self.get_address(self.get_param(1), mode1);
                    if let Some(input_value) = self.input.pop_front() {
                        self.memory[result_addr] = input_value;
                        self.ip += 2;
                    } else {
                        return DeviceStatus::WaitingInput
                    }
                },
                (4, [mode1, Position, Position]) => {
                    let value1 = self.get_value(self.get_param(1), mode1);
                    self.output.push_back(value1);
                    self.ip += 2;
                },
                (5, [mode1, mode2, Position]) => {
                    let value1 = self.get_value(self.get_param(1), mode1);
                    let addr = self.get_value(self.get_param(2), mode2);
                    if value1 != 0 {
                        self.ip = self.check_ip(addr);
                    } else {
                        self.ip += 3;
                    }
                },
                (6, [mode1, mode2, Position]) => {
                    let value1 = self.get_value(self.get_param(1), mode1);
                    let addr = self.get_value(self.get_param(2), mode2);
                    if value1 == 0 {
                        self.ip = self.check_ip(addr);
                    } else {
                        self.ip += 3;
                    }
                },
                (7, [mode1, mode2, mode3]) => {
                    let value1 = self.get_value(self.get_param(1), mode1);
                    let value2 = self.get_value(self.get_param(2), mode2);
                    let result_addr = self.get_address(self.get_param(3), mode3);
                    self.memory[result_addr] = if value1 < value2 { 1 } else { 0 };
                    self.ip += 4;
                },
                (8, [mode1, mode2, mode3]) => {
                    let value1 = self.get_value(self.get_param(1), mode1);
                    let value2 = self.get_value(self.get_param(2), mode2);
                    let result_addr = self.get_address(self.get_param(3), mode3);
                    self.memory[result_addr] = if value1 == value2 { 1 } else { 0 };
                    self.ip += 4;
                },
                (9, [mode1, Position, Position]) => {
                    let value1 = self.get_value(self.get_param(1), mode1);
                    self.relative_offset += value1 as isize;
                    self.ip += 2;
                },
                _ => panic!("Invalid pair of opcode and modes")
            }
        }
    }

    fn parse_opcode(opcode: usize) -> (usize, [ParamMode ; 3]) {
        (
            opcode % 100,
            [
                (opcode / 100 % 10).try_into().expect("Invalid parameter mode"),
                (opcode / 1000 % 10).try_into().expect("Invalid parameter mode"),
                (opcode / 10000 % 10).try_into().expect("Invalid parameter mode")
            ]
        )
    }

    fn check_ip(&self, ip: i64) -> usize {
        ip.try_into().ok()
            .filter(|&ip| ip < self.memory.len())
            .expect("Cannot set ip out of bounds")
    }

    fn get_param(&self, param_pos: usize) -> i64 {
        self.memory.get(self.ip + param_pos).cloned().expect("Can't fetch parameter")
    }

    fn get_address(&mut self, address: i64, mode: ParamMode) -> usize {
        let real_address = match mode {
            ParamMode::Position => address,
            ParamMode::Relative => address + self.relative_offset as i64,
            ParamMode::Immediate => panic!("Cannot use Immediate mode for indexing"),
        }.try_into().expect("Cannot write to negative address");

        self.require_memory_set(real_address);
        real_address
    }

    fn get_value(&mut self, param: i64, mode: ParamMode) -> i64 {
        match mode {
            ParamMode::Immediate => param,
            ParamMode::Relative | ParamMode::Position => {
                let addr = self.get_address(param, mode);
                self.require_memory_set(addr);
                self.memory[addr]
            }
        }
    }

    fn require_memory_set(&mut self, address: usize) {
        while self.memory.len() <= address {
            self.memory.push(0);
        }
    }

    pub fn reset(&mut self, base: &IntcodeDevice) {
        self.ip = base.ip;
        self.relative_offset = base.relative_offset;
        self.memory.clear();
        self.input.clear();
        self.output.clear();
        self.memory.extend(base.memory.iter().cloned());
        self.input.extend(base.input.iter().cloned());
        self.output.extend(base.output.iter().cloned());
    }

    pub fn write_str(&mut self, string: &str) {
        self.input.extend(string.bytes().map(|b| b as i64));
    }
}

impl FromStr for IntcodeDevice {
    type Err = String;
    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self {
            ip: 0,
            relative_offset: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            memory: input
                .trim()
                .split(',')
                .map(|n| n.parse::<i64>()
                    .map_err(|_| format!("Cannot parse {} as a positive integer", n))
                )
                .collect::<std::result::Result<_, _>>()?
        })
    }
}

#[derive(Debug)]
enum ParamMode { Immediate, Position, Relative }
impl TryFrom<usize> for ParamMode {
    type Error = Box<dyn std::error::Error>;
    fn try_from(mode: usize) -> Result<Self> {
        match mode {
            0 => Ok(Self::Position),
            1 => Ok(Self::Immediate),
            2 => Ok(Self::Relative),
            _ => Err("Invalid parameter mode in opcode".into())
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum DeviceStatus { Halt, WaitingInput }
