use std::env;
use std::fs::read_to_string;

use l::{Parser, MachineState};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let mut parser = Parser::new();

    for (line_num, line) in read_to_string(file_path).unwrap().lines().enumerate() {
        parser.parse_line(line, line_num);
    }

    let inputs = args[2..]
        .iter()
        .map(|s| usize::from_str_radix(s, 10).expect("Not an unsigned input"))
        .collect();

    let mut machine_state = MachineState::new(inputs, parser.instructions, parser.jump_table);

    machine_state.run();

    let y = machine_state.get_y();

    println!("{}", y);
}
