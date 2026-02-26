use std::collections::HashMap;

use crate::{
    jump_list::JumpList,
    primitives::{Instruction, Label, Register, Unsigned},
};

pub struct MachineState {
    pub register_values: HashMap<Register, Unsigned>,
    pub instructions: JumpList<Instruction, Label>,
    pub running: bool,
}

impl MachineState {
    pub fn new(
        inputs: Vec<usize>,
        instructions: JumpList<Instruction, Label>,
    ) -> Self {
        let mut register_values = HashMap::new();

        for (index, value) in inputs.iter().enumerate() {
            register_values.insert(Register::X(Unsigned(index)), Unsigned(*value));
        }

        Self {
            register_values,
            instructions,
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

                self.instructions.goto_next();
            }
            Instruction::Decrement(r) => {
                self.register_values
                    .entry(r.to_owned())
                    .and_modify(|u| u.decrement())
                    .or_insert(Unsigned(0));

                self.instructions.goto_next();
            }
            Instruction::Conditional(r, _) => {
                let value = self.register_values.get(&r).unwrap_or(&Unsigned(0)).0;
                if value != 0 {
                    self.instructions.goto_jump();
                } else {
                    self.instructions.goto_next();
                }
            }
            Instruction::Goto(_) => {
                self.instructions.goto_jump();
            }
            Instruction::Stop => {
                self.running = false;
            }
        }
    }

    pub fn run(&mut self) {
        while self.running {
            // println!("{:?} {:?}", self.register_values, self.instructions.get());
            // std::thread::sleep(std::time::Duration::from_millis(200));

            let instruction = match self.instructions.get() {
                Some(i) => i,
                None => {
                    self.running = false;
                    break;
                }
            };

            self.execute(&instruction);
        }
    }
}
