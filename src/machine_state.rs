use std::collections::HashMap;

use crate::primitives::{Instruction, Label, Register, Unsigned};

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
