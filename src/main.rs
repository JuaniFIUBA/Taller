use std::fs::File;
use std::io::{self, BufRead, Write};
use std::error::Error;
use std::fmt;
use std::env;


mod celda;
use celda::Celda;
mod mapa;
use mapa::Mapa;
mod explosion;
use explosion::Explosion;
mod enemigo;
use enemigo::Enemigo;


#[derive(Debug)]
struct ErrorTypeNotFound {
    mensaje: String
}
impl ErrorTypeNotFound {
    fn new(mensaje: &str) -> ErrorTypeNotFound {
        ErrorTypeNotFound { mensaje: mensaje.to_string() }
    }
}
impl Error for ErrorTypeNotFound {
    fn description(&self) -> &str {
        &self.mensaje
    }
}
// Como implementar un error
impl fmt::Display for ErrorTypeNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mi error: {}", self.mensaje)
    }
}

// fn guardar_error_y_salir(mensaje_error: &str, nombre_archivo: &str) -> Result<(), Box<dyn Error>> {
//     let mut archivo = File::create(nombre_archivo)?;
//     let mensaje_formateado = format!("ERROR: [{}]", mensaje_error);
//     archivo.write_all(mensaje_formateado.as_bytes());
//     Ok(())
// // }
fn guardar_error_y_salir<T: Error + 'static>(mensaje: &str, nombre_archivo: &str, error: T) -> Result<(), Box<dyn Error>> {
    let mut archivo = File::create(nombre_archivo)?;
    let mensaje_formateado = format!("ERROR: [{}]", mensaje);
    archivo.write_all(mensaje_formateado.as_bytes())?;
    Err(Box::new(error))
}
fn main() -> Result<(), Box<dyn Error>> {
    
    let args: Vec<String> = env::args().collect();
    let file_path = format!("{}/{}", args[2], args[1]);


    let mut  mapa = Mapa::crear_mapa(&file_path)?;
    mapa.mostrar_mapa();
    let bomba = mapa.obtener_celda(0, 0);
    match bomba {
        Celda::Bomba { representacion: _, alcance, de_traspaso } => {
            Explosion::new(*alcance as i32, *de_traspaso).iniciar_explosion(&mut mapa, 0, 0)
        },
        _ => {
            return guardar_error_y_salir("Tipo de objeto invalido", &file_path,std::io::Error::new(std::io::ErrorKind::Other, "Tipo de objeto invalido"));
        }
    }
    println!("-------------------------");
    mapa.mostrar_mapa();

    let _ = mapa.guardar_mapa("mapa2.txt");

    Ok(())

}

// #[cfg(test)]
// mod tests {
//     // Note this useful idiom: importing names from outer (for mod tests) scope.
//     use super::*;

//     #[test]
//     fn test_add() {
//         assert_eq!(add(1, 2), 3);
//     }

//     #[test]
//     fn test_bad_add() {
//         // This assert would fire and test will fail.
//         // Please note, that private functions can be tested too!
//         assert_eq!(bad_add(1, 2), 3);
//     }
// }