use std::env;
use std::fs::File;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use tp_ind::regex::Regex;

pub fn abrir_archivo(ruta: &Path) -> io::Result<File> {
    let file = File::open(ruta)?;
    Ok(file)
}

pub fn leer_entrada() -> Result<(Regex, PathBuf), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        return Err(String::from(
            "Uso incorrecto: egrep <expresion_regular> <ruta_del_archivo>",
        ));
    }

    let expresion_regular = &args[1];
    let ruta_archivo = &args[2];

    let regex = match Regex::new(expresion_regular) {
        Ok(r) => r,
        Err(_) => {
            return Err(String::from("Error al crear la expresión regular"));
        }
    };

    let ruta = PathBuf::from(ruta_archivo);

    Ok((regex, ruta))
}
