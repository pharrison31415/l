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

impl Instruction {
    pub fn parse(line: &str) -> (Option<Label>, Self) {
        let mut label: Option<Label> = None;

        let words = line.split_ascii_whitespace();
        let mut words_enum = words.enumerate();

        while let Some((index, word)) = words_enum.next() {
            // Handle comment
            if word.starts_with("#") {}

            // Handle label
            if index == 0 && word.starts_with("[") {
                label = Some(Label(word[1..word.len() - 1].to_owned()));
                continue;
            }

            // Handle Increment/Decrement
            if ["INCREMENT", "DECREMENT"].contains(&word) {
                let register_str = words_enum.next().unwrap().1;
                let register_arr = &register_str[0..1];
                let register = match register_arr {
                    "X" => Register::X(Unsigned(
                        usize::from_str_radix(&register_str[1..], 10).unwrap(),
                    )),
                    "Y" => Register::Y,
                    "Z" => Register::Z(Unsigned(
                        usize::from_str_radix(&register_str[1..], 10).unwrap(),
                    )),
                    _ => panic!("Bad register"),
                };

                match word {
                    "INCREMENT" => return (label, Instruction::Increment(register)),
                    "DECREMENT" => return (label, Instruction::Decrement(register)),
                    _ => panic!("Impossible state"),
                }
            }

            // Handle Conditional Jump
            if word == "IF" {
                let register_str = words_enum.next().unwrap().1;
                let register_arr = &register_str[0..1];
                let register = match register_arr {
                    "X" => Register::X(Unsigned(
                        usize::from_str_radix(&register_str[1..], 10).unwrap(),
                    )),
                    "Y" => Register::Y,
                    "Z" => Register::Z(Unsigned(
                        usize::from_str_radix(&register_str[1..], 10).unwrap(),
                    )),
                    _ => panic!("Bad register"),
                };

                while let Some((_, word)) = words_enum.next() {
                    if word == "GOTO" {
                        break;
                    }
                }
                let goto = Label(words_enum.next().unwrap().1.to_string());

                return (label, Instruction::Conditional(register, goto));
            }

            // Handle GOTO
            if word == "GOTO" {
                let target = Label(words_enum.next().unwrap().1.to_string());
                return (label, Instruction::Goto(target));
            }

            // Handle STOP
            if word == "STOP" {
                return (label, Instruction::Stop);
            }

            panic!("unable to process instruction begining with word {word}")
        }
        (Some(Label(String::from("hi"))), Instruction::Stop)
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
