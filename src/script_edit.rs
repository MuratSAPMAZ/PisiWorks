//! The text being written in Rules as Text: its lines, where the caret is, and what the
//! reading of it has found. A small editor of its own, since the program has no other
//! place where many lines are typed: enough to write, correct and paste rules in.

use crate::script::{Mistake, Read};

/// How far in a step is written.
const INDENT: &str = "    ";

#[derive(Clone, Debug, Default)]
pub struct ScriptEdit {
    pub lines: Vec<String>,
    /// The line the caret is on, and how many characters of it lie before the caret.
    pub row: usize,
    pub col: usize,
    /// What the text says, or what is wrong with it: read again after every change.
    pub outcome: Option<Result<Read, Vec<Mistake>>>,
    /// Whether the text has been changed since it was last taken into the document.
    pub changed: bool,
}

/// The place in a line, in bytes, of its `col`th character.
fn byte_at(line: &str, col: usize) -> usize {
    line.char_indices().nth(col).map_or(line.len(), |(at, _)| at)
}

impl ScriptEdit {
    pub fn new(text: &str) -> ScriptEdit {
        let mut lines: Vec<String> = text.lines().map(str::to_string).collect();
        if lines.is_empty() {
            lines.push(String::new());
        }
        let mut edit = ScriptEdit { lines, ..Default::default() };
        edit.check();
        edit.changed = false;
        edit
    }

    pub fn text(&self) -> String {
        let mut text = self.lines.join("\n");
        text.push('\n');
        text
    }

    /// Reads the text again.
    pub fn check(&mut self) {
        self.outcome = Some(crate::script::read(&self.text()));
        self.changed = true;
    }

    pub fn mistakes(&self) -> &[Mistake] {
        match &self.outcome {
            Some(Err(mistakes)) => mistakes,
            _ => &[],
        }
    }

    fn length(&self) -> usize {
        self.lines[self.row].chars().count()
    }

    /// Puts typed or pasted text in at the caret. A tab is a step's worth of spaces.
    pub fn insert(&mut self, text: &str) {
        for (index, piece) in text.replace('\r', "").replace('\t', INDENT).split('\n').enumerate() {
            if index > 0 {
                self.split();
            }
            let at = byte_at(&self.lines[self.row], self.col);
            self.lines[self.row].insert_str(at, piece);
            self.col += piece.chars().count();
        }
        self.check();
    }

    fn split(&mut self) {
        let at = byte_at(&self.lines[self.row], self.col);
        let rest = self.lines[self.row].split_off(at);
        self.lines.insert(self.row + 1, rest);
        self.row += 1;
        self.col = 0;
    }

    /// Enter: a new line. Under an event it starts further in, where its steps go; under a step, as far in as the step.
    pub fn newline(&mut self) {
        let line = &self.lines[self.row];
        let before: String = line.chars().take(self.col).collect();
        let lead: String = if before.trim_end().ends_with(':') { INDENT.to_string() } else { before.chars().take_while(|c| *c == ' ').collect() };
        self.split();
        self.lines[self.row].insert_str(0, &lead);
        self.col = lead.chars().count();
        self.check();
    }

    pub fn backspace(&mut self) {
        if self.col > 0 {
            // At the start of a step, the whole of its indent goes at once.
            let before: String = self.lines[self.row].chars().take(self.col).collect();
            let take = if before.chars().all(|c| c == ' ') && self.col % INDENT.len() == 0 { INDENT.len() } else { 1 };
            let (from, to) = (byte_at(&self.lines[self.row], self.col - take), byte_at(&self.lines[self.row], self.col));
            self.lines[self.row].replace_range(from..to, "");
            self.col -= take;
        } else if self.row > 0 {
            let line = self.lines.remove(self.row);
            self.row -= 1;
            self.col = self.length();
            self.lines[self.row].push_str(&line);
        }
        self.check();
    }

    pub fn delete(&mut self) {
        if self.col < self.length() {
            let (from, to) = (byte_at(&self.lines[self.row], self.col), byte_at(&self.lines[self.row], self.col + 1));
            self.lines[self.row].replace_range(from..to, "");
        } else if self.row + 1 < self.lines.len() {
            let line = self.lines.remove(self.row + 1);
            self.lines[self.row].push_str(&line);
        }
        self.check();
    }

