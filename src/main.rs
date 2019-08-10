extern crate serde;
extern crate serde_json;
extern crate rustyline;
extern crate reqwest;
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
