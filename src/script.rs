//! The written language of PisiSahne: rules as text.
//!
//! It is not a second thing beside the lists of Rules and Numbers: it is the sentences
//! those lists write, typed by hand. A rule is its event on a line ending in a colon and
//! its steps under it, further in:
//!
//! ```text
//! Numbers:
//!     Score = 0
//!
//! When Apple touches Basket:
//!     increase Score by 1
//!     put Apple at the top, at random
//! ```
//!
//! The words are those of the interface's language, so the same game reads in Turkish
//! as `Elma nesnesi Sepet nesnesine değince:`. What the text can say and the lists
//! cannot: a number may be a sum (`Score * 2`, `Hero.x + 10`, `random(1, 6)`), a rule
//! may have several conditions joined by "and", and steps to take "otherwise".
//!
//! Reading is the writing turned round: every sentence [`crate::rules`] can write is
//! tried against a line, and the first that fits gives the rule. So what the program
//! writes it can always read back.

use crate::i18n::{t, tf};
use crate::rules::{choice_name, Condition, Event, EventKind, Pick, Rule, Step, StepKind, Variable, EDGES, KEYS};

// ---- Sums ---------------------------------------------------------------------------------

/// A piece of a sum.
#[derive(Clone, Debug, PartialEq)]
enum Token {
    Number(f32),
    /// A name: of a number, or of an object when a place of it is asked for.
    Name(String),
    Sign(char),
}

fn tokens(text: &str) -> Result<Vec<Token>, String> {
    let chars: Vec<char> = text.chars().collect();
    let mut list = Vec::new();
    let mut at = 0;
    while at < chars.len() {
        let c = chars[at];
        if c.is_whitespace() {
            at += 1;
        } else if c.is_ascii_digit() || (c == '.' && chars.get(at + 1).is_some_and(|next| next.is_ascii_digit())) {
            let start = at;
            while at < chars.len() && (chars[at].is_ascii_digit() || chars[at] == '.') {
                at += 1;
            }
            let written: String = chars[start..at].iter().collect();
            list.push(Token::Number(written.parse::<f32>().map_err(|_| tf("{} is not a number", &[&written]))?));
        } else if c == '"' {
            // A name with spaces in it is written between quotation marks.
            let end = chars[at + 1..].iter().position(|c| *c == '"').ok_or_else(|| t("a quotation mark is not closed").to_string())?;
            list.push(Token::Name(chars[at + 1..at + 1 + end].iter().collect()));
            at += end + 2;
        } else if c.is_alphabetic() || c == '_' {
            let start = at;
            while at < chars.len() && (chars[at].is_alphanumeric() || chars[at] == '_') {
                at += 1;
            }
            list.push(Token::Name(chars[start..at].iter().collect()));
        } else if "+-*/().,".contains(c) {
            list.push(Token::Sign(c));
            at += 1;
        } else {
            return Err(tf("{} cannot be in a sum", &[&c]));
        }
    }
    Ok(list)
}

/// The words for a number picked at random, in either language.
const RANDOM: [&str; 2] = ["random", "rastgele"];

/// What a sum mentions: the numbers by name, and the objects whose place it asks for.
#[derive(Default)]
struct Mentions {
    numbers: Vec<String>,
    objects: Vec<String>,
}

/// Reads a sum through, to see that it is one. `sum := term (('+' | '-') term)*`, `term := part (('*' | '/') part)*`,
/// `part := number | name | name '.' ('x' | 'y') | random '(' sum ',' sum ')' | '(' sum ')' | '-' part`.
struct Reader<'a> {
    tokens: &'a [Token],
    at: usize,
    found: Mentions,
}

