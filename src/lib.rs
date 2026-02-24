use std::{collections::HashMap, fmt};

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Unsigned(pub usize);

impl Unsigned {
    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn decrement(&mut self) {
        self.0 = self.0.saturating_sub(1);
    }
}

impl fmt::Debug for Unsigned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Unsigned {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for Unsigned {
    fn default() -> Self {
        Self(0)
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum Register {
    X(Unsigned),
    Y,
    Z(Unsigned),
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Label(String);

#[derive(Debug, Clone)]
pub enum Instruction {
    Increment(Register),
    Decrement(Register),
    Conditional(Register, Label),
    Goto(Label),
    Stop,
}

pub struct Parser {
    pub blank_lines: usize,
    pub instructions: Vec<Instruction>,
    pub jump_table: HashMap<Label, usize>,
    pub max_x: Option<Unsigned>,
    pub max_z: Option<Unsigned>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            blank_lines: 0,
            instructions: Vec::new(),
            jump_table: HashMap::new(),
            max_x: None,
            max_z: None,
        }
    }

    fn maybe_set_max_x(&mut self, x: &Unsigned) {
        self.max_x = match &self.max_x {
            Some(max_x) => Some(Unsigned(std::cmp::max(max_x.0, x.0))),
            None => Some(x.clone()),
        };
    }

    fn maybe_set_max_z(&mut self, z: &Unsigned) {
        self.max_z = match &self.max_z {
            Some(max_z) => Some(Unsigned(std::cmp::max(max_z.0, z.0))),
            None => Some(z.clone()),
        };
    }

    fn parse_register(&mut self, register_str: &str) -> Register {
        let (head, tail) = register_str.split_at(1);
        match head {
            "X" => {
                let unsigned = Unsigned(usize::from_str_radix(tail, 10).unwrap());
                self.maybe_set_max_x(&unsigned);
                Register::X(unsigned)
            }
            "Y" => Register::Y,
            "Z" => {
                let unsigned = Unsigned(usize::from_str_radix(tail, 10).unwrap());
                self.maybe_set_max_z(&unsigned);
                Register::Z(unsigned)
            }
            _ => panic!("Unable to parse register"),
        }
    }

    pub fn parse_line(&mut self, line: &str, line_num: usize) {
        // Parse blank line
        if line.starts_with("#") || line.trim() == "" {
            self.blank_lines += 1;
            return;
        }

        let words = line.split_ascii_whitespace();
        let mut words_enum = words.enumerate();

        while let Some((index, word)) = words_enum.next() {
            // Parse label
            if index == 0 && word.starts_with("[") {
                let label = Label(word[1..word.len() - 1].to_owned());
                self.jump_table.insert(label, line_num - self.blank_lines);
                continue;
            }
            // Parse Increment/Decrement
            if ["INCREMENT", "DECREMENT"].contains(&word) {
                let register_str = words_enum.next().unwrap().1;
                let register = self.parse_register(register_str);
                let instruction = match word {
                    "INCREMENT" => Instruction::Increment(register),
                    "DECREMENT" => Instruction::Decrement(register),
                    _ => panic!("Impossible state"),
                };
                self.instructions.push(instruction);
            }
            // Parse Conditional Jump
            else if word == "IF" {
                let register_str = words_enum.next().unwrap().1;
                let register = self.parse_register(register_str);

                while let Some((_, word)) = words_enum.next() {
                    if word == "GOTO" {
                        break;
                    }
                }
                let target = Label(words_enum.next().unwrap().1.to_string());

                self.instructions
                    .push(Instruction::Conditional(register, target));
            }
            // Parse GOTO
            else if word == "GOTO" {
                let target = Label(words_enum.next().unwrap().1.to_string());

                self.instructions.push(Instruction::Goto(target));
            }
            // Parse STOP
            else if word == "STOP" {
                self.instructions.push(Instruction::Stop);
            } else {
                panic!("Unable to process instruction begining with word {word}")
            }
        }
    }
}

pub struct MachineState {
    pub program_counter: usize,
    pub register_values: HashMap<Register, Unsigned>,
    pub instructions: Vec<Instruction>,
    pub jump_table: HashMap<Label, usize>,
    pub running: bool,
}

impl MachineState {
    pub fn new(
        inputs: Vec<usize>,
        instructions: Vec<Instruction>,
        jump_table: HashMap<Label, usize>,
    ) -> Self {
        let mut register_values = HashMap::new();

        for (index, value) in inputs.iter().enumerate() {
            register_values.insert(Register::X(Unsigned(index)), Unsigned(*value));
        }

        Self {
            program_counter: 0,
            register_values,
            instructions,
            jump_table,
            running: true,
        }
    }

    pub fn get_y(&self) -> Unsigned {
        // TODO: unwrap or default
        self.register_values
            .get(&Register::Y)
            .unwrap_or(&Unsigned(0))
            .clone()
    }

    fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Increment(r) => {
                self.register_values
                    .entry(r.to_owned())
                    .and_modify(|u| u.increment())
                    .or_insert(Unsigned(1));

                self.program_counter += 1;
            }
            Instruction::Decrement(r) => {
                self.register_values
                    .entry(r.to_owned())
                    .and_modify(|u| u.decrement())
                    .or_insert(Unsigned(0));

                self.program_counter += 1;
            }
            Instruction::Conditional(r, l) => {
                let value = self.register_values.get(&r).unwrap_or(&Unsigned(0)).0;
                self.program_counter = if value != 0 {
                    *self.jump_table.get(&l).expect("unexpected label")
                } else {
                    self.program_counter + 1
                };
            }
            Instruction::Goto(l) => {
                self.program_counter = *self.jump_table.get(&l).expect("unexpected label");
            }
            Instruction::Stop => {
                self.running = false;
            }
        }
    }

    pub fn run(&mut self) {
        while self.running {
            // println!("{} {:?}", self.program_counter, self.register_values);
            // std::thread::sleep(std::time::Duration::from_millis(200));

            let instruction = match self.instructions.get(self.program_counter) {
                Some(i) => i.clone(),
                None => {
                    self.running = false;
                    break;
                }
            };

            self.execute(&instruction);
        }
    }
}
