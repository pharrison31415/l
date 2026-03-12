---

title: An Interpreter for Computability Theory Written the Hard Way
# theme: ./dark.json
author: Paul Harrison
paging: ssh [IP] -p 53531

---

# An Interpreter for Computability Theory  
## Written the Hard Way

&nbsp;

**Follow along**: `ssh [IP] -p 53531` (Warning: quiz spoilers ahead)

---

# Me

Paul Harrison

&nbsp;

SRE @ Lucid Software

B.S. in Computer Science from USU

&nbsp;

Started learning Rust around the end of 2024

---

# Why Rust?

I became fascinated with Rust because of its:

- simple algebraic type system
- strong compile-time guarantees

&nbsp;

I wanted:

- to build an interpreter 
- to practice writing code **without AI**

---

# Computability Theory

Asks:

> What kinds of problems can computers _theoretically_ solve?

This is the field that gives us the halting problem

---

# Computability Theory

Asks:

> What kinds of problems can computers _theoretically_ solve?

This is the field that gives us the halting problem


&nbsp;

Uses **programming formalisms** to model computation precisely

Examples of programming formalisms:
- Turing Machines
- lambda calculus

---

# The Lambda Calculus

Elegant, but hard to read

Examples:

- `λf. λx. f (f x)`
- `(λa. λb. a (b b)) (λb. λc. c) (λd. d)`

&nbsp;

Mathematically elegant, but **hard to read**

Not great for teaching beginners

---

# L

L is another **programming formalism**.

Properties:

- **Turing complete** (just like the lambda calculus)
- Tiny instruction set
- Reads more like **assembly**

From *Computability, Complexity, and Languages* by Davis, Weyuker, and Sigal

&nbsp;

Used in my computability theory course at USU.


---

# L - Register Machine

Registers hold **natural numbers**

Register types:

- `X0`, `X1`, `X2`, ... input registers
- `Z0`, `Z1`, `Z2`, ... temporary registers
- `Y` output register; the computation result

---

# L - Register Machine

Registers hold **natural numbers**

Register types:

- `X0`, `X1`, `X2`, ... input registers
- `Z0`, `Z1`, `Z2`, ... temporary registers
- `Y` output register; the computation result


&nbsp;

All registers start at **0**, except input registers.

---

# L code

Instructions execute line by line, unless a jump occurs.

---

# L code

Instructions execute line by line, unless a jump occurs.


## Instructions

- **Increment**         Add one to register value in-place  Eg: `INCREMENT Z0`
- **Decrement**         Subtract one                        Eg: `DECREMENT Y`
- **Conditional Jump**  If nonzero, jump to a label         Eg: `IF X1 != 0 GOTO [MYLABEL]`
- **Stop**              Halt execution                      Eg: `STOP`

---

# L code

Instructions execute line by line, unless a jump occurs.


## Instructions

- **Increment**         Add one to register value in-place  Eg: `INCREMENT Z0`
- **Decrement**         Subtract one                        Eg: `DECREMENT Y`
- **Conditional Jump**  If nonzero, jump to a label         Eg: `IF X1 != 0 GOTO [MYLABEL]`
- **Stop**              Halt execution                      Eg: `STOP`


## Labels

Identify instructions with bracket notation

Eg: `[MYLABEL] INCREMENT Y`

---

# Exercise 0

What does this program return?

```
INCREMENT Y
INCREMENT Y
INCREMENT Y
```

---

# Exercise 0

What does this program return?

```
INCREMENT Y
INCREMENT Y
INCREMENT Y
```


Always returns:

```
3
```

---

# Exercise 1

What does this compute?
```
    IF X0 != 0 GOTO [A]
    INCREMENT Y
[A] STOP
```

---

# Exercise 1

What does this compute?
```
    IF X0 != 0 GOTO [A]
    INCREMENT Y
[A] STOP
```


This program computes:

```
f(x) = if x == 0: 1
       else 0
```

---

# Exercise 2

Implement the identity program:

Return the value of `X0`.

*Hint: How could you move the value from X0 to Y one step at a time?*
---

# Exercise 2

Implement the identity program:

Return the value of `X0`.

*Hint: How could you move the value from X0 to Y one step at a time?*

```
[A] IF X0 != 0 GOTO [B]
    STOP
[B] DECREMENT X0
    INCREMENT Y
    IF Y != 0 GOTO [A]
```

---

# Something Always Bugged Me

In class we **proved** functions or programs were computable.

But we never had the opportunity to **run them**.

---

# First Interpreter Implementation

The initial version was simple:

