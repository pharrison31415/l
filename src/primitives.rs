use std::{collections::HashMap, fmt, hash::Hash};

use regex::Regex;

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Unsigned(pub usize);

impl Unsigned {
    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn decrement(&mut self) {
        self.0 = self.0.saturating_sub(1);
    }
}

impl fmt::Debug for Unsigned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Unsigned {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for Unsigned {
    fn default() -> Self {
        Self(0)
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum Register {
    X(Unsigned),
    Y,
    Z(Unsigned),
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Label(pub String);

#[derive(Debug, Clone)]
pub enum Instruction {
    Increment(Register),
    Decrement(Register),
    Conditional(Register, Label),
    Goto(Label),
    Stop,
}

#[derive(Debug, Clone)]
pub struct Macro {
    pub name: String,
    pub file_name: String,
    pub re: Regex,
    pub lines: Vec<String>,
}

impl PartialEq for Macro {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Debug, Clone)]
pub struct MacroInvocation {
    pub l_macro: Macro,
    pub line: String,
    pub line_number: usize,
    pub invocation_file_path: String,
    pub captures_map: HashMap<String, String>,
}

impl PartialEq for MacroInvocation {
    fn eq(&self, other: &Self) -> bool {
        self.line_number == other.line_number
            && self.invocation_file_path == other.invocation_file_path
    }
}

impl Eq for MacroInvocation {}

impl Hash for MacroInvocation {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.line_number.hash(state);
        self.invocation_file_path.hash(state);
    }
}

#[derive(Debug, Clone)]
pub enum Executable {
    Instruction(Instruction),
    MacroCallSite(MacroInvocation),
}
