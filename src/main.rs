use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

use l::{Instruction, Label, MachineState};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let mut instructions: Vec<Instruction> = Vec::new();

    let mut jump_table: HashMap<Label, usize> = HashMap::new();

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
                jump_table.insert(label, line_num - blank_lines);
            }
            None => {}
        }
    }

    let inputs = args[2..]
        .iter()
        .map(|s| usize::from_str_radix(s, 10).expect("Not an unsigned input"))
        .collect();

    let mut machine_state = MachineState::new(inputs, instructions, jump_table);

    machine_state.run();

    let y = machine_state.get_y();

    println!("{}", y);
}