impl Reader<'_> {
    fn sign(&mut self, wanted: char) -> bool {
        let is = self.tokens.get(self.at) == Some(&Token::Sign(wanted));
        if is {
            self.at += 1;
        }
        is
    }

    fn sum(&mut self) -> Result<(), String> {
        self.term()?;
        while self.sign('+') || self.sign('-') {
            self.term()?;
        }
        Ok(())
    }

    fn term(&mut self) -> Result<(), String> {
        self.part()?;
        while self.sign('*') || self.sign('/') {
            self.part()?;
        }
        Ok(())
    }

    fn part(&mut self) -> Result<(), String> {
        match self.tokens.get(self.at).cloned() {
            Some(Token::Number(_)) => self.at += 1,
            Some(Token::Sign('-')) => {
                self.at += 1;
                self.part()?;
            }
            Some(Token::Sign('(')) => {
                self.at += 1;
                self.sum()?;
                if !self.sign(')') {
                    return Err(t("a bracket is not closed").to_string());
                }
            }
            Some(Token::Name(name)) => {
                self.at += 1;
                if RANDOM.contains(&name.as_str()) && self.sign('(') {
                    self.sum()?;
                    if !self.sign(',') {
                        return Err(t("a number at random is written random(from, to)").to_string());
                    }
                    self.sum()?;
                    if !self.sign(')') {
                        return Err(t("a bracket is not closed").to_string());
                    }
                } else if self.sign('.') {
                    match self.tokens.get(self.at) {
                        Some(Token::Name(side)) if side == "x" || side == "y" => self.at += 1,
                        _ => return Err(tf("after {}. comes x or y", &[&name])),
                    }
                    self.found.objects.push(name);
                } else {
                    self.found.numbers.push(name);
                }
            }
            _ => return Err(t("a number or a name is missing").to_string()),
        }
        Ok(())
    }
}

fn mentions(text: &str) -> Result<Mentions, String> {
    let tokens = tokens(text)?;
    let mut reader = Reader { tokens: &tokens, at: 0, found: Mentions::default() };
    reader.sum()?;
    if reader.at < tokens.len() {
        return Err(t("the sum does not end where it should").to_string());
    }
    Ok(reader.found)
}

/// The objects a sum asks the place of. A sum that cannot be read asks for none.
pub fn objects_in(text: &str) -> Vec<&str> {
    if text.is_empty() {
        return Vec::new();
    }
    let found = mentions(text).map(|found| found.objects).unwrap_or_default();
    // The names are given back as they stand in the text, so that they live as long as it does.
    found.iter().filter_map(|name| text.find(name.as_str()).map(|at| &text[at..at + name.len()])).collect()
}

/// A number as it is written in a rule: plain, or a sum to work out. The sum is kept as text.
#[derive(Clone, Debug, PartialEq)]
pub enum Amount {
    Plain(f32),
    Sum(String),
}

/// Reads what stands where a number belongs. A comma is taken for a decimal point in a plain number.
pub fn amount(text: &str, numbers: &[String]) -> Result<Amount, String> {
    let text = text.trim();
    if let Ok(plain) = text.replace(',', ".").parse::<f32>() {
        if plain.is_finite() {
            return Ok(Amount::Plain(plain));
        }
    }
    let found = mentions(text)?;
    if let Some(unknown) = found.numbers.iter().find(|name| !numbers.contains(name)) {
        return Err(tf("there is no number called {}", &[unknown]));
    }
    Ok(Amount::Sum(text.to_string()))
}

// ---- Fitting a sentence to a line ---------------------------------------------------------

/// Where `mark` next stands in `line` from `from` on, outside brackets and quotation marks.
fn find_outside(line: &str, from: usize, mark: &str) -> Option<usize> {
    let (mut depth, mut quoted) = (0i32, false);
    for (at, c) in line[from..].char_indices() {
        let at = from + at;
        if !quoted && depth == 0 && line[at..].starts_with(mark) {
            return Some(at);
        }
        match c {
            '"' => quoted = !quoted,
            '(' if !quoted => depth += 1,
            ')' if !quoted => depth -= 1,
            _ => {}
        }
    }
    None
}

/// What stands in the gaps of a sentence, if the line is that sentence: `pattern` has `{}` for each gap.
/// A gap takes as little as it can, and the last one what is left.
fn fit(pattern: &str, line: &str) -> Option<Vec<String>> {
    let parts: Vec<&str> = pattern.split("{}").collect();
    let mut rest = line.strip_prefix(parts[0])?;
    let mut gaps = Vec::new();
    for (index, part) in parts.iter().enumerate().skip(1) {
        let last = index == parts.len() - 1;
        let end = if last {
            // The last words must end the line.
            if !rest.ends_with(part) {
                return None;
            }
            rest.len() - part.len()
        } else if part.is_empty() {
            return None;
        } else {
            find_outside(rest, 0, part)?
        };
        let gap = rest[..end].trim();
        if gap.is_empty() {
            return None;
        }
        gaps.push(gap.to_string());
        rest = &rest[end + part.len()..];
    }
    (parts.len() > 1 || rest.is_empty()).then_some(gaps)
}