- Global state
- All in `main.rs`
- `unwrap()`s everywhere

---

# First Interpreter Implementation

The initial version was simple:

- Global state
- All in `main.rs`
- `unwrap()`s everywhere


## Parser

Read files line by line, parsing instructions, adding them to `instructions: Vec<Instruction>`. 

Labeled instructions are added to a `jump_table: HashMap<Label, usize>`, where the value is an index in `instructions`.

---

# First Interpreter Implementation

The initial version was simple:

- Global state
- All in `main.rs`
- `unwrap()`s everywhere


## Parser

Read files line by line, parsing instructions, adding them to `instructions: Vec<Instruction>`. 

Labeled instructions are added to a `jump_table: HashMap<Label, usize>`, where the value is an index in `instructions`.


## Executor

Register values stored in a `HashMap<Register, Unsigned>`. If a register is not present, return a value of zero.

Initialize input registers (`X0`, `X1`, etc) from system args.

`program_counter: usize` is the index in the parsed `instructions` at which we are processing. 

Execute `instructions` sequentially, updating registers and using `jump_table` to set `program_counter` on Conditional Jump instructions (increment it for all other instructions).

---

# First Interpreter Implementation

The initial version was simple:

- Global state
- All in `main.rs`
- `unwrap()`s everywhere


## Parser

Read files line by line, parsing instructions, adding them to `instructions: Vec<Instruction>`. 

Labeled instructions are added to a `jump_table: HashMap<Label, usize>`, where the value is an index in `instructions`.


## Executor

Register values stored in a `HashMap<Register, Unsigned>`. If a register is not present, return a value of zero.

Initialize input registers (`X0`, `X1`, etc) from system args.

`program_counter: usize` is the index in the parsed `instructions` at which we are processing. 

Execute `instructions` sequentially, updating registers and using `jump_table` to set `program_counter` on Conditional Jump instructions (increment it for all other instructions).


**When done, print the value of `Y`**

---

This language works (turing complete, after all).

---

This language works (turing complete, after all).


But it **sucks**.

---

This language works (turing complete, after all).


But it **sucks**.


We have to write programs like this:

---

This language works (turing complete, after all).


But it **sucks**.


We have to write programs like this:


```
[A-1]   IF X0 != 0 GOTO [B-1]       [A-11]  IF X1 != 0 GOTO [B-11]      [E-0]   INCREMENT Y
        INCREMENT Z2                        INCREMENT Z7                        DECREMENT Y
        IF Z2 != 0 GOTO [C-1]               IF Z7 != 0 GOTO [C-11]

[B-1]   DECREMENT X0                [B-11]  DECREMENT X1                [A-0]   IF Z0 != 0 GOTO [B-0]
        INCREMENT Y                         INCREMENT Z0                        INCREMENT Z11
        INCREMENT Z1                        INCREMENT Z6                        IF Z11 != 0 GOTO [END-0]
        INCREMENT Z3                        INCREMENT Z8
        IF Z3 != 0 GOTO [A-1]               IF Z8 != 0 GOTO [A-11]

[C-1]   IF Z1 != 0 GOTO [D-1]       [C-11]  IF Z6 != 0 GOTO [D-11]      [B-0]   DECREMENT Z0
        INCREMENT Z4                        INCREMENT Z9                        INCREMENT Y
        IF Z4 != 0 GOTO [END-1]             IF Z9 != 0 GOTO [E]                 INCREMENT Z12
                                                                                IF Z12 != 0 GOTO [A-0]

[D-1]   DECREMENT Z1                [D-11]  DECREMENT Z6                [END-0] STOP
        INCREMENT X0                        INCREMENT X1
        INCREMENT Z5                        INCREMENT Z10
        IF Z5 != 0 GOTO [C-1]               IF Z10 != 0 GOTO [C-11]

[END-1] INCREMENT Y                 [A-12]  DECREMENT Z0
        DECREMENT Y                         IF Z0 != 0 GOTO [A-12]

                                    [A-14]  DECREMENT Z6
                                            IF Z6 != 0 GOTO [A-14]
```

---

This language works (turing complete, after all).


But it **sucks**.


We have to write programs like this:


