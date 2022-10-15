use std::env;
use std::io::stdout;
use std::io::Write;
use std::io::{self, BufRead};
use std::{thread, time};

#[allow(unused_must_use)]

// Read first argument as an integer timeout in microseconds
// If no arg is given, or an invalid arg is given, use default_timeout
fn get_timeout() -> u64 {
    // The value to be used if no arg, or an invalid arg is given
    let default_timeout = 1000;

    let args: Vec<String> = env::args().collect();

    let arg = match args.get(1) {
        Some(val) => val,
        None => {
            // No args given, use default value
            return default_timeout;
        }
    };

    let given_timeout = match arg.parse::<u64>() {
        Ok(val) => val,
        Err(e) => {
            eprintln!("{}\nUsing default timeout", e);
            // Invalid arg given, use default value
            return default_timeout;
        }
    };

    // The given arument was valid and will be used
    return given_timeout;
}

fn main() {
    let stdin = io::stdin();

    // Setup the timeout with the given timeout, or default val
    let timeout = time::Duration::from_micros(get_timeout());

    // Loop through all chars in all lines of piped input
    for line in stdin.lock().lines() {
        for c in line.unwrap().chars() {
            // Print char by char and flush the terminal
            print!("{}", c);
            let _ = stdout().flush();

            // Sleep for the given timeout
            thread::sleep(timeout);
        }

        // Re add the \n that was removed in the .lines() fn
        print!("\n")
    }

    // End with another \n to avoid EOF %
    print!("\n");
}
