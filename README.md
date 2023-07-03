# Rust Debug Print Macros

This Rust library provides four simple macros for conditional printing in debug mode: `dbgprint!`, `dbgprintln!`, `dbgeprint!`, `dbgeprintln!`. These macros act as equivalents to `print!`, `println!`, `eprint!`, `eprintln!` respectively, but only produce output when the program is running in debug mode.

## Usage

Here's an example of how you might use these macros in your code:

```rust
// In your Rust file...

// Print to stdout
dbgprint!("Hello, {}!", "world"); // prints "Hello, world!" in debug mode
dbgprintln!("Hello, {}!", "world"); // prints "Hello, world!\n" in debug mode

// Print to stderr
dbgeprint!("Error: {}", "something went wrong"); // prints "Error: something went wrong" to stderr in debug mode
dbgeprintln!("Error: {}", "something went wrong"); // prints "Error: something went wrong\n" to stderr in debug mode
```

Note that these macros will not produce any output when your program is compiled in release mode.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
dbgprint = "0.1.0"
```

Then import the macros in your Rust files with:

```rust
use dbgprint::{dbgprint, dbgprintln, dbgeprint, dbgeprintln};
```

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/bwintertkb/dbgprint.

## License

`dbgprint` is distributed under the [MIT](https://choosealicense.com/licenses/mit/) and [Apache-2.0](https://choosealicense.com/licenses/apache-2.0/) licenses.

