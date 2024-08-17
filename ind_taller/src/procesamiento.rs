use std::fs::File;
use std::io::{self, BufRead};
use tp_ind::regex::Regex;

/// Procesa las líneas de un archivo y devuelve un vector de string con las que cumplen con la expresión regular.
pub fn procesar_lineas(reader: io::BufReader<File>, regex: &Regex) -> Vec<String> {
    //println!("Regex: {:?}", regex);
    let mut vec_ok = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(value) => match regex.test(&value) {
                Ok(result) => {
                    if result {
                        vec_ok.push(value);
                    }
                }
                Err(err) => eprintln!("Error: {}", err),
            },
            Err(e) => eprintln!("No se pudo leer la línea: {}", e),
        }
    }
    vec_ok
}

pub fn imprimir_resultados(vec_ok: Vec<String>) {
    for c in vec_ok {
        println!("{}", c);
    }
}