/// A line with its first letter in the other case: "Hide Apple" for "hide Apple".
fn recased(line: &str) -> String {
    let mut chars = line.chars();
    match chars.next() {
        Some(first) if first.is_lowercase() => first.to_uppercase().chain(chars).collect(),
        Some(first) => first.to_lowercase().chain(chars).collect(),
        None => String::new(),
    }
}

/// What a line was found to lack, to be told to its writer.
type Fault = String;

/// What the reading knows: the numbers there are, and what it has found doubtful.
struct Known<'a> {
    numbers: &'a [String],
}

impl Known<'_> {
    fn number(&self, name: &str) -> Result<String, Fault> {
        if self.numbers.iter().any(|known| known == name) {
            Ok(name.to_string())
        } else {
            Err(tf("there is no number called {}", &[&name]))
        }
    }
}

/// The value a choice is kept as, from its name in the interface's language.
fn choice(pick: Pick, list: &[(&str, &str)], shown: &str) -> Option<String> {
    list.iter().find(|(key, _)| choice_name(pick, key).to_lowercase() == shown.to_lowercase()).map(|(key, _)| key.to_string())
}

/// The sentences an event can be, each with the kind it gives and, for a number held against a value, which way.
const EVENT_FORMS: [(&str, EventKind, &str); 11] = [
    ("When the film starts", EventKind::Start, ""),
    ("When a scene starts", EventKind::Scene, ""),
    ("When the {} key is pressed", EventKind::Key, ""),
    ("When {} is clicked", EventKind::Click, ""),
    ("When {} touches {} of the stage", EventKind::Edge, ""),
    ("When {} touches {}", EventKind::Touch, ""),
    ("Every {} seconds", EventKind::Every, ""),
    ("After {} seconds", EventKind::After, ""),
    ("When {} goes above {}", EventKind::Number, ">"),
    ("When {} goes below {}", EventKind::Number, "<"),
    ("When {} becomes {}", EventKind::Number, "="),
];

fn event(line: &str, known: &Known) -> Result<Event, Fault> {
    let mut fault = None;
    for (pattern, kind, how) in EVENT_FORMS {
        let Some(gaps) = fit(t(pattern), line).or_else(|| fit(t(pattern), &recased(line))) else { continue };
        let plain = |text: &str| text.replace(',', ".").parse::<f32>().ok().filter(|n| n.is_finite());
        let made = match kind {
            EventKind::Start | EventKind::Scene => Some(Event::new(kind)),
            EventKind::Key => choice(Pick::Key, &KEYS, &gaps[0]).map(|key| Event { kind, a: key, ..Default::default() }),
            EventKind::Click => Some(Event { kind, a: gaps[0].clone(), ..Default::default() }),
            EventKind::Touch => Some(Event { kind, a: gaps[0].clone(), b: gaps[1].clone(), ..Default::default() }),
            EventKind::Edge => choice(Pick::Edge, &EDGES, &gaps[1]).map(|edge| Event { kind, a: gaps[0].clone(), b: edge, ..Default::default() }),
            EventKind::Every | EventKind::After => plain(&gaps[0]).map(|n| Event { kind, n, ..Default::default() }),
            EventKind::Number => match (known.number(&gaps[0]), plain(&gaps[1])) {
                (Ok(name), Some(n)) => Some(Event { kind, a: name, b: how.to_string(), n }),
                (Err(why), Some(_)) => {
                    fault = Some(why);
                    None
                }
                _ => None,
            },
        };
        if let Some(made) = made {
            return Ok(made);
        }
    }
    Err(fault.unwrap_or_else(|| t("this is not an event I know").to_string()))
}

const CONDITION_FORMS: [(&str, &str); 3] = [("{} is more than {}", ">"), ("{} is less than {}", "<"), ("{} is {}", "=")];

fn condition(text: &str, known: &Known) -> Result<Condition, Fault> {
    let mut fault = None;
    for (pattern, how) in CONDITION_FORMS {
        let Some(gaps) = fit(t(pattern), text) else { continue };
        match (known.number(&gaps[0]), amount(&gaps[1], known.numbers)) {
            (Ok(number), Ok(Amount::Plain(value))) => return Ok(Condition { number, how: how.to_string(), value, ..Default::default() }),
            (Ok(number), Ok(Amount::Sum(expr))) => return Ok(Condition { number, how: how.to_string(), expr, ..Default::default() }),
            (Err(why), _) | (_, Err(why)) => fault = fault.or(Some(why)),
        }
    }
    Err(fault.unwrap_or_else(|| t("this is not a condition I know").to_string()))
}

