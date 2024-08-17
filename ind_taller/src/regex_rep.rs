#[derive(Debug, Clone)]
/// Enum de repeticiones que puede matchear
pub enum RegexRep {
    Any,
    Exact(usize),
    Range {
        min: Option<usize>,
        max: Option<usize>,
    },
}
