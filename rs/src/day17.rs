struct Memory {
    register_a: u64,
    register_b: u64,
    register_c: u64,
}

#[derive(Debug, Clone)]
struct Computer {
    rax: u64,
    rbx: u64,
    rcx: u64,
    instruction_ptr: usize,
    program: Vec<u8>,
}

pub fn part_one(input: &str) -> String {
    let mut computer = parse_input(input);
    computer.run_program()
}

pub fn part_two(input: &str) -> Option<u64> {
    let computer = parse_input(input);
    let mut memory = Memory {
        register_a: 0,
        register_b: computer.rbx,
        register_c: computer.rcx,
    };
    let program: Vec<u64> = computer.program.iter().map(|x| *x as u64).collect();
    search(&mut memory, program.len() - 1, &program)
}

fn search(memory: &mut Memory, iteration: usize, program: &[u64]) -> Option<u64> {
    for remainder in 0..8 {
        let multiplier = 8u64.pow(iteration as u32);

        if memory.register_a + multiplier * remainder < 8u64.pow(program.len() as u32 - 1) {
            continue;
        }

        let result = run_program(
            &mut Memory {
                register_a: memory.register_a + multiplier * remainder,
                register_b: memory.register_b,
                register_c: memory.register_c,
            },
            program,
        );

        if result[iteration] == program[iteration] {
            return if iteration == 0 {
                Some(memory.register_a + multiplier * remainder)
            } else if let Some(register_a) = search(
                &mut Memory {
                    register_a: memory.register_a + multiplier * remainder,
                    register_b: memory.register_b,
                    register_c: memory.register_c,
                },
                iteration - 1,
                program,
            ) {
                Some(register_a)
            } else {
                continue;
            };
        }
    }

    None
}

fn parse_input(input: &str) -> Computer {
    let (registers_string, program_string_dec) = input.split_once("\n\n").expect("invalid input");
    let registers: Vec<u64> = registers_string
        .lines()
        .map(|line| {
            let (_, val) = line.split_once(": ").expect("invalid input");
            val.parse().expect("invalid input")
        })
        .collect();
    assert_eq!(registers.len(), 3);
    let (_, program_string) = program_string_dec.split_once(": ").expect("invalid input");
    let program: Vec<u8> = program_string
        .trim()
        .split(",")
        .map(|val| val.parse().expect("invalid input"))
        .collect();

    Computer {
        rax: registers[0],
        rbx: registers[1],
        rcx: registers[2],
        instruction_ptr: 0,
        program,
    }
}

impl Computer {
    fn run_program(&mut self) -> String {
        let mut res = Vec::new();
        while self.instruction_ptr < self.program.len() {
            let opcode = self.program[self.instruction_ptr];
            match opcode {
                0 => {
                    self.instruction_ptr += 1;
                    let operand = match self.program[self.instruction_ptr] {
                        0..=3 => self.program[self.instruction_ptr] as u64,
                        4 => self.rax,
                        5 => self.rbx,
                        6 => self.rcx,
                        _ => unreachable!(),
                    };
                    let numerator = self.rax;
                    let denominator = (2 as u64).pow(operand as u32);
                    self.rax = numerator / denominator;
                    self.instruction_ptr += 1;
                }
                1 => {
                    self.instruction_ptr += 1;
                    self.rbx ^= self.program[self.instruction_ptr] as u64;
                    self.instruction_ptr += 1;
                }
                2 => {
                    self.instruction_ptr += 1;
                    let operand = match self.program[self.instruction_ptr] {
                        0..=3 => self.program[self.instruction_ptr] as u64,
                        4 => self.rax,
                        5 => self.rbx,
                        6 => self.rcx,
                        _ => unreachable!(),
                    };
                    self.rbx = operand % 8;
                    self.instruction_ptr += 1;
                }
                3 => {
                    if self.rax == 0 {
                        self.instruction_ptr += 1;
                        self.instruction_ptr += 1;
                        continue;
                    }
                    self.instruction_ptr += 1;
                    let jmp_to = self.program[self.instruction_ptr] as usize;
                    self.instruction_ptr = jmp_to;
                }
                4 => {
                    self.instruction_ptr += 1;
                    self.instruction_ptr += 1;

                    self.rbx = self.rbx ^ self.rcx;
                }
                5 => {
                    self.instruction_ptr += 1;
                    let operand = match self.program[self.instruction_ptr] {
                        0..=3 => self.program[self.instruction_ptr] as u64,
                        4 => self.rax,
                        5 => self.rbx,
                        6 => self.rcx,
                        _ => unreachable!(),
                    };
                    let out = operand % 8;
                    res.push(out.to_string());
                    self.instruction_ptr += 1;
                }
                6 => {
                    self.instruction_ptr += 1;
                    let operand = match self.program[self.instruction_ptr] {
                        0..=3 => self.program[self.instruction_ptr] as u64,
                        4 => self.rax,
                        5 => self.rbx,
                        6 => self.rcx,
                        _ => unreachable!(),
                    };
                    let numerator = self.rax;
                    let denominator = (2 as u64).pow(operand as u32);
                    self.rbx = numerator / denominator;
                    self.instruction_ptr += 1;
                }
                7 => {
                    self.instruction_ptr += 1;
                    let operand = match self.program[self.instruction_ptr] {
                        0..=3 => self.program[self.instruction_ptr] as u64,
                        4 => self.rax,
                        5 => self.rbx,
                        6 => self.rcx,
                        _ => unreachable!(),
                    };
                    let numerator = self.rax;
                    let denominator = (2 as u64).pow(operand as u32);
                    self.rcx = numerator / denominator;
                    self.instruction_ptr += 1;
                }
                _ => unreachable!("invalid program"),
            }
        }
        return res.join(",");
    }
}

fn combo_operand(memory: &Memory, operand: u64) -> u64 {
    match operand {
        literal if literal <= 3 => literal,
        4 => memory.register_a,
        5 => memory.register_b,
        6 => memory.register_c,
        _ => unreachable!(),
    }
}

fn run_program(memory: &mut Memory, program: &[u64]) -> Vec<u64> {
    let mut instruction_pointer = 0;
    let mut output = Vec::new();

    while instruction_pointer < program.len() - 1 {
        let opcode = program[instruction_pointer];
        let operand = program[instruction_pointer + 1];

        match opcode {
            0 => {
                memory.register_a >>= combo_operand(memory, operand);
                instruction_pointer += 2;
            }
            1 => {
                memory.register_b ^= operand;
                instruction_pointer += 2;
            }
            2 => {
                memory.register_b = combo_operand(memory, operand) & 7;
                instruction_pointer += 2;
            }
            3 => {
                if memory.register_a != 0 {
                    instruction_pointer = operand as usize;
                } else {
                    instruction_pointer += 2;
                }
            }
            4 => {
                memory.register_b ^= memory.register_c;
                instruction_pointer += 2;
            }
            5 => {
                output.push(combo_operand(memory, operand) & 7);
                instruction_pointer += 2;
            }
            6 => {
                memory.register_b = memory.register_a >> combo_operand(memory, operand);
                instruction_pointer += 2;
            }
            7 => {
                memory.register_c = memory.register_a >> combo_operand(memory, operand);
                instruction_pointer += 2;
            }
            _ => unreachable!(),
        }
    }
    output
}
