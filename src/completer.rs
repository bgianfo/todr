use rustyline;
use rustyline::line_buffer::LineBuffer;

pub struct CustomCompletion {
    commands: Vec<&'static str>,
    hinter: rustyline::hint::HistoryHinter,
}

impl CustomCompletion {
    pub fn new() -> Self {
        let commands: Vec<&str> = vec!["help", "items", "projs", "quit"];

        Self {
            commands,
            hinter: rustyline::hint::HistoryHinter {},
        }
    }
}

impl rustyline::completion::Completer for CustomCompletion {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<String>)> {
        let mut completions: Vec<String> = Vec::new();
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

impl rustyline::hint::Hinter for CustomCompletion {
    fn hint(&self, line: &str, pos: usize, ctx: &rustyline::Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl rustyline::Helper for CustomCompletion {}

impl rustyline::highlight::Highlighter for CustomCompletion {}

/*
 * Figure out how to re-enable tests, as latest rustyline makes it impossible?
 * See: https://github.com/kkawakam/rustyline/issues/261

#[cfg(test)]
use rustyline::completion::Completer;

#[cfg(test)]
fn verify_completion(input: &str, expected_completion: &str)
{
    let hist = rustyline::history::History::new();

    let ctx = rustyline::Context {
    };

    let completer = CustomCompletion::new();
    assert_eq!(completer.complete(input, 0, &ctx).unwrap(), (0, vec![String::from(expected_completion)]));
}

#[test]
fn completion_test_items() {
    // Verify that the completion for i completes to items.  verify_completion("i", "items");
    verify_completion("ite", "items");
}

#[test]
fn completion_test_quit() {
    // Verify that the completion for q completes to quit.
    verify_completion("q", "quit");
    verify_completion("qui", "quit");
}

#[test]
fn completion_test_help() {
    // Verify that the completion for h completes to help.
    verify_completion("h", "help");
    verify_completion("he", "help");
}

#[test]
fn completion_test_projects() {
    // Verify that the completion for p completes to projs.
    verify_completion("p", "projs");
    verify_completion("pro", "projs");
}
*/
