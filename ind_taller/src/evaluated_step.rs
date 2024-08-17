use super::regex_step::RegexStep;

#[derive(Debug)]
/// Estructura que representa un paso evaluado de la expresión regular.
pub struct EvaluatedStep {
    pub step: RegexStep,
    pub match_size: usize,
    pub backtrackeable: bool,
}
