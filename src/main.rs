use std::env;

use l::{machine_state::MachineState, parser::Parser};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let mut parser = Parser::new(file_path.clone());

    parser.parse_file();

    // dbg!(&parser.instructions);
    if args.get(2) == Some(&"-c".to_string()) {
        // TODO: fix
        let padding_num = ((parser.instructions.max_label_len + 4) / 4) * 4;
        // dbg!(&parser.instructions.max_label_len);
        // dbg!(&padding_num);

        parser.instructions.reset_pointer();
        while let Some((label_opt, executable)) = parser.instructions.get_with_label() {
            match label_opt {
                Some(l) => print!("{:<width$}", l, width = padding_num),
                None => print!("{}", " ".repeat(padding_num)),
            }
            match executable {
                l::primitives::Executable::MacroCallSite(inv) => {
                    println!("{}", inv.line);
                }
                l::primitives::Executable::Instruction(i) => match i {
                    l::primitives::Instruction::Increment(register) => {
                        println!("INCREMENT {}", register);
                    }
                    l::primitives::Instruction::Decrement(register) => {
                        println!("DECREMENT {}", register)
                    }
                    l::primitives::Instruction::Conditional(register, label) => {
                        println!("IF {} != 0 GOTO {}", register, label)
                    }
                    l::primitives::Instruction::Stop => {
                        println!("STOP");
                    }
                },
            }

            parser.instructions.goto_next();
        }

        return;
    }

    let inputs = args[2..]
        .iter()
        .map(|s| usize::from_str_radix(s, 10).expect("Not an unsigned input"))
        .collect();

    let mut machine_state = MachineState::new(inputs, parser.instructions);

    machine_state.run();

    let y = machine_state.get_y();

    println!("{}", y);
}
