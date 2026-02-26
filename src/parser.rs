use std::str::Lines;

use crate::{
    jump_list::JumpList,
    primitives::{Instruction, Label, Register, Unsigned},
};

pub struct Parser {
    pub blank_lines: usize,
    pub instructions: JumpList<Instruction, Label>,
    pub max_x: Option<Unsigned>,
    pub max_z: Option<Unsigned>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            blank_lines: 0,
            // macro_requests: HashSet::new(),
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

    fn parse_label(&mut self, word: &String) -> Label {
        Label(word[1..word.len() - 1].to_owned())
    }

    fn parse_instruction(&mut self, words: Vec<String>) -> Instruction {
        let mut word_iter = words.iter();
        let word = word_iter.next().unwrap();

        // Parse Increment/Decrement
        if ["INCREMENT".to_string(), "DECREMENT".to_string()].contains(word) {
            let register_str = word_iter.next().unwrap();
            let register = self.parse_register(register_str);
            let instruction = match word.as_str() {
                "INCREMENT" => Instruction::Increment(register),
                "DECREMENT" => Instruction::Decrement(register),
                _ => panic!("Impossible state"),
            };
            instruction
        }
        // Parse Conditional Jump
        else if word == "IF" {
            let register_str = word_iter.next().unwrap();
            let register = self.parse_register(register_str);

            while *word_iter.next().unwrap() != "GOTO" {}
            let target = Label(word_iter.next().unwrap().to_string());

            Instruction::Conditional(register, target)
        }
        // Parse GOTO
        else if word == "GOTO" {
            let target = Label(word_iter.next().unwrap().to_string());

            Instruction::Goto(target)
        }
        // Parse STOP
        else if word == "STOP" {
            // self.instructions.push(Instruction::Stop);
            Instruction::Stop
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

    pub fn parse_lines(&mut self, lines: Lines<'_>) {

        for line in lines {

            // Parse blank line
            if line.starts_with("#") || line.trim() == "" {
                continue;
            }

            let words: Vec<_> = line.split_ascii_whitespace().map(str::to_string).collect();
            let first_word = words.get(0).unwrap();

            // Parse label
            // TODO: do cleaner
            let label = if first_word.starts_with("[") {
                Some(self.parse_label(first_word))
            } else {
                None
            };

            // Parse instruction
            let instruction = self.parse_instruction(match label {
                Some(_) => words[1..].to_vec(),
                None => words,
            });

            // Jump
            let jump = match &instruction {
                Instruction::Conditional(_r, l) => Some(l.clone()),
                Instruction::Goto(l)=> Some(l.clone()),
                _ => None,
            };

            self.instructions.append(instruction, label, jump);
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
