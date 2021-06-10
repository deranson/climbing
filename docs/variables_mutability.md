# Variables and Mutability

Code in [here](../src/modules/variables_mutability.rs)

A variable is **immutable by default** in Rust, eg `let x = 3; // its value can't be changed`

Use the `mut` keyword before its name to make it mutable, eg `let mut x = 3;`

## Constants

A **Constants** is immutable always, and the type of the value must be annotated eg `const PI: u32 = 3.14`

> Constants can be set only to a constant expression, not the result of a function call or any other value that could only be computed at runtime.

## Shadowing

A variable shadowed can be changed type of the value also, eg

```
let s: u32 = 1;
let s: u32 = 2; // shadow but change the value only
let s: &str = "s"; // shadow and change the type of the variable
...
```

## Summary:

- A variable is immutable by default

- The `mut` keyword to make it mutable

- Constants can be set only to a constant expression

- Shadowing to change the type and value of a previous variable

## Links

Variables and Mutability in [The Rust Programming Language](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)

Code in this repository in [here](../src/modules/variables_mutability.rs)

Table of Doc contents in [here](../README.md#contents--docs--modules)
