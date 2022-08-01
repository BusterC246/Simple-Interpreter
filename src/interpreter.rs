use std::fs;

pub struct Interpreter {
    memory: [u8; 30000],
    index: usize,
}

impl Interpreter {
    pub fn process_instructions(&mut self, filename: &str) {
        let instructions: Vec<char> = fs::read_to_string(filename).expect("Error reading file.").chars().collect();

        let mut i = 0;

        while i < instructions.len() {
            match instructions[i] {
                '<' => {
                    if self.index == 0 {
                        self.index = self.memory.len() - 1;
                    } else {
                        self.index -= 1;
                    }
                }
                '>' => {
                    if self.index == self.memory.len() - 1 {
                        self.index = 0;
                    } else {
                        self.index += 1;
                    }
                }
                '+' => self.memory[self.index] += 1,
                '-' => self.memory[self.index] -= 1,
                '[' => {
                    if self.memory[self.index] == 0 {
                        while instructions[i] != ']' {
                            i += 1;
                        }
                    }
                }
                ']' => {
                    if self.memory[self.index] != 0 {
                        while instructions[i] != '[' {
                            i -= 1;
                        }
                    }
                }
                ',' => {
                    let mut input = String::new();

                    std::io::stdin().read_line(&mut input).expect("Error reading input");

                    self.memory[self.index] = input.chars().into_iter().next().unwrap() as u8;
                }
                '.' => print!("{}", self.memory[self.index] as char),
                _ => {}
            }

            i += 1;
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Interpreter { memory: [0; 30000], index: 0 }
    }
}