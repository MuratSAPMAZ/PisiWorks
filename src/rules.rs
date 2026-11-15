//! Rules: what makes a film a game. "When this happens, do that", picked from lists.
//!
//! PisiSahne's own, the step after behaviors. A rule is an event and the steps to take
//! when it happens; both name the objects they are about by the names the objects have
//! in the document, so one rule about "Apple" holds for every apple. Numbers (a score,
//! lives, a clock) are kept by name as well, and a text that has `{Score}` in it shows
//! the number in the film.
//!
//! Every rule can be read as a sentence, in the language of the interface. Those
//! sentences are the beginning of a written language: the lists only write them for you.
//! The ideas are kept to an object model on purpose (an object, an event on it, a step
//! done to it, a number), so that a programming language can be laid over them later.
//!
//! Rules run in the published film, as buttons do.

use serde::{Deserialize, Serialize};

use crate::i18n::{t, tf};

/// What a rule waits for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventKind {
    /// The film starts, or starts again.
    Start,
    /// A key goes down: `a` is the key.
    Key,
    /// An object is clicked: `a` is its name.
    Click,
    /// An object comes to touch another: `a` and `b` are their names.
    Touch,
    /// An object comes to touch an edge of the stage: `a` is its name, `b` the edge.
    Edge,
    /// Again and again, every `n` seconds.
    Every,
    /// Once, `n` seconds after the film starts.
    After,
    /// A number comes to be, to pass or to fall under `n`: `a` is its name, `b` which of the three.
    Number,
}

pub const EVENTS: [EventKind; 8] = [EventKind::Start, EventKind::Key, EventKind::Click, EventKind::Touch, EventKind::Edge, EventKind::Every, EventKind::After, EventKind::Number];

/// What a name in a rule is chosen from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pick {
    Object,
    Key,
    Edge,
    Compare,
    Variable,
    Sound,
}

impl EventKind {
    /// The name in the list, in English.
    pub const fn name(self) -> &'static str {
        match self {
            EventKind::Start => "When the film starts",
            EventKind::Key => "When a key is pressed",
            EventKind::Click => "When an object is clicked",
            EventKind::Touch => "When an object touches another",
            EventKind::Edge => "When an object touches the edge of the stage",
            EventKind::Every => "Every so many seconds",
            EventKind::After => "After so many seconds",
            EventKind::Number => "When a number reaches a value",
        }
    }

    /// The word the published page's script knows the kind by.
    pub const fn key(self) -> &'static str {
        match self {
            EventKind::Start => "start",
            EventKind::Key => "key",
            EventKind::Click => "click",
            EventKind::Touch => "touch",
            EventKind::Edge => "edge",
            EventKind::Every => "every",
            EventKind::After => "after",
            EventKind::Number => "number",
        }
    }

    /// What the kind's first and second names are chosen from, and what its number is called, if it has them.
    pub const fn needs(self) -> (Option<Pick>, Option<Pick>, Option<&'static str>) {
        match self {
            EventKind::Start => (None, None, None),
            EventKind::Key => (Some(Pick::Key), None, None),
            EventKind::Click => (Some(Pick::Object), None, None),
            EventKind::Touch => (Some(Pick::Object), Some(Pick::Object), None),
            EventKind::Edge => (Some(Pick::Object), Some(Pick::Edge), None),
            EventKind::Every | EventKind::After => (None, None, Some("seconds")),
            EventKind::Number => (Some(Pick::Variable), Some(Pick::Compare), Some("")),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub kind: EventKind,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub a: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub b: String,
    #[serde(default)]
    pub n: f32,
}

/// What a rule does.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum StepKind {
    /// Moves the object `a` by `x` to the right and `y` down.
    Move,
    /// Puts the object's top left corner at `x`, `y`.
    GoTo,
    /// Puts the object somewhere on the stage at random.
    Random,
    /// Puts the object at the top of the stage, at a random place along it, and stops its fall.
    RandomTop,
    Show,
    Hide,
    /// Throws the object up at `x` pixels a second; it comes down if it has Fall and Jump.
    Jump,
    /// Adds `x` to the number `a`.
    Add,
    /// Makes the number `a` be `x`.
    Set,
    StopFilm,
    PlayFilm,
    /// Goes to frame `x` of the scene, counted from 1, and plays on.
    GotoFrame,
    /// Goes to frame `x` and stops there.
    GotoFrameStop,
    /// Goes to scene `x`, counted from 1.
    GotoScene,
    NextScene,
    /// Plays the sound called `a`.
    Sound,
    /// Stops the film and everything that moves in it.
    StopAll,
    /// Starts the film again from the beginning, with every number as it started.
    Restart,
}

