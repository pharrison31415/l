use std::env;

use l::{machine_state::MachineState, parser::Parser};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let mut parser = Parser::new(file_path.clone());

    parser.parse_file();

    // dbg!(&parser.instructions);

    let inputs = args[2..]
        .iter()
        .map(|s| usize::from_str_radix(s, 10).expect("Not an unsigned input"))
        .collect();

    let mut machine_state = MachineState::new(inputs, parser.instructions);

    machine_state.run();

    let y = machine_state.get_y();

    println!("{}", y);
}
