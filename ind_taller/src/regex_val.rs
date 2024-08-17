#[derive(Debug, Clone)]
/// Enum de valores que puede matchear
pub enum RegexVal {
    // valor que vamos a matchear
    Literal(char),            //caracter literal
    Wildcard,                 //comodin
    Bracket(Vec<char>, bool), //manejar las expresiones entre corchetes
    Clase(String, bool),      //clase de caracteres, alpha, digit, etc
    Inicio,                   //inicio de la linea
    Fin,                      //final de la linea
}

impl RegexVal {
    pub fn matches(&self, value: &str) -> usize {
        match self {
            RegexVal::Literal(l) => self.match_literal(l, value),
            RegexVal::Wildcard => self.match_wildcard(value),
            RegexVal::Bracket(chars, negado) => self.match_bracket(chars, negado, value),
            RegexVal::Clase(clase, negado) => self.match_clase(clase, negado, value),
            RegexVal::Inicio => 0,
            RegexVal::Fin => 0,
        }
    }

    fn match_literal(&self, l: &char, value: &str) -> usize {
        if value.starts_with(*l) {
            l.len_utf8() //cantidad consumida en el input
        } else {
            0
        }
    }

    fn match_wildcard(&self, value: &str) -> usize {
        if let Some(c) = value.chars().next() {
            c.len_utf8()
        } else {
            0
        }
    }

    fn match_bracket(&self, chars: &[char], negado: &bool, value: &str) -> usize {
        if let Some(c) = value.chars().next() {
            if chars.contains(&c) ^ negado {
                c.len_utf8()
            } else {
                0
            }
        } else {
            0
        }
    }

    fn match_clase(&self, clase: &str, negado: &bool, value: &str) -> usize {
        if let Some(c) = value.chars().next() {
            let matches = match clase {
                "alnum" => c.is_alphanumeric(),
                "alpha" => c.is_alphabetic(),
                "digit" => c.is_numeric(),
                "lower" => c.is_lowercase(),
                "upper" => c.is_uppercase(),
                "space" => c.is_whitespace(),
                "punct" => c.is_ascii_punctuation(),
                _ => false,
            };
            if matches ^ negado {
                c.len_utf8()
            } else {
                0
            }
        } else {
            0
        }
    }
}
