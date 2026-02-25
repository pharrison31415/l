use std::fmt;

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