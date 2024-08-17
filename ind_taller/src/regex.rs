use super::evaluated_step::EvaluatedStep;
use super::regex_rep::RegexRep;
use super::regex_step::RegexStep;
use super::regex_val::RegexVal;
use std::collections::VecDeque;

/// Estructura que representa una expresión regular (Regular Expression).
#[derive(Debug)]
pub struct Regex {
    steps: Vec<RegexStep>,
    sub_regex: Vec<Regex>,
}

impl Regex {
    /// Función que se encarga de parsear el contenido de las llaves {} de una expresión regular.
    pub fn contenido_llaves(content: &str) -> Result<(Option<usize>, Option<usize>), &'static str> {
        let parts: Vec<&str> = content.split(',').collect();
        match parts.len() {
            1 => {
                let n = parts[0]
                    .parse::<usize>()
                    .map_err(|_| "No se pudo parsear el número")?;
                Ok((Some(n), None))
            }
            2 => {
                let min = if parts[0].is_empty() {
                    None
                } else {
                    Some(
                        parts[0]
                            .parse::<usize>()
                            .map_err(|_| "No se pudo parsear el número mínimo")?,
                    )
                };
                let max = if parts[1].is_empty() {
                    None
                } else {
                    Some(
                        parts[1]
                            .parse::<usize>()
                            .map_err(|_| "No se pudo parsear el número máximo")?,
                    )
                };
                if let (Some(min_val), Some(max_val)) = (min, max) {
                    if min_val > max_val {
                        return Err("El rango mínimo no puede ser mayor al rango máximo");
                    }
                }
                Ok((min, max))
            }
            _ => Err("Formato de llaves inválido"),
        }
    }

    /// Constructor de la estructura Regex.
    pub fn new(expression: &str) -> Result<Self, &str> {
        let parts: Vec<&str> = expression.split('|').collect();
        let mut all_parts: Vec<Regex> = vec![];

        for part in parts {
            let mut steps: Vec<RegexStep> = vec![];
            let mut chars_iter = part.chars();

            while let Some(c) = chars_iter.next() {
                let step = match c {
                    '^' => {
                        if steps.is_empty() {
                            evaluar_inicio_linea()
                        } else {
                            return Err("El caracter '^' no está al inicio de la regex");
                        }
                    }
                    '$' => {
                        if chars_iter.next().is_none() {
                            evaluar_fin_linea()
                        } else {
                            return Err("El caracter '$' no está al final de la regex");
                        }
                    }
                    '.' => evaluar_period(),
                    'a'..='z' | 'A'..='Z' | '0'..='9' | ' ' => evaluar_literal(c),
                    '+' => evaluar_mas(&mut steps)?,
                    '*' => evaluar_asterisco(&mut steps)?,
                    '?' => evaluar_interrogacion(&mut steps)?,
                    '\\' => evaluar_backslash(&mut chars_iter)?,
                    '{' => evaluar_llaves(&mut chars_iter, &mut steps)?,
                    '[' => evaluar_corchete(&mut chars_iter, &mut steps)?,
                    _ => return Err("Se encontró un caracter insesperado"),
                };

                if let Some(p) = step {
                    steps.push(p);
                }
            }
            all_parts.push(Regex {
                steps,
                sub_regex: vec![],
            });
        }
        Ok(Regex {
            steps: vec![],
            sub_regex: all_parts,
        })
    }

    fn manejar_exact(
        &self,
        step: &RegexStep,
        n: usize,
        value: &str,
        index: &mut usize,
        stack: &mut Vec<EvaluatedStep>,
        queue: &mut VecDeque<RegexStep>,
    ) -> Option<usize> {
        let mut match_size = 0;
        for _ in 0..n {
            let size = step.val.matches(&value[*index..]);

            if size == 0 {
                match backtrack(step.to_owned(), stack, queue) {
                    Some(back_size) => {
                        *index -= back_size;
                        return Some(*index);
                    }
                    None => return None,
                }
            } else {
                match_size += size;
                *index += size;
            }
        }
        stack.push(EvaluatedStep {
            //como voy a poner varias veces step en la cola, realmente necesito duplicarlo
            step: step.clone(),
            match_size,
            backtrackeable: false,
        });
        Some(*index)
    }

    fn manejar_any(
        &self,
        step: &RegexStep,
        value: &str,
        index: &mut usize,
        stack: &mut Vec<EvaluatedStep>,
    ) -> Option<usize> {
        let mut match_size = 0;
        let mut keep_matching = true;
        while keep_matching {
            let size = step.val.matches(&value[*index..]);
            if size != 0 {
                match_size += size;
                *index += size;
                stack.push(EvaluatedStep {
                    //como voy a poner varias veces step en la cola, realmente necesito duplicarlo
                    step: step.clone(),
                    match_size,
                    backtrackeable: true,
                });
            } else {
                keep_matching = false;
            }
        }
        Some(*index)
    }

    /// Función que se encarga de evaluar si un valor cumple con la expresión regular.
    ///
    /// # Errores
    ///
    /// Esta función retornará un error si el valor no es ASCII o si la expresión regular no es válida.
    pub fn test(&self, value: &str) -> Result<bool, &str> {
        if !value.is_ascii() {
            return Err("El valor no es ASCII");
        }

        let mut found_match = false;

        if self.sub_regex.is_empty() {
            'input: for start in 0..value.len() {
                //uso el clone porque es necesario para que no se pierda el valor de self.steps en cada llamada a test
                let mut queue = VecDeque::from(self.steps.clone());
                let mut stack = Vec::new();
                let mut index = start;
                let mut backtrack_count = 0;

                'steps: while let Some(step) = queue.pop_front() {
                    match &step.val {
                        RegexVal::Inicio => {
                            if index == 0 {
                                continue 'steps;
                            } else {
                                break 'steps;
                            }
                        }
                        RegexVal::Fin => {
                            if index == value.len() {
                                found_match = true;
                                continue 'input;
                            } else if queue.is_empty() {
                                continue 'input;
                            } else {
                                continue 'steps;
                            }
                        }
                        _ => {
                            match step.rep {
                                RegexRep::Exact(n) => {
                                    if self
                                        .manejar_exact(
                                            &step, n, value, &mut index, &mut stack, &mut queue,
                                        )
                                        .is_none()
                                    {
                                        break 'steps;
                                    }
                                }
                                RegexRep::Any => {
                                    if self
                                        .manejar_any(&step, value, &mut index, &mut stack)
                                        .is_none()
                                    {
                                        break 'steps;
                                    }
                                }
                                RegexRep::Range { min, max } => {
                                    let mut match_size = 0;
                                    let mut keep_matching = true;
                                    while keep_matching {
                                        let size = step.val.matches(&value[index..]);
                                        if size != 0 {
                                            match_size += size;
                                            index += size;
                                            stack.push(EvaluatedStep {
                                                //como voy a poner varias veces step en la cola, realmente necesito duplicarlo
                                                step: step.clone(),
                                                match_size: size,
                                                backtrackeable: true,
                                            });
                                        } else {
                                            keep_matching = false;
                                        }
                                        if let Some(max) = max {
                                            if match_size >= max {
                                                break;
                                            }
                                        }
                                    }
                                    if let Some(min) = min {
                                        if match_size < min {
                                            match backtrack(step, &mut stack, &mut queue) {
                                                Some(back_size) => {
                                                    index -= back_size;
                                                    backtrack_count += 1;
                                                    if backtrack_count > value.len() {
                                                        return Ok(false);
                                                    }
                                                    continue 'steps;
                                                }
                                                None => break 'steps,
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                if queue.is_empty() {
                    return Ok(true);
                }
            }
        } else {
            for regex in &self.sub_regex {
                if regex.test(value)? {
                    return Ok(true);
                }
            }
        }
        if found_match {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

/// Función que se encarga de hacer el backtrack
/// Esta función toma un paso actual, una lista de pasos evaluados y una lista de los próximos pasos.
/// Realiza un backtrack a través de los pasos evaluados, sumando el tamaño de la coincidencia de cada paso,
/// hasta que encuentra un paso que sea backtrackeable. En ese punto, devuelve el tamaño total de la coincidencia
/// de los pasos que ha retrocedido. Si no encuentra ningún paso backtrackeable, devuelve None.
pub fn backtrack(
    current: RegexStep,
    evaluated: &mut Vec<EvaluatedStep>,
    next: &mut VecDeque<RegexStep>,
) -> Option<usize> {
    let mut back_size = 0;

    next.push_front(current);
    while let Some(e) = evaluated.pop() {
        back_size += e.match_size;
        if e.backtrackeable {
            return Some(back_size);
        } else {
            next.push_front(e.step);
        }
    }
    None
}

/// devuelve la regex step para el inicio de la linea '^'
fn evaluar_inicio_linea() -> Option<RegexStep> {
    Some(RegexStep {
        rep: RegexRep::Exact(1),
        val: RegexVal::Inicio,
    })
}

/// devuelve la regex step para el final de la linea '$'
fn evaluar_fin_linea() -> Option<RegexStep> {
    Some(RegexStep {
        rep: RegexRep::Exact(1),
        val: RegexVal::Fin,
    })
}

/// devuelve la regex step para el comodin '.'
fn evaluar_period() -> Option<RegexStep> {
    Some(RegexStep {
        rep: RegexRep::Exact(1),
        val: RegexVal::Wildcard,
    })
}

/// devuelve la regex step para un literal
fn evaluar_literal(c: char) -> Option<RegexStep> {
    Some(RegexStep {
        rep: RegexRep::Exact(1),
        val: RegexVal::Literal(c),
    })
}

/// devuelve la regex step para el caracter '+'
fn evaluar_mas(steps: &mut [RegexStep]) -> Result<Option<RegexStep>, &'static str> {
    if let Some(last) = steps.last_mut() {
        last.rep = RegexRep::Range {
            min: Some(1),
            max: None,
        };
        Ok(None)
    } else {
        Err("Se encontró un caracter '+' insesperado")
    }
}

/// devuelve la regex step para el caracter '*'
fn evaluar_asterisco(steps: &mut [RegexStep]) -> Result<Option<RegexStep>, &'static str> {
    if let Some(last) = steps.last_mut() {
        last.rep = RegexRep::Range {
            min: Some(0),
            max: None,
        };
        Ok(None)
    } else {
        Err("Se encontró un caracter '*' insesperado")
    }
}

/// devuelve la regex step para el caracter '?'
fn evaluar_interrogacion(steps: &mut [RegexStep]) -> Result<Option<RegexStep>, &'static str> {
    if let Some(last) = steps.last_mut() {
        last.rep = RegexRep::Range {
            min: Some(0),
            max: Some(1),
        };
        Ok(None)
    } else {
        Err("Se encontró un caracter '?' insesperado")
    }
}

/// devuelve la regex step para el caracter '\'
fn evaluar_backslash(chars_iter: &mut std::str::Chars) -> Result<Option<RegexStep>, &'static str> {
    match chars_iter.next() {
        Some(literal) => Ok(Some(RegexStep {
            rep: RegexRep::Exact(1),
            val: RegexVal::Literal(literal),
        })),
        None => Err("Se encontró un caracter '\\' insesperado"),
    }
}

/// devuelve la regex step para el caracter '{'
fn evaluar_llaves(
    chars_iter: &mut std::str::Chars,
    steps: &mut [RegexStep],
) -> Result<Option<RegexStep>, &'static str> {
    let mut content = String::new();
    for c in chars_iter.by_ref() {
        if c == '}' {
            break;
        } else {
            content.push(c);
        }
    }
    let (min, max) = Regex::contenido_llaves(&content)?;
    if let (Some(min_val), Some(max_val)) = (min, max) {
        if min_val > max_val {
            return Err("El rango minimo no puede ser mayor al rango maximo");
        }
    }
    if let Some(last) = steps.last_mut() {
        last.rep = RegexRep::Range { min, max };
        Ok(None)
    } else {
        Err("Se encontró un caracter '{' insesperado")
    }
}

/// devuelve la regex step para el caracter '['
fn evaluar_corchete(
    chars_iter: &mut std::str::Chars,
    steps: &mut Vec<RegexStep>,
) -> Result<Option<RegexStep>, &'static str> {
    let mut chars = Vec::new();
    let mut last_char = None;
    let mut range = false;
    let mut clase = String::new();
    let mut es_clase = false;
    let mut es_corchete_doble = false;
    let mut es_negado = false;
    for c in chars_iter.by_ref() {
        match c {
            ']' => {
                // si encuentra un corchete cierra el loop
                if es_corchete_doble {
                    es_corchete_doble = false;
                } else {
                    break;
                }
            }
            '^' if chars.is_empty() && !es_clase => {
                es_negado = true; // si el primer carácter después de '[' es '^' entonces la clase sera negada
            }
            '[' => {
                es_corchete_doble = true;
            }
            '-' => {
                //si encuentra '-' es un rango
                range = true;
            }
            ':' => {
                //si encuentra ':' es una clase
                es_clase = !es_clase;
                if es_clase {
                    clase.clear();
                } else {
                    steps.push(RegexStep {
                        rep: RegexRep::Exact(1),
                        val: RegexVal::Clase(clase, es_negado),
                    });
                    clase = String::new(); //reasigno 'clase' a una nueva cadena de texto vacía
                }
            }
            _ if es_clase => {
                clase.push(c);
            }
            _ => {
                if range {
                    if let Some(last) = last_char {
                        for c in (last as u8 + 1)..=(c as u8) {
                            //itera desde el ultimo caracter hasta el actual
                            chars.push(c as char);
                        }
                    }
                    range = false;
                } else {
                    chars.push(c);
                }
                last_char = Some(c);
            }
        }
    }
    if !chars.is_empty() {
        Ok(Some(RegexStep {
            rep: RegexRep::Exact(1),
            val: RegexVal::Bracket(chars, es_negado),
        }))
    } else {
        Ok(None)
    }
}
