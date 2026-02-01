## Project Goal

> This is my first Rust project. Instead of reading official documentation from ch1, I like to learn a programming language by building a project. I refer documentation when I face error or need a new feature for the project.

A macOS computer cleaner CLI:

- [ ] Reviews installed applications on macOS
- [ ] Checks how much space applications are taking up
- [ ] Tracks last time applications were used
- [ ] Scans for legacy files and caches from deleted applications
- [ ] Identifies orphaned files that are safe to delete
- [ ] Provides safe cleanup functionality with verification before deletion

The main purpose is to help efficiently find and clean up leftover files from manually deleted applications, which is typically a manual and inefficient process on macOS.

## Step 1: Reading file or directory

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
- In Rust's function, if the last thing in a function is an expression (no semicolon), that expression’s value is the return value of the function.

## Step 2: Listing macOS applications

```rust
use std::path::Path;

fn main() {
    let application_dir=Path::new("/Applications");
    println!("This is the path name of target: {application_dir}");
}
```

Original intention of this code was to print the directory name of system level of Applications. But when I run `cargo run`, error shows `` Path` doesn't implement `std::fmt::Display ``.

- So what I was doing was to use a type which doesn't implement some trait in a place which expected that trait.
- A trait is like an interface that data types can implement. Traits can be made up of three varieties of associated items: 1)functions and methods 2)types and 3)constants.
- A path is not guaranteed to be valid UTF-8. On Unix (including macOS), a file name can be any sequence of bytes.

```rust
// Correct way to show the path name is
use std::Path;

fn main(){
  let application_dir=Path::new("/Applications");
  println!("This is the path name of target: {}", application_dir.display());
}
// This will return This is the path name of target: /Applications when I run `cargo run`
```

To list all the applications inside `/Applications`,

- https://doc.rust-lang.org/std/path/struct.Path.html#method.read_dir

```rust
use std::fs;
use std::path::Path;

fn main() {
    // Set Absolute path for macOS Applications
    // In macOS, there are two different path for applications: system level: `/Applications`  vs User level: `~Applications`
    let application_dir=Path::new("/Applications");
    println!("Target directory: {}", application_dir.display());

    // List the each application name under /Applications
    let entries = fs::read_dir(application_dir).expect("Failed to read /Applications");
    for entry in entries {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        if path.is_dir() && path.extension().map_or(false, |e| e == "app") {
            println!("  {}", path.display());
        }
    }
}
```

- A function `read_dir()` returns an iterator over the entries within a directory.
- `expect()` method allows me to make the `panic!` error message. By using this, it will either give `ReadDir` or an error.
- `entry.path()`: A method on `fs::DirEntry`. It returns the full path of that directory entry (i.e. /Applications/Safari.app). This is required to check if it's a directory, what its extension is and to print it.
- `path.is_dir()`: A method on `Path/PathBuf`. It returns `true` if that path is a directory, `false` otherwise (or if it doesn't exist).
- `path.extension()`: A method on Path. Returns `Some(ext)` if the path has an extension (e.g. "app"), `None` if not (e.g. no dot or trailing dot).
- `map_or(false, |e| e == "app")`: if `None`, returns false. If `Some(e)`, returns the result of `e=="app"`. `e` indicates extension.
  - `|e| e` is Rust closure (anonymous function) syntax. `|e|` indicates parameter named e. The part between `|` and `|` is the parameter list. The part after `|e|` is a body/return value.
