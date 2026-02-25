use std::{collections::HashMap, str::Lines};

use crate::primitives::{Instruction, Label, Register, Unsigned};

pub struct Parser {
    pub blank_lines: usize,
    // pub macro_requests: HashSet<String>,
    pub instructions: Vec<Instruction>,
    pub jump_table: HashMap<Label, usize>,
    pub max_x: Option<Unsigned>,
    pub max_z: Option<Unsigned>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            blank_lines: 0,
            // macro_requests: HashSet::new(),
            instructions: Vec::new(),
            jump_table: HashMap::new(),
            max_x: None,
            max_z: None,
        }
    }

    fn maybe_set_max_x(&mut self, x: &Unsigned) {
        self.max_x = match &self.max_x {
            Some(max_x) => Some(Unsigned(std::cmp::max(max_x.0, x.0))),
            None => Some(x.clone()),
        };
    }

    fn maybe_set_max_z(&mut self, z: &Unsigned) {
        self.max_z = match &self.max_z {
            Some(max_z) => Some(Unsigned(std::cmp::max(max_z.0, z.0))),
            None => Some(z.clone()),
        };
    }

    fn parse_register(&mut self, register_str: &str) -> Register {
        let (head, tail) = register_str.split_at(1);
        match head {
            "X" => {
                let unsigned = Unsigned(usize::from_str_radix(tail, 10).unwrap());
                self.maybe_set_max_x(&unsigned);
                Register::X(unsigned)
            }
            "Y" => Register::Y,
            "Z" => {
                let unsigned = Unsigned(usize::from_str_radix(tail, 10).unwrap());
                self.maybe_set_max_z(&unsigned);
                Register::Z(unsigned)
            }
            _ => panic!("Unable to parse register"),
        }
    }

    fn parse_instruction(&mut self, line: &str, line_num: usize) {
        // Parse blank line
        if line.starts_with("#") || line.trim() == "" {
            self.blank_lines += 1;
            return;
        }

        let words = line.split_ascii_whitespace();
        let mut words_enum = words.enumerate();

        while let Some((index, word)) = words_enum.next() {
            // Parse label
            if index == 0 && word.starts_with("[") {
                let label = Label(word[1..word.len() - 1].to_owned());
                self.jump_table.insert(label, line_num - self.blank_lines);
                continue;
            }
            // Parse Increment/Decrement
            if ["INCREMENT", "DECREMENT"].contains(&word) {
                let register_str = words_enum.next().unwrap().1;
                let register = self.parse_register(register_str);
                let instruction = match word {
                    "INCREMENT" => Instruction::Increment(register),
                    "DECREMENT" => Instruction::Decrement(register),
                    _ => panic!("Impossible state"),
                };
                self.instructions.push(instruction);
            }
            // Parse Conditional Jump
            else if word == "IF" {
                let register_str = words_enum.next().unwrap().1;
                let register = self.parse_register(register_str);

                while let Some((_, word)) = words_enum.next() {
                    if word == "GOTO" {
                        break;
                    }
                }
                let target = Label(words_enum.next().unwrap().1.to_string());

                self.instructions
                    .push(Instruction::Conditional(register, target));
            }
            // Parse GOTO
            else if word == "GOTO" {
                let target = Label(words_enum.next().unwrap().1.to_string());

                self.instructions.push(Instruction::Goto(target));
            }
            // Parse STOP
            else if word == "STOP" {
                self.instructions.push(Instruction::Stop);
            // }
            // // Parse macro
            // else if word.starts_with("!") {
            //     // For pattern in macro patterns
            //     // Find which one matches

            //     // // Expand macro while executing replacements
            //     // Empty parser with
            } else {
                panic!("Unable to process instruction begining with word {word}")
            }
        }
    }

    pub fn parse_lines(&mut self, lines: Lines<'_>) {
        for (line_num, line) in lines.enumerate() {
            // if line.starts_with("USEMACRO") {
            //     let mut words = line.split_ascii_whitespace();
            //     words.next(); // "USEMACRO"
            //     let macro_name = words
            //         .next()
            //         .expect("Expected macro name after 'USEMACRO'")
            //         .to_string();
            //     self.macro_requests.insert(macro_name);
            // } else {
                // Parse instruction
                self.parse_instruction(line, line_num);
            // }
        }
    }

    // fn resolve_macro_request(&mut self, name: String) -> Macro {
    //     let file_str = read_to_string(format!("{}.macro.l", name))
    //         .expect(&format!("Could not find macro {}", name));
    //     let mut lines = file_str.lines();

    //     let first_line = lines.next().expect("Expected line in file");
    //     let mut first_line_words = first_line.split_ascii_whitespace();

    //     if first_line_words.next().unwrap() != "MACRODEF" {
    //         panic!("Expected macro file to start with 'MACRODEF'");
    //     }

    //     let pattern = &first_line["MACRODEF ".len()..];

    //     Macro {
    //         name,
    //         pattern: pattern.to_string(),
    //     }
    // }
}