```
[A-1]   IF X0 != 0 GOTO [B-1]       [A-11]  IF X1 != 0 GOTO [B-11]      [E-0]   INCREMENT Y
        INCREMENT Z2                        INCREMENT Z7                        DECREMENT Y
        IF Z2 != 0 GOTO [C-1]               IF Z7 != 0 GOTO [C-11]

[B-1]   DECREMENT X0                [B-11]  DECREMENT X1                [A-0]   IF Z0 != 0 GOTO [B-0]
        INCREMENT Y                         INCREMENT Z0                        INCREMENT Z11
        INCREMENT Z1                        INCREMENT Z6                        IF Z11 != 0 GOTO [END-0]
        INCREMENT Z3                        INCREMENT Z8
        IF Z3 != 0 GOTO [A-1]               IF Z8 != 0 GOTO [A-11]

[C-1]   IF Z1 != 0 GOTO [D-1]       [C-11]  IF Z6 != 0 GOTO [D-11]      [B-0]   DECREMENT Z0
        INCREMENT Z4                        INCREMENT Z9                        INCREMENT Y
        IF Z4 != 0 GOTO [END-1]             IF Z9 != 0 GOTO [E]                 INCREMENT Z12
                                                                                IF Z12 != 0 GOTO [A-0]

[D-1]   DECREMENT Z1                [D-11]  DECREMENT Z6                [END-0] STOP
        INCREMENT X0                        INCREMENT X1
        INCREMENT Z5                        INCREMENT Z10
        IF Z5 != 0 GOTO [C-1]               IF Z10 != 0 GOTO [C-11]

[END-1] INCREMENT Y                 [A-12]  DECREMENT Z0
        DECREMENT Y                         IF Z0 != 0 GOTO [A-12]

                                    [A-14]  DECREMENT Z6
                                            IF Z6 != 0 GOTO [A-14]
```


Solution?

---

# Macros

We can write **reusable, parameterized programs** that are inserted into other programs with a single line of code.

We define the program once, then **invoke it with arguments** wherever we need it.

For example, how can we set a register's value to zero?

---

# Macros

We can write **reusable, parameterized programs** that are inserted into other programs with a single line of code.

We define the program once, then **invoke it with arguments** wherever we need it.

For example, how can we set a register's value to zero?


```
MACRODEF !{REG} <- ZERO

[A] DECREMENT {REG}
    IF {REG} != 0 GOTO [A]
```

---

# Macros

We can write **reusable, parameterized programs** that are inserted into other programs with a single line of code.

We define the program once, then **invoke it with arguments** wherever we need it.

For example, how can we set a register's value to zero?


```
MACRODEF !{REG} <- ZERO

[A] DECREMENT {REG}
    IF {REG} != 0 GOTO [A]
```


Another useful macro: `goto`

---

# Macros

We can write **reusable, parameterized programs** that are inserted into other programs with a single line of code.

We define the program once, then **invoke it with arguments** wherever we need it.

For example, how can we set a register's value to zero?


```
MACRODEF !{REG} <- ZERO

[A] DECREMENT {REG}
    IF {REG} != 0 GOTO [A]
```


Another useful macro: `goto`


```
MACRODEF !goto {LABEL}

INCREMENT Z0
IF Z0 != 0 GOTO {LABEL}
```

---

Can we compose boolean **AND** using these macros?

```
MACRODEF !{TARGET} <- {A} and {B}

USEMACRO zero
USEMACRO goto
USEMACRO noop
```

---

Can we compose boolean **AND** using these macros?

```
MACRODEF !{TARGET} <- {A} and {B}

USEMACRO zero
USEMACRO goto
USEMACRO noop
```


```
            !{TARGET} <- ZERO
            IF {A} != 0 GOTO [CHECKB]
            !goto [END]

[CHECKB]    IF {B} != 0 GOTO [SET]
            !goto [END]

[SET]       INCREMENT {TARGET}

[END]       !noop
```

---

# We need to handle macro hygiene

Rename temporary `Z` registers

Rename labels to avoid collisions

---

# Subsequent Interpreter Implementation for Macros

<!-- In expanding macros, our `Vec<Instruction>` would need to realloc, and we would need to update our `jump_table: HashMap<Label, usize>` to fix our instruction indices. -->

When expanding macros, a `Vec<Instruction>` becomes awkward:
- insertions shift indices
- `jump_table: HashMap<Label, usize>` becomes fragile

---

# Subsequent Interpreter Implementation for Macros

<!-- In expanding macros, our `Vec<Instruction>` would need to realloc, and we would need to update our `jump_table: HashMap<Label, usize>` to fix our instruction indices. -->

When expanding macros, a `Vec<Instruction>` becomes awkward:
- insertions shift indices
- `jump_table: HashMap<Label, usize>` becomes fragile


## A New Data Structure: Jump List

Like a Linked List, but with some overhead

---

# Subsequent Interpreter Implementation for Macros

