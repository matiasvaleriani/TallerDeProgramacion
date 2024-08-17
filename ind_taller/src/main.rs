mod entrada;
mod procesamiento;
use entrada::{abrir_archivo, leer_entrada};
use procesamiento::{imprimir_resultados, procesar_lineas};
use std::io;

fn main() {
    let (regex, path) = match leer_entrada() {
        Ok((r, p)) => (r, p),
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    let file = match abrir_archivo(&path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}: {}", path.display(), e);
            return;
        }
    };
    let reader = io::BufReader::new(file);
    let resultados = procesar_lineas(reader, &regex);
    imprimir_resultados(resultados);
}
