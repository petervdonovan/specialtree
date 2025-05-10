// #[derive(Debug)]
// pub struct Unparse {
//     lines: Vec<String>,
// }

// impl Unparse {
//     pub fn new(start: &str) -> Unparse {
//         if start.contains('\n') {
//             panic!("UnparseResult::new only works on single-line strings");
//         }
//         if start.is_empty() {
//             panic!("UnparseResult::new only works on non-empty strings");
//         }
//         if start.chars().next().unwrap().is_whitespace()
//             || start.chars().last().unwrap().is_whitespace()
//         {
//             panic!(
//                 "UnparseResult::new only works on strings without leading or trailing whitespace"
//             );
//         }
//         Unparse {
//             lines: vec![start.to_string()],
//         }
//     }
//     pub fn render(&self) -> String {
//         self.lines.join("\n")
//     }
//     pub fn indent(&mut self) {
//         for line in self.lines.iter_mut() {
//             *line = format!("  {}", line);
//         }
//     }
//     pub fn hstack(&mut self, other: Unparse) {
//         if self.lines.len() != 1 || other.lines.len() != 1 {
//             panic!("hstack only works on single-line unparse results");
//         }
//         self.lines[0].push(' ');
//         self.lines[0].push_str(&other.lines[0]);
//     }
//     pub fn vstack(&mut self, other: Unparse) {
//         self.lines.extend(other.lines);
//     }
//     pub fn width(&self) -> usize {
//         self.lines.iter().map(|it| it.len()).max().unwrap_or(0)
//     }
//     pub fn height(&self) -> usize {
//         self.lines.len()
//     }
// }

// impl std::fmt::Display for Unparse {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.render())
//     }
// }
#[derive(Debug)]
pub struct Unparse<'arena> {
    arena: &'arena bumpalo::Bump,
    events: Vec<UnparseEvent<'arena>>,
}
#[derive(Debug)]
pub enum UnparseEvent<'arena> {
    // see http://i.stanford.edu/pub/cstr/reports/cs/tr/79/770/CS-TR-79-770.pdf,
    // https://docs.rs/prettyplease/latest/prettyplease/
    ConsistentBreakingGroupStart,
    InconsistentBreakingGroupStart,
    GroupEnd,
    StaticText(&'static str),
    DynamicText(&'arena str),
    Break,
}
impl<'arena> Unparse<'arena> {
    pub fn new(arena: &'arena bumpalo::Bump) -> Self {
        Self {
            arena,
            events: Vec::new(),
        }
    }
    pub fn consistent_group_start(&mut self) {
        self.events.push(UnparseEvent::ConsistentBreakingGroupStart);
    }
    pub fn inconsistent_group_start(&mut self) {
        self.events
            .push(UnparseEvent::InconsistentBreakingGroupStart);
    }
    pub fn group_end(&mut self) {
        self.events.push(UnparseEvent::GroupEnd);
    }
    pub fn static_text(&mut self, text: &'static str) {
        self.events.push(UnparseEvent::StaticText(text));
    }
    pub fn dynamic_text(&mut self, text: String) {
        self.events
            .push(UnparseEvent::DynamicText(self.arena.alloc(text)));
    }
}
