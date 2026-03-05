use std::collections::HashMap;

use crate::{
    jump_list::JumpList,
    primitives::{Executable, Instruction, Register, Unsigned},
};

pub struct MachineState {
    pub register_values: HashMap<Register, Unsigned>,
    pub jump_list: JumpList,
    pub running: bool,
}

impl MachineState {
    pub fn new(inputs: Vec<usize>, instructions: JumpList) -> Self {
        let mut register_values = HashMap::new();

        for (index, value) in inputs.iter().enumerate() {
            register_values.insert(Register::X(Unsigned(index)), Unsigned(*value));
        }

        Self {
            register_values,
            jump_list: instructions,
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

                self.jump_list.goto_next();
            }
            Instruction::Decrement(r) => {
                self.register_values
                    .entry(r.to_owned())
                    .and_modify(|u| u.decrement())
                    .or_insert(Unsigned(0));

                self.jump_list.goto_next();
            }
            Instruction::Conditional(r, _) => {
                let value = self.register_values.get(&r).unwrap_or(&Unsigned(0)).0;
                if value != 0 {
                    self.jump_list.goto_jump();
                } else {
                    self.jump_list.goto_next();
                }
            }
            Instruction::Goto(_) => {
                self.jump_list.goto_jump();
            }
            Instruction::Stop => {
                self.running = false;
            }
        }
    }

    pub fn run(&mut self) {
        self.jump_list.reset_pointer();

        while self.running {
            // println!("{:?} {:?}", self.register_values, self.jump_list.get());
            // std::thread::sleep(std::time::Duration::from_millis(200));

            let instruction = match self.jump_list.get() {
                Some(Executable::Instruction(i)) => i,
                Some(Executable::MacroCallSite(_)) => panic!("MacroCallSite in jump_list"),
                None => {
                    self.running = false;
                    break;
                }
            };

            self.execute(&instruction);
        }
    }
}