    /// Moves the caret by lines and characters; over the end of a line it goes on to the next.
    pub fn step(&mut self, rows: isize, cols: isize) {
        if rows != 0 {
            self.row = (self.row as isize + rows).clamp(0, self.lines.len() as isize - 1) as usize;
            self.col = self.col.min(self.length());
        }
        if cols < 0 {
            if self.col > 0 {
                self.col -= 1;
            } else if self.row > 0 {
                self.row -= 1;
                self.col = self.length();
            }
        } else if cols > 0 {
            if self.col < self.length() {
                self.col += 1;
            } else if self.row + 1 < self.lines.len() {
                self.row += 1;
                self.col = 0;
            }
        }
    }

    pub fn home(&mut self) {
        self.col = 0;
    }

    pub fn end(&mut self) {
        self.col = self.length();
    }

    /// Puts the caret on a line, as near the character asked for as the line has.
    pub fn place(&mut self, row: usize, col: usize) {
        self.row = row.min(self.lines.len() - 1);
        self.col = col.min(self.length());
    }

    /// Adds a sentence as a line of its own under the caret's line: an event at the edge with a blank
    /// line before it, a step further in. The caret is left at its end.
    pub fn add_sentence(&mut self, sentence: &str, event: bool) {
        let line = if event { format!("{sentence}:") } else { format!("{INDENT}{sentence}") };
        // An empty line is written into; otherwise the sentence goes under the line the caret is on.
        if self.lines[self.row].trim().is_empty() {
            self.lines[self.row] = line;
        } else {
            if event {
                self.lines.insert(self.row + 1, String::new());
                self.row += 1;
            }
            self.lines.insert(self.row + 1, line);
            self.row += 1;
        }
        self.col = self.length();
        self.check();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::{set_language, Language};

    #[test]
    fn typing_a_rule_line_by_line_gives_a_rule() {
        set_language(Language::English);
        let mut edit = ScriptEdit::new("");
        edit.insert("When Apple is clicked:");
        // Enter after an event goes further in, where its step is written; Enter after a step stays as far in.
        edit.newline();
        assert_eq!((edit.row, edit.col, edit.lines[1].as_str()), (1, 4, "    "));
        edit.insert("hide Apple");
        edit.newline();
        edit.insert("stop evrything");
        assert_eq!(edit.mistakes().iter().map(|m| m.line).collect::<Vec<_>>(), vec![3]);
        // The caret goes back to the slip and it is put right.
        for _ in 0.."rything".len() {
            edit.step(0, -1);
        }
        edit.insert("e");
        assert!(edit.mistakes().is_empty());
        let found = edit.outcome.clone().unwrap().unwrap();
        assert_eq!(found.rules[0].sentence(), "When Apple is clicked: hide Apple; stop everything");
        // Backspace at the start of a step takes its indent at once, then joins it to the line above.
        edit.home();
        for _ in 0..4 {
            edit.step(0, 1);
        }
        edit.backspace();
        assert_eq!((edit.col, edit.lines[2].as_str()), (0, "stop everything"));
        edit.backspace();
        assert_eq!((edit.row, edit.lines.len(), edit.lines[1].as_str()), (1, 2, "    hide Applestop everything"));
    }

    #[test]
    fn pasting_and_adding_sentences_keeps_the_lines_right() {
        set_language(Language::English);
        let mut edit = ScriptEdit::new("");
        edit.insert("Numbers:\n\tScore = 0\n");
        assert_eq!(edit.lines, vec!["Numbers:", "    Score = 0", ""]);
        edit.add_sentence("When Apple is clicked", true);
        edit.add_sentence("increase Score by 1", false);
        assert_eq!(edit.text(), "Numbers:\n    Score = 0\nWhen Apple is clicked:\n    increase Score by 1\n");
        // A second event gets a blank line before it.
        edit.add_sentence("When the film starts", true);
        assert_eq!(&edit.lines[4..], ["", "When the film starts:"]);
        assert!(edit.mistakes().is_empty() && edit.changed);
        // Letters that take more than a byte are one character to the caret.
        let mut edit = ScriptEdit::new("Şişe");
        edit.end();
        edit.backspace();
        edit.step(0, -1);
        edit.insert("ğ");
        assert_eq!((edit.lines[0].as_str(), edit.col), ("Şiğş", 3));
        edit.delete();
        assert_eq!(edit.lines[0], "Şiğ");
    }
}
