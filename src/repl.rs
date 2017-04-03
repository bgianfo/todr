//! This module implements the todr REPL logic.

use std::env;
use std::path::Path;
use rustyline::error::ReadlineError;
use rustyline::Editor;

// Use our internal handlers module.
use handlers;

// On unix platforms you can use ANSI escape sequences
#[cfg(unix)]
static PROMPT: &'static str = "\x1b[1;32m>>\x1b[0m ";

// Windows consoles typically don't support ANSI escape sequences out
// of the box
#[cfg(windows)]
static PROMPT: &'static str = ">> ";
static DEFAULT_HISTORY_FILE: &'static str = ".todr_history";

#[derive(Debug)]
pub struct TodrRepl {
    /// The repl's readline editor.
    readline_editor: Editor<()>,

    // The configured history file.
    history_file: String,

    /// Flag that marks if we should exit or not.
    should_exit: bool,
}

impl TodrRepl {
    /// Factory method.
    pub fn new() -> TodrRepl {
        let home = env::home_dir().unwrap();
        let history_file = Path::new(&home)
            .join(DEFAULT_HISTORY_FILE)
            .to_string_lossy()
            .to_string();
        TodrRepl {
            // `()` can be used when no completer is required
            readline_editor: Editor::<()>::new(),
            history_file: history_file,
            should_exit: false,
        }
    }

    /// Executes the main process loop.
    pub fn process_command_loop(&mut self) {

        if let Err(_) = self.readline_editor.load_history(&self.history_file) {
            println!("No previous history to load...");
        }

        loop {

            // Handle graceful exit request.
            if self.should_exit {
                break;
            }

            let readline = self.readline_editor.readline(PROMPT);
            match readline {
                Ok(line) => {
                    self.process_line(line);
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }

        self.readline_editor
            .save_history(&self.history_file)
            .unwrap();
    }

    /// Processes a single line for a command.
    fn process_line(&mut self, line: String) {

        self.readline_editor.add_history_entry(&line);

        match line.as_ref() {

            // Handle printing help message.
            "help" | "h" => handlers::help_command(),

            // Handle executing the items commands.
            "items" | "i" => handlers::items_command(),

            // Handle executing the items commands.
            "projs" | "p" => handlers::projects_command(),

            // Handle graceful exit.
            "quit" | "q" => self.should_exit = true,

            // Handle unknown commands.
            _ => handlers::unknown_command(&line),
        }
    }
}