pub const STEPS: [StepKind; 18] = [
    StepKind::Add,
    StepKind::Set,
    StepKind::Hide,
    StepKind::Show,
    StepKind::Move,
    StepKind::GoTo,
    StepKind::Random,
    StepKind::RandomTop,
    StepKind::Jump,
    StepKind::Sound,
    StepKind::NextScene,
    StepKind::GotoScene,
    StepKind::GotoFrame,
    StepKind::GotoFrameStop,
    StepKind::StopFilm,
    StepKind::PlayFilm,
    StepKind::StopAll,
    StepKind::Restart,
];

impl StepKind {
    pub const fn name(self) -> &'static str {
        match self {
            StepKind::Move => "Move an object",
            StepKind::GoTo => "Put an object at a place",
            StepKind::Random => "Put an object somewhere at random",
            StepKind::RandomTop => "Put an object at the top, at random",
            StepKind::Show => "Show an object",
            StepKind::Hide => "Hide an object",
            StepKind::Jump => "Make an object jump",
            StepKind::Add => "Change a number",
            StepKind::Set => "Set a number",
            StepKind::StopFilm => "Stop the film",
            StepKind::PlayFilm => "Play the film",
            StepKind::GotoFrame => "Go to a frame and play",
            StepKind::GotoFrameStop => "Go to a frame and stop",
            StepKind::GotoScene => "Go to a scene",
            StepKind::NextScene => "Go to the next scene",
            StepKind::Sound => "Play a sound",
            StepKind::StopAll => "Stop everything",
            StepKind::Restart => "Start again",
        }
    }

    pub const fn key(self) -> &'static str {
        match self {
            StepKind::Move => "move",
            StepKind::GoTo => "goto",
            StepKind::Random => "random",
            StepKind::RandomTop => "top",
            StepKind::Show => "show",
            StepKind::Hide => "hide",
            StepKind::Jump => "jump",
            StepKind::Add => "add",
            StepKind::Set => "set",
            StepKind::StopFilm => "stop",
            StepKind::PlayFilm => "play",
            StepKind::GotoFrame => "frame",
            StepKind::GotoFrameStop => "frame-stop",
            StepKind::GotoScene => "scene",
            StepKind::NextScene => "next",
            StepKind::Sound => "sound",
            StepKind::StopAll => "halt",
            StepKind::Restart => "restart",
        }
    }

    /// What the kind's name is chosen from, and what its two numbers are called and start as, if it has them.
    pub const fn needs(self) -> (Option<Pick>, Option<(&'static str, f32)>, Option<(&'static str, f32)>) {
        match self {
            StepKind::Move => (Some(Pick::Object), Some(("Rightwards:", 10.0)), Some(("Down:", 0.0))),
            StepKind::GoTo => (Some(Pick::Object), Some(("X:", 0.0)), Some(("Y:", 0.0))),
            StepKind::Random | StepKind::RandomTop | StepKind::Show | StepKind::Hide => (Some(Pick::Object), None, None),
            StepKind::Jump => (Some(Pick::Object), Some(("Speed:", 400.0)), None),
            StepKind::Add => (Some(Pick::Variable), Some(("By:", 1.0)), None),
            StepKind::Set => (Some(Pick::Variable), Some(("Value:", 0.0)), None),
            StepKind::GotoFrame | StepKind::GotoFrameStop => (None, Some(("Frame:", 1.0)), None),
            StepKind::GotoScene => (None, Some(("Scene:", 1.0)), None),
            StepKind::Sound => (Some(Pick::Sound), None, None),
            StepKind::StopFilm | StepKind::PlayFilm | StepKind::NextScene | StepKind::StopAll | StepKind::Restart => (None, None, None),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Step {
    pub kind: StepKind,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub a: String,
    #[serde(default)]
    pub x: f32,
    #[serde(default)]
    pub y: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rule {
    pub event: Event,
    #[serde(default)]
    pub steps: Vec<Step>,
}

/// A number the film keeps, such as a score: its name, and what it is when the film starts.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Variable {
    pub name: String,
    #[serde(default)]
    pub start: f32,
}

/// The keys a rule can wait for: what the browser calls the key, and its name in English.
pub const KEYS: [(&str, &str); 42] = [
    (" ", "Space"),
    ("ArrowUp", "Up Arrow"),
    ("ArrowDown", "Down Arrow"),
    ("ArrowLeft", "Left Arrow"),
    ("ArrowRight", "Right Arrow"),
    ("Enter", "Enter"),
    ("a", "A"),
    ("b", "B"),
    ("c", "C"),
    ("d", "D"),
    ("e", "E"),
    ("f", "F"),
    ("g", "G"),
    ("h", "H"),
    ("i", "I"),
    ("j", "J"),
    ("k", "K"),
    ("l", "L"),
    ("m", "M"),
    ("n", "N"),
    ("o", "O"),
    ("p", "P"),
    ("q", "Q"),
    ("r", "R"),
    ("s", "S"),
    ("t", "T"),
    ("u", "U"),
    ("v", "V"),
    ("w", "W"),
    ("x", "X"),
    ("y", "Y"),
    ("z", "Z"),
    ("0", "0"),
    ("1", "1"),
    ("2", "2"),
    ("3", "3"),
    ("4", "4"),
    ("5", "5"),
    ("6", "6"),
    ("7", "7"),
    ("8", "8"),
    ("9", "9"),
];

/// The edges of the stage: the word the script knows each by, and its name in English.
pub const EDGES: [(&str, &str); 5] = [("any", "any edge"), ("bottom", "the bottom edge"), ("top", "the top edge"), ("left", "the left edge"), ("right", "the right edge")];

/// The ways a number is held against a value.
pub const COMPARES: [(&str, &str); 3] = [("=", "becomes"), (">", "goes above"), ("<", "goes below")];

/// The name in the interface's language of a choice that is not an object's or a number's own name.
pub fn choice_name(pick: Pick, value: &str) -> String {
    let list: &[(&str, &str)] = match pick {
        Pick::Key => &KEYS,
        Pick::Edge => &EDGES,
        Pick::Compare => &COMPARES,
        Pick::Object | Pick::Variable | Pick::Sound => return value.to_string(),
    };
    list.iter().find(|(key, _)| *key == value).map_or_else(|| value.to_string(), |(_, name)| if name.len() == 1 { name.to_string() } else { t(name).to_string() })
}

/// A number as a sentence has it: whole where it is whole.
fn plain(n: f32) -> String {
    if n.fract() == 0.0 { format!("{}", n as i64) } else { format!("{n}") }
}

impl Event {
    pub fn new(kind: EventKind) -> Event {
        Event { kind, a: String::new(), b: String::new(), n: if matches!(kind, EventKind::Every | EventKind::After) { 1.0 } else { 0.0 } }
    }

    pub fn sentence(&self) -> String {
        let (a, n) = (&self.a, plain(self.n));
        match self.kind {
            EventKind::Start => t("When the film starts").to_string(),
            EventKind::Key => tf("When the {} key is pressed", &[&choice_name(Pick::Key, a)]),
            EventKind::Click => tf("When {} is clicked", &[a]),
            EventKind::Touch => tf("When {} touches {}", &[a, &self.b]),
            EventKind::Edge => tf("When {} touches {} of the stage", &[a, &choice_name(Pick::Edge, &self.b)]),
            EventKind::Every => tf("Every {} seconds", &[&n]),
            EventKind::After => tf("After {} seconds", &[&n]),
            EventKind::Number => match self.b.as_str() {
                ">" => tf("When {} goes above {}", &[a, &n]),
                "<" => tf("When {} goes below {}", &[a, &n]),
                _ => tf("When {} becomes {}", &[a, &n]),
            },
        }
    }

    fn text(&self) -> String {
        format!("[\"{}\",{},{},{}]", self.kind.key(), quoted(&self.a), quoted(&self.b), self.n)
    }
}

fn quoted(text: &str) -> String {
    serde_json::to_string(text).unwrap_or_else(|_| "\"\"".to_string())
}

impl Step {
    pub fn new(kind: StepKind) -> Step {
        let (_, x, y) = kind.needs();
        Step { kind, a: String::new(), x: x.map_or(0.0, |x| x.1), y: y.map_or(0.0, |y| y.1) }
    }

    pub fn sentence(&self) -> String {
        let (a, x, y) = (&self.a, plain(self.x), plain(self.y));
        match self.kind {
            StepKind::Move => tf("move {} by {} right and {} down", &[a, &x, &y]),
            StepKind::GoTo => tf("put {} at {}, {}", &[a, &x, &y]),
            StepKind::Random => tf("put {} somewhere at random", &[a]),
            StepKind::RandomTop => tf("put {} at the top, at random", &[a]),
            StepKind::Show => tf("show {}", &[a]),
            StepKind::Hide => tf("hide {}", &[a]),
            StepKind::Jump => tf("make {} jump at {}", &[a, &x]),
            StepKind::Add => tf("change {} by {}", &[a, &x]),
            StepKind::Set => tf("set {} to {}", &[a, &x]),
            StepKind::StopFilm => t("stop the film").to_string(),
            StepKind::PlayFilm => t("play the film").to_string(),
            StepKind::GotoFrame => tf("go to frame {} and play", &[&x]),
            StepKind::GotoFrameStop => tf("go to frame {} and stop", &[&x]),
            StepKind::GotoScene => tf("go to scene {}", &[&x]),
            StepKind::NextScene => t("go to the next scene").to_string(),
            StepKind::Sound => tf("play the sound {}", &[a]),
            StepKind::StopAll => t("stop everything").to_string(),
            StepKind::Restart => t("start again").to_string(),
        }
    }
}

impl Rule {
    pub fn new() -> Rule {
        Rule { event: Event::new(EventKind::Start), steps: Vec::new() }
    }

    /// The rule as one sentence: the event, then the steps.
    pub fn sentence(&self) -> String {
        let steps: Vec<String> = self.steps.iter().map(Step::sentence).collect();
        if steps.is_empty() {
            format!("{}: …", self.event.sentence())
        } else {
            format!("{}: {}", self.event.sentence(), steps.join("; "))
        }
    }

    /// The names of the objects the rule is about.
    pub fn objects(&self) -> Vec<&str> {
        let (a, b, _) = self.event.kind.needs();
        let mut names = Vec::new();
        if a == Some(Pick::Object) {
            names.push(self.event.a.as_str());
        }
        if b == Some(Pick::Object) {
            names.push(self.event.b.as_str());
        }
        names.extend(self.steps.iter().filter(|step| step.kind.needs().0 == Some(Pick::Object)).map(|step| step.a.as_str()));
        names.retain(|name| !name.is_empty());
        names
    }

    /// What the script of the published page reads. A sound is given by its place in `sounds`.
    pub fn text(&self, sounds: &[String]) -> String {
        let steps: Vec<String> = self
            .steps
            .iter()
            .map(|step| {
                let x = if step.kind == StepKind::Sound { sounds.iter().position(|name| *name == step.a).map_or(-1.0, |at| at as f32) } else { step.x };
                format!("[\"{}\",{},{},{}]", step.kind.key(), quoted(&step.a), x, step.y)
            })
            .collect();
        format!("{{\"e\":{},\"s\":[{}]}}", self.event.text(), steps.join(","))
    }
}

impl Default for Rule {
    fn default() -> Self {
        Rule::new()
    }
}

/// The names of numbers that a text shows: every `{name}` in it that is the name of one.
pub fn shown_in<'a>(content: &str, variables: &'a [Variable]) -> Vec<&'a str> {
    variables.iter().filter(|variable| content.contains(&format!("{{{}}}", variable.name))).map(|variable| variable.name.as_str()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rule() -> Rule {
        Rule {
            event: Event { kind: EventKind::Touch, a: "Elma".into(), b: "Sepet".into(), n: 0.0 },
            steps: vec![Step { kind: StepKind::Add, a: "Puan".into(), x: 1.0, y: 0.0 }, Step { kind: StepKind::RandomTop, a: "Elma".into(), x: 0.0, y: 0.0 }, Step { kind: StepKind::Sound, a: "zil".into(), x: 0.0, y: 0.0 }],
        }
    }

    #[test]
    fn a_rule_reads_as_a_sentence_and_names_its_objects() {
        crate::i18n::set_language(crate::i18n::Language::English);
        let rule = rule();
        assert_eq!(rule.sentence(), "When Elma touches Sepet: change Puan by 1; put Elma at the top, at random; play the sound zil");
        assert_eq!(rule.objects(), vec!["Elma", "Sepet", "Elma"]);
        let key = Event { kind: EventKind::Key, a: " ".into(), b: String::new(), n: 0.0 };
        assert_eq!(key.sentence(), "When the Space key is pressed");
        let number = Event { kind: EventKind::Number, a: "Can".into(), b: "<".into(), n: 1.0 };
        assert_eq!(number.sentence(), "When Can goes below 1");
        assert_eq!(Rule::new().sentence(), "When the film starts: …");
    }

    #[test]
    fn the_page_gets_a_rule_with_its_sound_by_number() {
        let text = rule().text(&["davul".to_string(), "zil".to_string()]);
        assert_eq!(text, "{\"e\":[\"touch\",\"Elma\",\"Sepet\",0],\"s\":[[\"add\",\"Puan\",1,0],[\"top\",\"Elma\",0,0],[\"sound\",\"zil\",1,0]]}");
    }

    #[test]
    fn every_kind_has_its_own_word_and_a_text_tells_its_numbers() {
        let mut words: Vec<&str> = EVENTS.iter().map(|kind| kind.key()).chain(STEPS.iter().map(|kind| kind.key())).collect();
        let count = words.len();
        words.sort();
        words.dedup();
        assert_eq!(words.len(), count);
        let numbers = [Variable { name: "Puan".into(), start: 0.0 }, Variable { name: "Can".into(), start: 3.0 }];
        assert_eq!(shown_in("Puan: {Puan}", &numbers), vec!["Puan"]);
        assert!(shown_in("Puan", &numbers).is_empty());
    }
}
