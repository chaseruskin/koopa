pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const SHORT_HELP: &str = "\
Koopa is a copy/paste tool with superpowers.

Usage:
    kp [options] <src> <dest>

Arguments:
    <src>           filesystem path to copy
    <dest>          filesystem path to place copied contents 

Options:
    --shell, -s <key=value>...  specify runtime in-line text replacements
    --ignore-work               ignore .koopa folders along the working path
    --ignore-home               ignore the .koopa folder at the home path
    --force                     bypass safety checks and errors
    --verbose                   use verbose output
    --list                      list available files + shells and exit
    --version                   print version information and exit
    --help, -h                  print this help information and exit

Use 'kp --help --verbose' for more information about koopa.
";

pub const LONG_HELP: &str = "\
Koopa is a copy/paste tool with superpowers.
";

use colored::Colorize;

/// Displays the message `msg` to stdout as a warning.
pub fn warning(msg: String, verbose: bool) {
    if verbose == true {
        println!("{}: {}", "warning".yellow(), msg);
    }
}

/// Displays the message `msg` to stdout as information.
pub fn info(msg: String, verbose: bool) {
    if verbose == true {
        println!("info: {}", msg);
    }
}