/// The first line of a rule: its event, and after ", if" its conditions joined by "and".
fn head(line: &str, known: &Known) -> Result<(Event, Vec<Condition>), Fault> {
    // The line is split at each place the words between event and conditions stand, until both halves read.
    // Only when no split reads is the whole line taken for an event, since an event's last name would
    // otherwise swallow the conditions.
    let pattern = t("{}, if {}");
    let between = pattern.split("{}").nth(1).unwrap_or(", ");
    let mut from = 0;
    let mut fault: Option<Fault> = None;
    while let Some(at) = find_outside(line, from, between) {
        from = at + between.len();
        let Ok(found) = event(&line[..at], known) else { continue };
        let mut conditions = Vec::new();
        let mut bad = None;
        let (mut rest, joiner) = (&line[at + between.len()..], t(" and "));
        loop {
            let end = find_outside(rest, 0, joiner);
            match condition(rest[..end.unwrap_or(rest.len())].trim(), known) {
                Ok(only) => conditions.push(only),
                Err(why) => bad = bad.or(Some(why)),
            }
            match end {
                Some(end) => rest = &rest[end + joiner.len()..],
                None => break,
            }
        }
        match bad {
            None => return Ok((found, conditions)),
            Some(why) => fault = Some(why),
        }
    }
    match (event(line, known), fault) {
        (Ok(found), None) => Ok((found, Vec::new())),
        // A line that reads as an event only by taking its conditions for a name has conditions that do not read.
        (Ok(_), Some(why)) | (Err(_), Some(why)) => Err(why),
        (Err(why), None) => Err(why),
    }
}

/// What stands in a gap of a step's sentence.
#[derive(Clone, Copy, PartialEq)]
enum Gap {
    A,
    B,
    X,
    Y,
}

/// The sentences a step can be: the kind each gives, what its gaps hold in the order they stand, and
/// whether the first number is taken away ("decrease", "slower").
const STEP_FORMS: [(&str, StepKind, &[Gap], bool); 26] = [
    ("move {} by {} right and {} down", StepKind::Move, &[Gap::A, Gap::X, Gap::Y], false),
    ("put {} somewhere at random", StepKind::Random, &[Gap::A], false),
    ("put {} at the top, at random", StepKind::RandomTop, &[Gap::A], false),
    ("put {} at {}, {}", StepKind::GoTo, &[Gap::A, Gap::X, Gap::Y], false),
    ("show {}", StepKind::Show, &[Gap::A], false),
    ("hide {}", StepKind::Hide, &[Gap::A], false),
    ("make {} jump at {}", StepKind::Jump, &[Gap::A, Gap::X], false),
    ("decrease {} by {}", StepKind::Add, &[Gap::A, Gap::X], true),
    ("increase {} by {}", StepKind::Add, &[Gap::A, Gap::X], false),
    ("make a copy of {} at {}", StepKind::Spawn, &[Gap::A, Gap::B], false),
    ("remove {}", StepKind::Remove, &[Gap::A], false),
    ("make {} blink for a moment", StepKind::Flash, &[Gap::A], false),
    ("make {} slower by {} percent", StepKind::Faster, &[Gap::A, Gap::X], true),
    ("make {} faster by {} percent", StepKind::Faster, &[Gap::A, Gap::X], false),
    ("set {} at random from {} to {}", StepKind::RandomNumber, &[Gap::A, Gap::X, Gap::Y], false),
    ("set {} to {}", StepKind::Set, &[Gap::A, Gap::X], false),
    ("wait {} seconds", StepKind::Wait, &[Gap::X], false),
    ("stop the film", StepKind::StopFilm, &[], false),
    ("play the film", StepKind::PlayFilm, &[], false),
    ("go to frame {} and play", StepKind::GotoFrame, &[Gap::X], false),
    ("go to frame {} and stop", StepKind::GotoFrameStop, &[Gap::X], false),
    ("go to the next scene", StepKind::NextScene, &[], false),
    ("go to scene {}", StepKind::GotoScene, &[Gap::X], false),
    ("play the sound {}", StepKind::Sound, &[Gap::A], false),
    ("stop everything", StepKind::StopAll, &[], false),
    ("start again", StepKind::Restart, &[], false),
];

