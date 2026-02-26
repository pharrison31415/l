use std::env;
use std::fs::read_to_string;

use l::{parser::Parser, machine_state::MachineState};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let mut parser = Parser::new();

    let file_str = read_to_string(file_path).unwrap();
    let lines = file_str.lines();

    parser.parse_lines(lines);

    let inputs = args[2..]
        .iter()
        .map(|s| usize::from_str_radix(s, 10).expect("Not an unsigned input"))
        .collect();

    let mut machine_state = MachineState::new(inputs, parser.instructions);

    machine_state.run();

    let y = machine_state.get_y();

    println!("{}", y);
}