<!-- In expanding macros, our `Vec<Instruction>` would need to realloc, and we would need to update our `jump_table: HashMap<Label, usize>` to fix our instruction indices. -->

When expanding macros, a `Vec<Instruction>` becomes awkward:
- insertions shift indices
- `jump_table: HashMap<Label, usize>` becomes fragile


## A New Data Structure: Jump List

Like a Linked List, but with some overhead


It's basically a doubly-linked `Node` with one extra field:
- `element`     Either an `Instruction` or `MacroInvocation`
- `prev`        Previous node
- `next`        Next node
- NEW: `jump`   `Option`al reference to an arbitrary node somewhere in the chain

This lets us recursively expand `Node<MacroInvocation>`s in place without breaking jumps!

---

# Subsequent Interpreter Implementation for Macros

<!-- In expanding macros, our `Vec<Instruction>` would need to realloc, and we would need to update our `jump_table: HashMap<Label, usize>` to fix our instruction indices. -->

When expanding macros, a `Vec<Instruction>` becomes awkward:
- insertions shift indices
- `jump_table: HashMap<Label, usize>` becomes fragile


## A New Data Structure: Jump List

Like a Linked List, but with some overhead


It's basically a doubly-linked `Node` with one extra field:
- `element`     Either an `Instruction` or `MacroInvocation`
- `prev`        Previous node
- `next`        Next node
- NEW: `jump`   `Option`al reference to an arbitrary node somewhere in the chain

This lets us recursively expand `Node<MacroInvocation>`s in place without breaking jumps!


## Macro Hygiene w/ Regex

Capture groups are an advantage at the cost of ugly code.

Here, labels get a unique numeric suffix during expansion:
```rust
let uid = self.next_label_uid;

let label_regex = Regex::new(r"\[(?<lab>\S+)\]").unwrap();

*line = label_regex
    .replace_all(line, format!(r"[$lab-{}]", uid))
    .into_owned();
```

---

# Building Up

Stacking macros allows us to write more complex (and meaningful) programs.

---

# Building Up

Stacking macros allows us to write more complex (and meaningful) programs.


- Addition:     `plus.macro.l`
---

# Building Up

Stacking macros allows us to write more complex (and meaningful) programs.


- Addition:     `plus.macro.l`
- Subtraction:  `minus.macro.l`
---

# Building Up

Stacking macros allows us to write more complex (and meaningful) programs.


- Addition:     `plus.macro.l`
- Subtraction:  `minus.macro.l`
- Equality:     `eq.macro.l`
---

# Building Up

Stacking macros allows us to write more complex (and meaningful) programs.


- Addition:     `plus.macro.l`
- Subtraction:  `minus.macro.l`
- Equality:     `eq.macro.l`
- Divisibility: `divisible.macro.l`
---

# Building Up

Stacking macros allows us to write more complex (and meaningful) programs.


- Addition:     `plus.macro.l`
- Subtraction:  `minus.macro.l`
- Equality:     `eq.macro.l`
- Divisibility: `divisible.macro.l`
- Primality:    `isprime.macro.l`

---

# Demo Time

---

# Reflection - Code Design

---

# Reflection - Code Design


## I was fast and sloppy

My implementation includes:
- `struct`s with too much state
- No (or very poor) error messaging
- `JumpList` does not use generic types

---

# Reflection - Code Design


## I was fast and sloppy

My implementation includes:
- `struct`s with too much state
- No (or very poor) error messaging
- `JumpList` does not use generic types


## On idiomatic Rust

> Make invalid state unrepresentable

That takes careful design, and I did not get there.

`JumpList` implementation is especially in violation.

---

# Reflection - Code Design


## I was fast and sloppy

My implementation includes:
- `struct`s with too much state
- No (or very poor) error messaging
- `JumpList` does not use generic types


## On idiomatic Rust

> Make invalid state unrepresentable

That takes careful design, and I did not get there.

`JumpList` implementation is especially in violation.


