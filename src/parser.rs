use std::{collections::HashSet, fs::read_to_string, slice::Iter, str::Lines};

use crate::{
    jump_list::JumpList,
    primitives::{Executable, Instruction, Label, Macro, Register, Unsigned},
};

pub struct Parser {
    pub macros_to_resolve: HashSet<Macro>,
    pub instructions: JumpList,
    pub max_x: Option<Unsigned>,
    pub max_z: Option<Unsigned>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            macros_to_resolve: HashSet::new(),
            instructions: JumpList::new(),
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

    fn parse_label(&mut self, word: &str) -> Label {
        Label(word[1..word.len() - 1].to_owned())
    }

    fn parse_instruction(&mut self, mut word_iter: Iter<'_, &str>) -> Instruction {
        let word = *word_iter.next().unwrap();

        match word {
            // Parse Increment/Decrement
            "INCREMENT" | "DECREMENT" => {
                let register_str = word_iter.next().unwrap();
                let register = self.parse_register(register_str);
                let instruction = match word {
                    "INCREMENT" => Instruction::Increment(register),
                    "DECREMENT" => Instruction::Decrement(register),
                    _ => panic!("Impossible state"),
                };
                instruction
            }
            // Parse Conditional Jump
            "IF" => {
                let register_str = word_iter.next().unwrap();
                let register = self.parse_register(register_str);

                while *word_iter.next().unwrap() != "GOTO" {}
                let target = Label(word_iter.next().unwrap().to_string());

                Instruction::Conditional(register, target)
            }
            // Parse GOTO
            "GOTO" => {
                let target = Label(word_iter.next().unwrap().to_string());

                Instruction::Goto(target)
            }
            // Parse STOP
            "STOP" => Instruction::Stop,
            _ => panic!("Unable to process instruction begining with word {word}"),
        }
    }

    pub fn parse_lines(&mut self, lines: Lines<'_>) {
        for line in lines {
            // Parse blank line
            if line.starts_with("#") || line.trim() == "" {
                continue;
            }

            let words: Vec<_> = line.split_ascii_whitespace().collect();
            let first_word = *words.get(0).expect("Expected word on line");

            // Parse USEMACRO declaration
            if first_word == "USEMACRO" {
                self.resolve_macro_request(words[1..].concat());
                continue;
            }

            // Parse label
            let label = first_word
                .starts_with('[')
                .then(|| self.parse_label(first_word));

            // Parse macro
            let possible_macro_word = match label {
                Some(_) => *words.get(1).expect("Expected word after label"),
                None => *words.get(0).expect("Expected word on line"),
            };
            if possible_macro_word.starts_with("!") {
                // self.parse_macro(...)
                todo!("Macro expansion not yet implemented");
            }

            // Parse instruction
            let instruction = self.parse_instruction(match label {
                Some(_) => words[1..].iter(),
                None => words.iter(),
            });

            // Jump
            let jump = match &instruction {
                Instruction::Conditional(_r, l) => Some(l.clone()),
                Instruction::Goto(l) => Some(l.clone()),
                _ => None,
            };

            self.instructions
                .append(Executable::Instruction(instruction), label, jump);
        }
    }

    fn resolve_macro_request(&mut self, name: String) -> Macro {
        let file_str = read_to_string(format!("{}.macro.l", name))
            .expect(&format!("Could not find macro {}", name));
        let mut lines = file_str.lines();

        let first_line = lines
            .next()
            .expect(&format!("Expected line in file {}.macro.l", name));
        let mut first_line_words = first_line.split_ascii_whitespace();

        if first_line_words.next().unwrap() != "MACRODEF" {
            panic!(
                "Expected macro file {}.macro.l to start with 'MACRODEF'",
                name
            );
        }

        let pattern = &first_line["MACRODEF ".len()..];

        Macro {
            name,
            pattern: pattern.to_string(),
        }
    }
}
