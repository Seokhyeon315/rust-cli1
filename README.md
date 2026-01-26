## Project Goal

A macOS computer cleaner CLI tool that:

- Reviews installed applications on macOS
- Checks how much space applications are taking up
- Tracks last time applications were used
- Scans for legacy files and caches from deleted applications
- Identifies orphaned files that are safe to delete
- Provides safe cleanup functionality with verification before deletion

The main purpose is to help efficiently find and clean up leftover files from manually deleted applications, which is typically a manual and inefficient process on macOS.

## Things I learned through this project

- To run the project: `cargo run cli1`
- `println` vs `println!`
  - In Rust, `println` is not a standard function. It will trigger a compile error if it's used.
  - `!` is a macro, not a function.
  - The primary purpose of this macro is to print text to the standard output (console) with optional formatting.
- Double Colons `(::)`:
  - Separates module/crate paths
  - `std::env::args()` means go into `std`, then `env`, then call `args()`
- Semicolons (`;`) mark the end of statement.
  - They are required for after statements (`let`, `println!`, function calls)
  - Optional/not needed: after expressions that return values (last line in function, match arms, if expressions)
  - So use semicolon when it returns a value
- `dbg!`: Prints and returns the value of a given expression for quick and dirty debugging.

## Feature 1: Reading file or directory

- [Official Docs reference of CLI](https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html)
- To accept command line argument, need `std::env::args` function provided in Rust’s standard library.
- `std::env::args` function returns an iterator of the command line arguments passed to the program. Iterator produce a series of values, and by calling `collect` method on an iterator to turn it into a collection (a.k.a vector) which contains all the elements the iterator produces.

```rust
// Reading the argument value
use std::env;

fn main() {
    let args: Vec<String>=env::args().collect();
    dbg!(args);
}

```

By running `cargo run`, I noticed that `[src/main.rs:7:5] args = [
    "target/debug/cli1",
]`. This means the program's name takes up the first value in the vector at `args[0]`.

```rust
// Saving the argument values in variables
use std::env;

fn main() {
    let args: Vec<String>=env::args().collect();
    // dbg!(args);
    let query = &args[1];
    let file_path = &args[2];
    println!("Searching for {query}");
    println!("In file {file_path}");
}
```
