
use rustyline;
use rustyline::line_buffer::LineBuffer;


pub struct CommandCompleter {
    commands:Vec<&'static str>,
}

impl CommandCompleter {
    pub fn new() -> CommandCompleter {

        let commands: Vec<&str> = vec!["help", "items", "projs", "quit" ];

        CommandCompleter {
           commands: commands
        }
    }
}

impl rustyline::completion::Completer for CommandCompleter {
    fn complete(&self, line: &str, pos: usize) -> rustyline::Result<(usize, Vec<String>)> {

        let mut completions : Vec<String> = Vec::new();
        for command in &self.commands {
            if command.starts_with(line) {
                completions.push(command.to_string());
            }
        }

        Ok((pos, completions))
    }

    fn update(&self, line: &mut LineBuffer, start: usize, elected: &str) {
        line.update(elected, start);
    }
}

#[cfg(test)]
use rustyline::completion::Completer;

#[test]
fn completion_test() {

    let completer = CommandCompleter::new();

    // Verify that the completion for i completes to items.
    assert_eq!(completer.complete("i", 0).unwrap(), (0, vec![String::from("items")]));
    assert_eq!(completer.complete("ite", 0).unwrap(), (0, vec![String::from("items")]));

    // Verify that the completion for q completes to quit.
    assert_eq!(completer.complete("q", 0).unwrap(), (0, vec![String::from("quit")]));
    assert_eq!(completer.complete("qui", 0).unwrap(), (0, vec![String::from("quit")]));

    // Verify that the completion for h completes to help.
    assert_eq!(completer.complete("h", 0).unwrap(), (0, vec![String::from("help")]));
    assert_eq!(completer.complete("he", 0).unwrap(), (0, vec![String::from("help")]));
}

