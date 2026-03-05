use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
    slice::Iter,
};

use regex::Regex;

use crate::{
    jump_list::JumpList,
    primitives::{Executable, Instruction, Label, Macro, MacroCallSite, Register, Unsigned},
};

pub struct Parser {
    pub file_path: String,
    pub requested_macros: Vec<Macro>,
    pub macro_expansion_queue: VecDeque<MacroCallSite>,
    pub instructions: JumpList,
    // pub max_x: Option<Unsigned>,
    pub max_z: isize,
}

impl Parser {
    pub fn new(file_path: String) -> Self {
        Self {
            file_path: file_path.clone(),
            requested_macros: Vec::new(),
            macro_expansion_queue: VecDeque::new(),
            instructions: JumpList::new(file_path),
            // max_x: None,
            max_z: -1,
        }
    }

    // pub fn new_subparser(max_z: isize) -> Self {
    //     let mut new = Self::new();
    //     new.max_z = max_z;
    //     new
    // }

    // fn maybe_set_max_x(&mut self, x: &Unsigned) {
    //     self.max_x = match &self.max_x {
    //         Some(max_x) => Some(Unsigned(std::cmp::max(max_x.0, x.0))),
    //         None => Some(x.clone()),
    //     };
    // }

    fn maybe_set_max_z(&mut self, z: &Unsigned) {
        self.max_z = self
            .max_z
            .max(isize::try_from(z.0).expect("z.0 doesn't fit in isize"));
        // self.max_z = match &self.max_z {
        //     Some(max_z) => Some(Unsigned(std::cmp::max(max_z.0, z.0))),
        //     None => Some(z.clone()),
        // };
    }

    fn parse_register(&mut self, register_str: &str) -> Register {
        let (head, tail) = register_str.split_at(1);
        match head {
            "X" => {
                let unsigned = Unsigned(usize::from_str_radix(tail, 10).unwrap());
                // self.maybe_set_max_x(&unsigned);
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
                let target = self.parse_label(word_iter.next().unwrap());

                Instruction::Conditional(register, target)
            }
            // Parse GOTO
            "GOTO" => {
                let target = self.parse_label(word_iter.next().unwrap());

                Instruction::Goto(target)
            }
            // Parse STOP
            "STOP" => Instruction::Stop,
            _ => panic!("Unable to process instruction begining with word {word}"),
        }
    }

    pub fn parse_lines<'a, I>(&mut self, lines: I)
    where
        I: Iterator<Item = &'a str>,
    {
        for (idx, line) in lines.enumerate() {
            // Parse blank line
            if line.starts_with("#") || line.trim() == "" {
                continue;
            }

            let words: Vec<_> = line.split_ascii_whitespace().collect();
            let first_word = *words.get(0).expect("Expected word on line");

            // Parse USEMACRO declaration
            if first_word == "USEMACRO" {
                let l_macro = self.build_macro(words[1..].concat());
                self.requested_macros.push(l_macro);
                continue;
            }

            // Parse label
            let label = first_word
                .starts_with('[')
                .then(|| self.parse_label(first_word));

            let possible_macro_word = match label {
                Some(_) => *words.get(1).expect("Expected word after label"),
                None => *words.get(0).expect("Expected word on line"),
            };
            // Check if line is a macro call site
            let exec = if possible_macro_word.starts_with("!") {
                // Find which macro this call site matches
                let (l_macro, captures_map) = self
                    .find_macro_from_call_site(&line)
                    .expect(&format!("Could not match macro: {}", line));

                // Form call site
                let call_site = MacroCallSite {
                    l_macro: l_macro.clone(),
                    invocation_file_path: self.file_path.clone(),
                    line: line.to_string(),
                    line_number: idx,
                    captures_map,
                };

                // Insert into expansion queue
                self.macro_expansion_queue.push_back(call_site.clone());
                Executable::MacroCallSite(call_site)
            } else {
                // Parse instruction
                Executable::Instruction(self.parse_instruction(match label {
                    Some(_) => words[1..].iter(),
                    None => words.iter(),
                }))
            };

            // Jump
            let jump = match &exec {
                Executable::Instruction(Instruction::Conditional(_, l)) => Some(l.clone()),
                Executable::Instruction(Instruction::Goto(l)) => Some(l.clone()),
                _ => None,
            };

            self.instructions.append(exec, label, jump);
        }

        // Macro expansion queue must be emptied
        self.empty_macro_expansion_queue();
    }

