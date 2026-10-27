/*# Rust Programming – Day 1: Basics

## 1. Hello World Program

```rust*/
fn main() {
    println!("Hello, World!");
}
/*```

## 2. Creating and Running a Rust Project

```bash
cargo new my_project   # Create a new Rust project
cd my_project          # Navigate to the project folder
cargo run              # Compile and run the program
```

Project Structure:
```text
my_project/
├── Cargo.toml      # Project settings and dependencies
└── src/
    └── main.rs     # Main Rust source code
```

## 3. Rust Syntax

- `fn` – Keyword used to define a function.
- `main()` – Entry point of every executable Rust program.
- `{ }` – Curly braces define a block of code.
- `println!()` – Macro used to print text to the console.
- `;` – Semicolon used to terminate statements where required.
- `//` – Single-line comment.
 – Multi-line comment.

## 4. What Is a Macro?

A macro is similar to a function, but it generates code during compilation.

- Macros are invoked using an exclamation mark `!`.
- `println!()` is a macro used to print text.
- Other examples include `vec!()`, `format!()`, and `assert!()`.

Example:
```rust
println!("Hello, Rust!");
```

## 5. Whitespace in Rust

Rust generally ignores extra whitespace, such as spaces, tabs, and newlines, between tokens.

Both examples work the same way:

```rust
fn main() {
    println!("Hello");
}
```

```rust
fn main(){println!("Hello");}
```*/

**Note:** Whitespace inside strings is preserved, and some whitespace is required to separate tokens.