- `unwrap()` and `clone()` everywhere
- No `Result` types
- I made too many fields `pub`
- I do not understand lifetimes; I avoided using them :(
- I do not leverage the compiler to catch application logic mistakes

---

# Reflection - Code Design


## I was fast and sloppy

My implementation includes:
- `struct`s with too much state
- No (or very poor) error messaging
- `JumpList` does not use generic types


## On idiomatic Rust

> Make invalid state unrepresentable

That takes careful design, and I did not get there.

`JumpList` implementation is especially in violation.


- `unwrap()` and `clone()` everywhere
- No `Result` types
- I made too many fields `pub`
- I do not understand lifetimes; I avoided using them :(
- I do not leverage the compiler to catch application logic mistakes


One good early decision was introducing an `Unsigned(usize)` type abstraction.
It keeps the door open to swapping in another non-negative integer type later.

---

# Where I Broke the No-AI Rule

---

# Where I Broke the No-AI Rule


## Heap allocated data structures in Rust are hard

I had trouble debugging `JumpList::replace_node_with_list()` (used during macro expansion). Twice, I had to turn to AI for help with implementation. `dbg!` only got me so far, and my implementation made the `JumpList` hard to inspect.

---

# Where I Broke the No-AI Rule


## Heap allocated data structures in Rust are hard

I had trouble debugging `JumpList::replace_node_with_list()` (used during macro expansion). Twice, I had to turn to AI for help with implementation. `dbg!` only got me so far, and my implementation made the `JumpList` hard to inspect.


## Borrow checker fight

An excerpt from a log I kept while building the interpreter:

> I found myself reliving the hell of clicking through the third page of search results, every link already purple, scouring Stack Overflow to see if anyone had the same issue as me. Each result was frustratingly close, always missing one or two crucial qualifiers.

---

# Where I Broke the No-AI Rule


## Heap allocated data structures in Rust are hard

I had trouble debugging `JumpList::replace_node_with_list()` (used during macro expansion). Twice, I had to turn to AI for help with implementation. `dbg!` only got me so far, and my implementation made the `JumpList` hard to inspect.


## Borrow checker fight

An excerpt from a log I kept while building the interpreter:

> I found myself reliving the hell of clicking through the third page of search results, every link already purple, scouring Stack Overflow to see if anyone had the same issue as me. Each result was frustratingly close, always missing one or two crucial qualifiers.


I tried to solve this borrow violation via search results:
```
error[E0502]: cannot borrow `*self` as mutable because it is also borrowed as immutable
   --> src/lib.rs:190:13
    |
189 |             let i = self.instructions.get(self.program_coun...
    |                     ----------------- immutable borrow occurs here
190 |             self.execute(i);
    |             ^^^^^-------^^^
    |             |    |
    |             |    immutable borrow later used by call
    |             mutable borrow occurs here
```

After 15 minutes, I gave up. Turning to ChatGPT, I got an immediate solution:

---

# Where I Broke the No-AI Rule


## Heap allocated data structures in Rust are hard

I had trouble debugging `JumpList::replace_node_with_list()` (used during macro expansion). Twice, I had to turn to AI for help with implementation. `dbg!` only got me so far, and my implementation made the `JumpList` hard to inspect.


## Borrow checker fight

An excerpt from a log I kept while building the interpreter:

> I found myself reliving the hell of clicking through the third page of search results, every link already purple, scouring Stack Overflow to see if anyone had the same issue as me. Each result was frustratingly close, always missing one or two crucial qualifiers.


I tried to solve this borrow violation via search results:
```
error[E0502]: cannot borrow `*self` as mutable because it is also borrowed as immutable
   --> src/lib.rs:190:13
    |
189 |             let i = self.instructions.get(self.program_coun...
    |                     ----------------- immutable borrow occurs here
190 |             self.execute(i);
    |             ^^^^^-------^^^
    |             |    |
    |             |    immutable borrow later used by call
    |             mutable borrow occurs here
```

After 15 minutes, I gave up. Turning to ChatGPT, I got an immediate solution:


> If `Instruction` is reasonably small (or can be made cheap to clone), Fix 1 is the idiomatic solution: clone the instruction into a local variable, then call execute.

---

# Where I Broke the No-AI Rule - cont.

---

# Where I Broke the No-AI Rule - cont.


## `bool` to `Option`

```rust
// Parse label
// TODO: do cleaner
let label = if first_word.starts_with("[") {
    Some(self.parse_label(first_word))
} else {
    None
};
```

---

# Where I Broke the No-AI Rule - cont.


## `bool` to `Option`

```rust
// Parse label
// TODO: do cleaner
let label = if first_word.starts_with("[") {
    Some(self.parse_label(first_word))
} else {
    None
};
```


Reading `Option` docs and searching results led to nothing. ChatGPT gave me the answer in one query:

```rust
let label = first_word
    .starts_with('[')
    .then(|| self.parse_label(first_word));
```

---

# Reflections

AI has a place in software development: **faster iteration** during debugging.

Where I find AI tools most useful:
- explaining unfamiliar code
- automating unintelligent, repetitive work
- turning technical design ideas into code

---

# Q&A

Repository - https://github.com/pharrison31415/l

Email - pharrison31415@&#8203;gmail.com