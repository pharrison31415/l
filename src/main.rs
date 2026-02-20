use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

use l::{Instruction, Label, Register, Unsigned};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let mut instructions: Vec<Instruction> = Vec::new();

    let mut label_to_instruction_index: HashMap<Label, usize> = HashMap::new();

    let mut blank_lines = 0;

    for (line_num, line) in read_to_string(file_path).unwrap().lines().enumerate() {
        // Comment or blank line
        if line.starts_with("#") || line.trim() == "" {
            blank_lines += 1;
            continue;
        }

        // Parse line
        let (label_opt, instruction) = Instruction::parse(line);
        instructions.push(instruction);

        // Label map
        match label_opt {
            Some(label) => {
                label_to_instruction_index.insert(label, line_num - blank_lines);
            }
            None => {}
        }
    }

    let mut registers: HashMap<Register, Unsigned> = HashMap::new();

    for (i, str) in args[2..].iter().enumerate() {
        let register = Register::X(Unsigned(i));
        let val = Unsigned(usize::from_str_radix(str, 10).expect("Not an unsigned integer"));
        registers.insert(register, val);
    }

    let mut instruction_index: usize = 0;

    loop {
        // println!("{} {:?}", instruction_index, registers);
        // std::thread::sleep(std::time::Duration::from_millis(200));

        // Execute instruction
        match &instructions[instruction_index] {
            Instruction::Increment(register) => match registers.get_mut(&register) {
                Some(val) => val.increment(),
                None => {
                    registers.insert(register.clone(), Unsigned(1));
                    ()
                }
            },
            Instruction::Decrement(register) => match registers.get_mut(&register) {
                Some(val) => val.decrement(),
                None => {
                    registers.insert(register.clone(), Unsigned(1));
                    ()
                }
            },
            Instruction::Conditional(register, label) => {
                let value = registers.get(register).unwrap_or(&Unsigned(0)).0;
                if value != 0 {
                    instruction_index = *label_to_instruction_index
                        .get(label)
                        .expect("unexpected label");
                    continue;
                }
            }
            Instruction::Goto(label) => {
                instruction_index = *label_to_instruction_index
                    .get(label)
                    .expect("unexpected label");
                continue;
            }
            Instruction::Stop => {
                break;
            }
        }

        // Go to the next line
        instruction_index += 1;
    }

    let y_val = registers.get(&Register::Y).unwrap_or(&Unsigned(0));
    println!("{}", y_val);
}
