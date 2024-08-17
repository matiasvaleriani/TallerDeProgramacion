use super::regex_rep::RegexRep;
use super::regex_val::RegexVal;

#[derive(Debug, Clone)]
/// Estructura que representa un paso de la expresión regular.
pub struct RegexStep {
    pub val: RegexVal, //valor del caracter
    pub rep: RegexRep, //repeticion del caracter
}
