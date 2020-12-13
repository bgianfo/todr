// Subscribe to most of the clippy lints.
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
// Disable warnings for things we don't care about.
#![allow(unknown_lints)]
#![allow(clippy::implicit_return)]
#![allow(clippy::integer_arithmetic)]
#![allow(clippy::print_stdout)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::match_same_arms)]
// TODO: We should really fix these issues.
#![allow(clippy::use_debug)]
#![allow(clippy::missing_docs_in_private_items)]
// Disable unsafe code.
#![forbid(unsafe_code)]

extern crate dirs;
extern crate reqwest;
extern crate rustyline;
extern crate serde;
extern crate serde_json;
extern crate term;

// Use the module in the sub directory.
mod completer;
mod config;
mod handlers;
mod renderer;
mod repl;
mod types;

fn main() {
    let mut repl = repl::Todr::new();
    repl.process_command_loop();
}
