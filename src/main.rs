use std::fs::File;
use std::io::{Write};
use std::error::Error;
use std::{fmt, result};
use std::env;


mod celda;
use celda::Celda;
mod mapa;
use mapa::Mapa;
mod explosion;
use explosion::Explosion;
mod enemigo;
use enemigo::Enemigo;

enum Errores {

}
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

fn guardar_error_y_salir(mensaje: &str, file_path_destino: &str) -> Result<(), Box<dyn Error>> {
    let mut archivo = File::create(file_path_destino)?;
    let mensaje_formateado = format!("ERROR: [{}]", mensaje);
    archivo.write_all(mensaje_formateado.as_bytes())?;
    Ok(())
}
fn main() -> Result<(), Box<dyn Error>> {
    
    let args: Vec<String> = env::args().collect();
    let file_path_origen = format!("{}", args[1]);
    let file_path_destino: String = format!("{}{}", args[2], args[1]);
    let x = args[3].parse::<i32>().map_err(|err| format!("Error al parsear x a i32, {}", err))?;
    let y = args[4].parse::<i32>().map_err(|err| format!("Error al parsear y a i32, {}", err))?;
    // MANEJAR EL INPUT EN UNA FUNCION

    let result_mapa = Mapa::crear_mapa(&file_path_origen);
    let mut mapa = match result_mapa {
        Ok(mapa) => mapa,
        Err(err) => {
            let err_msj = format!("Error al crear el mapa, {}", err); 
            guardar_error_y_salir(&err_msj, &file_path_destino)?;
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Error al crear el mapa")));}
    };
    mapa.mostrar_mapa();
    let bomba = mapa.obtener_celda(y as usize, x as usize);
    match bomba {
        Celda::Bomba { representacion: _, alcance, de_traspaso } => {
            Explosion::new(*alcance as i32, *de_traspaso).iniciar_explosion(&mut mapa, y, x)
        },
        _ => {
            guardar_error_y_salir("No hay una bomba en la posicion elegida", &file_path_destino)?; 
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "No hay una bomba en la posicion elegida")));
        }
    }
    println!("-------------------------");
    mapa.mostrar_mapa();

    let _ = mapa.guardar_mapa(&file_path_destino);

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