fn step(line: &str, known: &Known) -> Result<Step, Fault> {
    let mut fault = None;
    'forms: for (pattern, kind, gaps, taken_away) in STEP_FORMS {
        let Some(found) = fit(t(pattern), line).or_else(|| fit(t(pattern), &recased(line))) else { continue };
        let mut made = Step { kind, ..Default::default() };
        for (gap, text) in gaps.iter().zip(&found) {
            match gap {
                Gap::A if kind.needs().0 == Some(Pick::Variable) => match known.number(text) {
                    Ok(name) => made.a = name,
                    Err(why) => {
                        fault = fault.or(Some(why));
                        continue 'forms;
                    }
                },
                Gap::A => made.a = text.clone(),
                Gap::B => made.b = text.clone(),
                Gap::X | Gap::Y => {
                    let read = match amount(text, known.numbers) {
                        Ok(read) => read,
                        Err(why) => {
                            fault = fault.or(Some(why));
                            continue 'forms;
                        }
                    };
                    let turn = taken_away && *gap == Gap::X;
                    match (read, *gap) {
                        (Amount::Plain(n), Gap::X) => made.x = if turn { -n } else { n },
                        (Amount::Plain(n), _) => made.y = n,
                        (Amount::Sum(sum), Gap::X) => made.ex = if turn { format!("-({sum})") } else { sum },
                        (Amount::Sum(sum), _) => made.ey = sum,
                    }
                }
            }
        }
        return Ok(made);
    }
    Err(fault.unwrap_or_else(|| t("this is not a step I know").to_string()))
}

/// Every sentence an event can be, in the interface's language, with a question mark where a name or a number goes.
pub fn event_sentences() -> Vec<String> {
    EVENT_FORMS.iter().map(|(pattern, _, _)| t(pattern).replace("{}", "?")).collect()
}

/// Every sentence a step can be, likewise.
pub fn step_sentences() -> Vec<String> {
    STEP_FORMS.iter().map(|(pattern, _, _, _)| t(pattern).replace("{}", "?")).collect()
}

// ---- The whole text ----------------------------------------------------------------------

/// What is wrong on a line of the text, counted from 1.
#[derive(Clone, Debug, PartialEq)]
pub struct Mistake {
    pub line: usize,
    pub what: String,
}

/// The rules and numbers a text says.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Read {
    pub variables: Vec<Variable>,
    pub rules: Vec<Rule>,
}

/// How far in a step is written.
const INDENT: &str = "    ";

/// The rules and the numbers as text, in the language of the interface.
pub fn write(variables: &[Variable], rules: &[Rule]) -> String {
    let mut text = String::new();
    if !variables.is_empty() {
        text.push_str(&format!("{}:\n", t("Numbers")));
        for variable in variables {
            text.push_str(&format!("{INDENT}{} = {}\n", variable.name, variable.start));
        }
        text.push('\n');
    }
    for rule in rules {
        text.push_str(&format!("{}:\n", rule.head()));
        for step in &rule.steps {
            text.push_str(&format!("{INDENT}{}\n", step.sentence()));
        }
        if !rule.otherwise.is_empty() {
            text.push_str(&format!("{}:\n", t("otherwise")));
            for step in &rule.otherwise {
                text.push_str(&format!("{INDENT}{}\n", step.sentence()));
            }
        }
        text.push('\n');
    }
    text
}

/// Where the steps being read go.
#[derive(Clone, Copy, PartialEq)]
enum Under {
    Nothing,
    Numbers,
    Rule,
    Otherwise,
}

