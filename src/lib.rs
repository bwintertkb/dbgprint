/// Prints to the standard output, with a newline, in debug mode.
///
/// Equivalent to the `println!` macro, but does nothing when not in debug mode.
///
/// # Example
///
/// ```
/// use dbgprint::dbgprintln;
/// dbgprintln!("Hello, {}!", "world"); // prints "Hello, world!" in debug mode
/// ```
#[macro_export]
macro_rules! dbgprintln {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            println!($($arg)*);
        }
    };
}

/// Prints to the standard output in debug mode.
///
/// Equivalent to the `print!` macro, but does nothing when not in debug mode.
///
/// # Example
///
/// ```
/// use dbgprint::dbgprint;
/// dbgprint!("Hello, {}!", "world"); // prints "Hello, world!" in debug mode
/// ```
#[macro_export]
macro_rules! dbgprint {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            print!($($arg)*);
        }
    };
}

/// Prints to the standard error, with a newline, in debug mode.
///
/// Equivalent to the `eprintln!` macro, but does nothing when not in debug mode.
///
/// # Example
///
/// ```
/// use dbgprint::dbgeprintln;
/// dbgeprintln!("Error: {}", "something went wrong"); // prints "Error: something went wrong" to stderr in debug mode
/// ```
#[macro_export]
macro_rules! dbgeprintln {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            eprintln!($($arg)*);
        }
    };
}

/// Prints to the standard error in debug mode.
///
/// Equivalent to the `eprint!` macro, but does nothing when not in debug mode.
///
/// # Example
///
/// ```
/// use dbgprint::dbgeprint;
/// dbgeprint!("Error: {}", "something went wrong"); // prints "Error: something went wrong" to stderr in debug mode
/// ```
#[macro_export]
macro_rules! dbgeprint {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            eprint!($($arg)*);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dbgprint() {
        // There is no easy way to test the output of these macros
        // without adding additional dependencies, so we just call
        // them to make sure they compile correctly.
        dbgprint!("Test argument: {}", 1);
        dbgprintln!("Test argument: {}", 1);
        dbgeprint!("Some other test: {}", "text");
        dbgeprintln!("Some other test: {}", "text");
    }
}
