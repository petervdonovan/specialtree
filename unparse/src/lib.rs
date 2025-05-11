use std::fmt::Write;

pub struct Unparse<'arena> {
    arena: &'arena bumpalo::Bump,
    events: Vec<UnparseEvent<'arena>>,
    stack: Vec<usize>,
    n_chars: usize,
}
#[derive(Clone, Copy)]
pub enum BreakingStrategy {
    Consistent,
    Inconsistent,
}

#[derive(Clone, Copy)]
enum UnparseEvent<'arena> {
    // see http://i.stanford.edu/pub/cstr/reports/cs/tr/79/770/CS-TR-79-770.pdf,
    // https://docs.rs/prettyplease/latest/prettyplease/
    GroupStart(GroupData),
    StaticText(&'static str),
    DynamicText(&'arena str),
    Break,
}
#[derive(Clone, Copy)]
struct GroupData {
    n_events: usize,
    n_chars: usize,
    bs: BreakingStrategy,
}

impl<'arena> Unparse<'arena> {
    pub fn new(arena: &'arena bumpalo::Bump) -> Self {
        Self {
            arena,
            events: Vec::new(),
            stack: vec![],
            n_chars: 0,
        }
    }
    pub fn consistent_group_start(&mut self) {
        self.stack.push(self.events.len());
        self.events.push(UnparseEvent::GroupStart(GroupData {
            n_events: self.events.len(),
            n_chars: self.n_chars,
            bs: BreakingStrategy::Consistent,
        }));
    }
    pub fn inconsistent_group_start(&mut self) {
        self.stack.push(self.events.len());
        self.events.push(UnparseEvent::GroupStart(GroupData {
            n_events: self.events.len(),
            n_chars: self.n_chars,
            bs: BreakingStrategy::Inconsistent,
        }));
    }
    pub fn group_end(&mut self) {
        let pop = self.stack.pop().unwrap();
        let current_n_events = self.events.len();
        match self.events.get_mut(pop).unwrap() {
            UnparseEvent::GroupStart(gd) => {
                gd.n_events = current_n_events - gd.n_events;
                gd.n_chars = self.n_chars - gd.n_chars;
            }
            _ => unreachable!(),
        };
    }
    pub fn static_text(&mut self, text: &'static str) {
        self.n_chars += text.len() + 1;
        self.events.push(UnparseEvent::StaticText(text));
    }
    pub fn dynamic_text(&mut self, text: String) {
        self.n_chars += text.len() + 1;
        self.events
            .push(UnparseEvent::DynamicText(self.arena.alloc(text)));
    }
    pub fn linebreak(&mut self) {
        self.events.push(UnparseEvent::Break);
    }
}

impl<'arena> std::fmt::Display for Unparse<'arena> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        breaking_fmt(self.events.as_slice(), f, 0, BreakingStrategy::Consistent)
    }
}

const TARGET_LINE_WIDTH: usize = 100;
const INDENTATION_PER_LEVEL: usize = 2;
const INDENTATION: &str = "\n                                                                                                    ";

fn should_break(gd: GroupData, current_line_width: usize, indentation: usize) -> bool {
    gd.n_chars + indentation + current_line_width > TARGET_LINE_WIDTH
}

fn recurse_group<'a, 'arena>(
    events: &'a [UnparseEvent<'arena>],
    f: &mut std::fmt::Formatter<'_>,
    current_line_width: usize,
    indentation: usize,
    gd: GroupData,
) -> std::fmt::Result {
    if should_break(gd, current_line_width, indentation) {
        breaking_fmt(
            &events[1..gd.n_events],
            f,
            indentation + INDENTATION_PER_LEVEL,
            gd.bs,
        )
    } else {
        non_breaking_fmt(&events[..gd.n_events], f)
    }
}
fn breaking_fmt<'a, 'arena>(
    mut events: &'a [UnparseEvent<'arena>],
    f: &mut std::fmt::Formatter<'_>,
    indentation: usize,
    bs: BreakingStrategy,
) -> std::fmt::Result {
    let mut current_line_width = 0;
    let mut first_in_line = true;
    while !events.is_empty() {
        match events[0] {
            UnparseEvent::GroupStart(gd) => {
                recurse_group(events, f, current_line_width, indentation, gd)?;
                events = &events[gd.n_events..];
            }
            UnparseEvent::StaticText(s) | UnparseEvent::DynamicText(s) => {
                if !first_in_line {
                    f.write_char(' ')?;
                }
                f.write_str(s)?;
                current_line_width += s.len() + 1;
                events = &events[1..];
            }
            UnparseEvent::Break => {
                if current_line_width >= TARGET_LINE_WIDTH
                    || matches!(bs, BreakingStrategy::Consistent)
                {
                    f.write_str(&INDENTATION[..indentation + 1])?;
                    current_line_width = 0;
                    first_in_line = true;
                }
                events = &events[1..];
            }
        }
    }
    Ok(())
}

fn non_breaking_fmt<'a, 'arena>(
    events: &'a [UnparseEvent<'arena>],
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    let mut first = true;
    for event in events {
        match event {
            UnparseEvent::StaticText(s) | UnparseEvent::DynamicText(s) => {
                if !first {
                    f.write_char(' ')?;
                }
                f.write_str(s)?;
                first = false;
            }
            _ => {}
        }
    }
    Ok(())
}