/// Reads a text. Every line that cannot be read is told of, and nothing is taken unless all of it reads.
pub fn read(text: &str) -> Result<Read, Vec<Mistake>> {
    let mut found = Read::default();
    let mut mistakes = Vec::new();
    let lines: Vec<&str> = text.lines().collect();
    // The numbers are gathered first, so that a rule may speak of a number written further down.
    let mut under = Under::Nothing;
    for (index, raw) in lines.iter().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with("//") {
            continue;
        }
        let inside = raw.starts_with(' ') || raw.starts_with('\t');
        if !inside {
            under = if line.trim_end_matches(':').trim().to_lowercase() == t("Numbers").to_lowercase() { Under::Numbers } else { Under::Nothing };
        } else if under == Under::Numbers {
            match line.split_once('=') {
                Some((name, start)) if !name.trim().is_empty() && !name.trim().contains(char::is_whitespace) => match start.trim().replace(',', ".").parse::<f32>() {
                    Ok(start) if !found.variables.iter().any(|v| v.name == name.trim()) => found.variables.push(Variable { name: name.trim().to_string(), start }),
                    Ok(_) => mistakes.push(Mistake { line: index + 1, what: tf("the number {} is written twice", &[&name.trim()]) }),
                    Err(_) => mistakes.push(Mistake { line: index + 1, what: tf("{} is not a number", &[&start.trim()]) }),
                },
                _ => mistakes.push(Mistake { line: index + 1, what: t("a number is written as: Name = 0, the name one word").to_string() }),
            }
        }
    }
    let names: Vec<String> = found.variables.iter().map(|v| v.name.clone()).collect();
    let known = Known { numbers: &names };
    under = Under::Nothing;
    for (index, raw) in lines.iter().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with("//") {
            continue;
        }
        let inside = raw.starts_with(' ') || raw.starts_with('\t');
        if !inside {
            let Some(title) = line.strip_suffix(':').map(str::trim) else {
                mistakes.push(Mistake { line: index + 1, what: t("a line that starts at the edge is an event and ends with a colon; steps are written further in").to_string() });
                under = Under::Nothing;
                continue;
            };
            if title.to_lowercase() == t("Numbers").to_lowercase() {
                under = Under::Numbers;
            } else if title.to_lowercase() == t("otherwise") {
                if found.rules.is_empty() || !matches!(under, Under::Rule) {
                    mistakes.push(Mistake { line: index + 1, what: t("\"otherwise\" comes after the steps of a rule").to_string() });
                    under = Under::Nothing;
                } else {
                    under = Under::Otherwise;
                }
            } else {
                match head(title, &known) {
                    Ok((event, mut conditions)) => {
                        let when = if conditions.is_empty() { None } else { Some(conditions.remove(0)) };
                        found.rules.push(Rule { event, when, also: conditions, ..Default::default() });
                        under = Under::Rule;
                    }
                    Err(what) => {
                        mistakes.push(Mistake { line: index + 1, what });
                        under = Under::Nothing;
                    }
                }
            }
        } else {
            match under {
                Under::Numbers => {}
                Under::Nothing => {
                    // A step under an event that could not be read is passed over: the event has been told of.
                    if found.rules.is_empty() && mistakes.is_empty() {
                        mistakes.push(Mistake { line: index + 1, what: t("a step comes under an event").to_string() });
                    }
                }
                Under::Rule | Under::Otherwise => match (step(line, &known), found.rules.last_mut()) {
                    (Ok(step), Some(rule)) if under == Under::Rule => rule.steps.push(step),
                    (Ok(step), Some(rule)) => rule.otherwise.push(step),
                    (Err(what), _) => mistakes.push(Mistake { line: index + 1, what }),
                    (Ok(_), None) => {}
                },
            }
        }
    }
    if mistakes.is_empty() {
        Ok(found)
    } else {
        Err(mistakes)
    }
}

