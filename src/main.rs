#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate rustyline;
extern crate ease;
extern crate term;

// Use the module in the sub directory.
mod repl;
mod handlers;
mod types;
mod render;
mod completer;

fn main() {
    let mut repl = repl::TodrRepl::new();
    repl.process_command_loop();
}