    fn empty_macro_expansion_queue(&mut self) {
        let mut queue_i = 0;
        while let Some(mut call_site) = self.macro_expansion_queue.pop_front() {
            // Update labels and Z registers
            call_site.l_macro.lines.iter_mut().for_each(|line| {
                // Update z vars
                for (u_string, idx) in find_z_vars(&line).iter().rev() {
                    let replacement_unsigned =
                        self.max_z + 1 + isize::from_str_radix(u_string, 10).unwrap();
                    line.replace_range(
                        idx..&(idx + &u_string.len()),
                        &replacement_unsigned.to_string(),
                    );
                }
                // Update labels
                let label_regex = Regex::new(r"\[(?<lab>\S+)\]").unwrap();
                *line = label_regex
                    .replace_all(line, format!(r"[$lab-{}]", queue_i))
                    .into_owned();
            });

            // dbg!(&call_site);

            // Replace all captures in macro lines
            let replaced_lines: Vec<_> = call_site
                .l_macro
                .lines
                .iter()
                .map(|line| {
                    // dbg!(&line);
                    let mut out = line.clone();
                    for (k, v) in &call_site.captures_map {
                        let curlied_k = "\\{".to_owned() + &k + "}";
                        out = Regex::new(&curlied_k)
                            .unwrap()
                            .replace_all(&out, v)
                            .to_string();
                    }
                    out
                })
                .collect();

            // Parse macro lines
            let mut sub_parser = Self::new(call_site.l_macro.file_name.clone());
            sub_parser.parse_lines(replaced_lines.iter().map(|s| s.as_str()));
            self.instructions.expand_macro(call_site, sub_parser.instructions);
            queue_i += 1;
        }
    }

    pub fn parse_file(&mut self) {
        let file_str = read_to_string(&self.file_path).unwrap();
        let lines = file_str.lines();

        self.parse_lines(lines);
    }

    fn build_macro(&mut self, name: String) -> Macro {
        let file_str = read_to_string(format!("{}.macro.l", name))
            .expect(&format!("Could not find macro {}", name));
        let mut lines = file_str.lines();

        let first_line = lines
            .next()
            .expect(&format!("Expected line in file {}.macro.l", name));
        let mut first_line_words = first_line.split_ascii_whitespace();

        // Ensure MACRODEF
        if first_line_words.next().unwrap() != "MACRODEF" {
            panic!(
                "Expected macro file {}.macro.l to start with 'MACRODEF'",
                name
            );
        }

        let macrodef_pattern = &first_line["MACRODEF ".len()..];

        // Find all strings wrapped in curlies. These are the arguments to the macro
        let macrodef_re = Regex::new(r"\{(?<arg>\S+)}").unwrap();
        let macro_re_string = macrodef_re
            .replace_all(&macrodef_pattern, r"(?<$arg>\S+)")
            .into_owned();
        let macro_re = Regex::new(&macro_re_string).unwrap();

        // Build file_name. This is sloppy and needs fixing
        let file_name = name.clone() + ".macro.l";

        Macro {
            name,
            file_name,
            re: macro_re,
            lines: lines.map(str::to_string).collect(),
        }
    }

    fn find_macro_from_call_site(&self, line: &str) -> Option<(&Macro, HashMap<String, String>)> {
        for l_macro in &self.requested_macros {
            if let Some(captures) = l_macro.re.captures(&line) {
                // This is totally a code smell. I should figure out how to deal with lifetimes
                let captures_map: HashMap<String, String> = l_macro
                    .re
                    .capture_names()
                    .flatten()
                    .filter_map(|n| Some((n.to_string(), captures.name(n)?.as_str().to_string())))
                    .collect();

                return Some((l_macro, captures_map));
            }
        }
        None
    }
}

#[derive(Debug)]
enum LookingFor {
    Z,
    CloseCurly,
    Digit,
}

// This should be a regex with a look-around group
fn find_z_vars(line: &String) -> Vec<(String, usize)> {
    let mut out = Vec::new();
    let mut look = LookingFor::Z;
    let mut first_digit = false;
    let mut current_finding = (String::new(), 0);
    for (idx, character) in line.chars().enumerate() {
        // println!("char: {}, look: {:?}", character, look);
        match (&look, character) {
            (_, '{') => {
                current_finding = (String::new(), 0);
                look = LookingFor::CloseCurly;
            }
            (LookingFor::Z, 'Z') => {
                look = LookingFor::Digit;
                first_digit = true;
            }
            (LookingFor::CloseCurly, '}') => {
                look = LookingFor::Z;
            }
            (LookingFor::Digit, c) => {
                if c.is_digit(10) {
                    if first_digit {
                        current_finding.1 = idx;
                    }
                    current_finding.0.push(c);
                } else if c.is_whitespace() {
                    out.push(current_finding);
                    current_finding = (String::new(), idx);
                    look = LookingFor::Z;
                } else {
                    panic!("Unable to find Z fars in line: {}", line);
                }
            }
            (_, _) => {}
        }
    }

    if matches!(look, LookingFor::Digit) {
        out.push(current_finding);
    }
    out
}