/// The names in the rules that nothing in the document answers to: objects and sounds. They do no
/// harm, but a rule about a thing that is not there does nothing.
pub fn strangers(rules: &[Rule], objects: &[String], sounds: &[String]) -> Vec<String> {
    let mut missing: Vec<String> = rules.iter().flat_map(|rule| rule.objects()).filter(|name| !objects.iter().any(|known| known == name)).map(str::to_string).collect();
    let played = rules.iter().flat_map(|rule| rule.steps.iter().chain(rule.otherwise.iter())).filter(|step| step.kind == StepKind::Sound).map(|step| step.a.clone());
    missing.extend(played.filter(|name| !sounds.contains(name)));
    missing.sort();
    missing.dedup();
    missing
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::{set_language, Language};

    #[test]
    fn a_sum_is_read_or_told_what_is_wrong_with_it() {
        set_language(Language::English);
        let numbers = vec!["Score".to_string(), "Lives".to_string()];
        assert_eq!(amount("12", &numbers), Ok(Amount::Plain(12.0)));
        assert_eq!(amount("0,5", &numbers), Ok(Amount::Plain(0.5)));
        assert_eq!(amount("Score * 2 + 1", &numbers), Ok(Amount::Sum("Score * 2 + 1".to_string())));
        assert_eq!(amount("Hero.x + 10", &numbers), Ok(Amount::Sum("Hero.x + 10".to_string())));
        assert_eq!(amount("\"Giant Rat\".y - (Lives + 1)", &numbers), Ok(Amount::Sum("\"Giant Rat\".y - (Lives + 1)".to_string())));
        assert_eq!(amount("random(1, 6)", &numbers), Ok(Amount::Sum("random(1, 6)".to_string())));
        assert_eq!(amount("Points + 1", &numbers), Err("there is no number called Points".to_string()));
        assert_eq!(amount("Score +", &numbers), Err("a number or a name is missing".to_string()));
        assert_eq!(amount("(Score", &numbers), Err("a bracket is not closed".to_string()));
        assert_eq!(amount("Hero.width", &numbers), Err("after Hero. comes x or y".to_string()));
        assert_eq!(objects_in("Hero.x + \"Giant Rat\".y - Score"), vec!["Hero", "Giant Rat"]);
    }

    #[test]
    fn a_sentence_fits_a_line_or_does_not() {
        assert_eq!(fit("When {} touches {}", "When Apple touches Basket"), Some(vec!["Apple".to_string(), "Basket".to_string()]));
        assert_eq!(fit("put {} at {}, {}", "put Hero at random(1, 6), Score * 2"), Some(vec!["Hero".to_string(), "random(1, 6)".to_string(), "Score * 2".to_string()]));
        assert_eq!(fit("stop the film", "stop the film"), Some(Vec::new()));
        assert_eq!(fit("stop the film", "stop the film now"), None);
        assert_eq!(fit("show {}", "hide Apple"), None);
    }

    const GAME: &str = "\
# A game in a few lines.
Numbers:
    Score = 0
    Lives = 3

When Apple touches Basket:
    increase Score by 1
    put Apple at the top, at random

When Hero touches Door, if Score is more than 2 and Lives is more than 0:
    play the sound fanfare
    wait 1 seconds
    go to scene 2
otherwise:
    put Hero at Hero.x - 40, 262

When the Space key is pressed:
    make a copy of Bullet at Ship
    set Score to Score * 2 + random(1, 6)
";

    #[test]
    fn a_text_is_read_into_rules_and_written_back_the_same() {
        set_language(Language::English);
        let found = read(GAME).unwrap();
        assert_eq!(found.variables, vec![Variable { name: "Score".into(), start: 0.0 }, Variable { name: "Lives".into(), start: 3.0 }]);
        assert_eq!(found.rules.len(), 3);
        // The door: two conditions, three steps, and one to take otherwise, with a sum that asks where the hero is.
        let door = &found.rules[1];
        assert_eq!((door.event.kind, door.event.a.as_str(), door.event.b.as_str()), (EventKind::Touch, "Hero", "Door"));
        assert_eq!(door.conditions().map(|only| (only.number.as_str(), only.how.as_str(), only.value)).collect::<Vec<_>>(), vec![("Score", ">", 2.0), ("Lives", ">", 0.0)]);
        assert_eq!(door.steps.iter().map(|step| step.kind).collect::<Vec<_>>(), vec![StepKind::Sound, StepKind::Wait, StepKind::GotoScene]);
        assert_eq!((door.otherwise[0].kind, door.otherwise[0].ex.as_str(), door.otherwise[0].y), (StepKind::GoTo, "Hero.x - 40", 262.0));
        assert_eq!(found.rules[2].steps[1].ex, "Score * 2 + random(1, 6)");
        // Written out and read again, it is the same; so is the writing, but for the remark at the top.
        let written = write(&found.variables, &found.rules);
        assert_eq!(read(&written).unwrap(), found);
        assert_eq!(written.trim(), GAME.lines().skip(1).collect::<Vec<_>>().join("\n").trim());
        // The page is told of the conditions, of what to do otherwise, and of the sums as they are written.
        let page = door.text(&["fanfare".to_string()]);
        assert!(page.contains("\"c\":[[\"Score\",\">\",2],[\"Lives\",\">\",0]]") && page.contains("\"o\":[[\"goto\",\"Hero\",\"Hero.x - 40\",262,\"\"]]") && page.contains("[\"wait\",\"\",1,0,\"\"]"), "{page}");
        assert_eq!(strangers(&found.rules, &["Apple".to_string(), "Basket".to_string(), "Hero".to_string()], &[]), vec!["Bullet", "Door", "Ship", "fanfare"]);
    }

    /// Writes a page whose rules use what only the written language has, to try in a browser:
    /// `cargo test -- --ignored language_page`.
    #[test]
    #[ignore]
    fn language_page() {
        set_language(Language::English);
        let mut e = crate::model::Editor::new(crate::model::Document::new(300, 200));
        e.doc.page_mut().start_timeline(20.0);
        let mut object = crate::model::Object::new_web(e.alloc_id(), crate::model::WebKind::Slice, crate::model::Rect { x: 20.0, y: 50.0, w: 40.0, h: 40.0 });
        (object.name, object.shape, object.fill) = ("Box".to_string(), crate::model::Shape::Rect { radius: 4.0 }, Some(crate::model::Color::rgb(30, 120, 210)));
        e.add_object(object);
        let found = read("Numbers:\n    Score = 0\n    Lives = 2\n\nWhen Box is clicked, if Lives is more than 0 and Score is less than 3:\n    increase Score by Lives * 2\n    put Box at Box.x + 30, 50\n    wait 0.5 seconds\n    set Lives to 0\notherwise:\n    set Score to random(100, 100) + Box.x\n").unwrap();
        (e.doc.rules, e.doc.variables) = (found.rules, found.variables);
        std::fs::write(concat!(env!("CARGO_MANIFEST_DIR"), "/target/dil.html"), crate::publish::html(&e.doc, "dil").unwrap()).unwrap();
    }

    #[test]
    fn what_cannot_be_read_is_told_with_its_line() {
        set_language(Language::English);
        let told = |text: &str| read(text).unwrap_err().into_iter().map(|m| (m.line, m.what)).collect::<Vec<_>>();
        assert_eq!(told("When Apple dances:\n    hide Apple\n"), vec![(1, "this is not an event I know".to_string())]);
        assert_eq!(told("When Apple is clicked:\n    make Apple sing\n"), vec![(2, "this is not a step I know".to_string())]);
        assert_eq!(told("When Apple is clicked:\n    increase Score by 1\n"), vec![(2, "there is no number called Score".to_string())]);
        assert_eq!(told("When Apple is clicked\n"), vec![(1, "a line that starts at the edge is an event and ends with a colon; steps are written further in".to_string())]);
        assert_eq!(told("Numbers:\n    Score = lots\n"), vec![(2, "lots is not a number".to_string())]);
        assert_eq!(told("otherwise:\n    hide Apple\n"), vec![(1, "\"otherwise\" comes after the steps of a rule".to_string())]);
        // Capitals at the start of a step do no harm, and neither do tabs or a decimal comma.
        let found = read("Numbers:\n\tTime = 2,5\n\nEvery 0,5 seconds:\n\tDecrease Time by 0,5\n").unwrap();
        assert_eq!((found.variables[0].start, found.rules[0].event.n, found.rules[0].steps[0].x), (2.5, 0.5, -0.5));
    }

    #[test]
    fn every_game_that_comes_with_the_program_reads_back_from_its_own_text() {
        // In English here; `tests/turkce.rs` does the same in Turkish, in a run of its own, since the language
        // is one for the whole program and the tests beside this one read English.
        for language in [Language::English] {
            set_language(language);
            for example in crate::sahne_examples::EXAMPLES {
                let doc = (example.build)();
                if doc.rules.is_empty() {
                    continue;
                }
                let written = write(&doc.variables, &doc.rules);
                let found = read(&written).unwrap_or_else(|mistakes| panic!("{} ({language:?}): {mistakes:?}\n{written}", example.name));
                assert_eq!(found.rules, doc.rules, "{} ({language:?})\n{written}", example.name);
                assert_eq!(found.variables, doc.variables, "{}", example.name);
                assert!(strangers(&doc.rules, &doc.object_names(), &doc.sounds.iter().map(|s| s.name.clone()).collect::<Vec<_>>()).is_empty(), "{}", example.name);
            }
        }
        set_language(Language::English);
    }
}
