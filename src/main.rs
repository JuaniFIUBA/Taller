use std::fs::File;
use std::io::Write;
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
    if let Err(x) = args[3].parse::<i32>().map_err(|err|format!("Error al parsear x, {}", err)) {}
    // let x = args[3].parse::<i32>().map_err(|err| format!("Error al parsear x a i32"))
    //     .unw;
    // let x = match args[3].parse::<i32>().map_err(|err| format!("Error al parsear x a i32, {}", err)) {
    //     Ok(x) => {x},
    //     Err(err) => {
    //         guardar_error_y_salir(&err, &file_path_destino)?;
    //         return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, err)));
    //     }
    // };
    let y = match args[4].parse::<i32>().map_err(|err| format!("Error al parsear y a i32, {}", err)) {
        Ok(x) => {x},
        Err(err) => {
            guardar_error_y_salir(&err, &file_path_destino)?;
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, err)));
        }
    };
    // MANEJAR EL INPUT EN UNA FUNCION

    let mut mapa = match Mapa::crear_mapa(&file_path_origen).map_err(|err| format!("Error al crear el mapa {}", err)) {
        Ok(mapa) => mapa,
        Err(err) => {
            guardar_error_y_salir(&err, &file_path_destino)?;
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, err)));}
    };
    mapa.mostrar_mapa();

    let celda_result = mapa.obtener_celda(y as usize, x as usize)
                                    .map_err(|err| format!("Error, no hay ninguna bomba en la posicion elegida. {}", err));
    let bomba = match celda_result {
        Ok(bomba) => bomba,
        Err(err) => {
            guardar_error_y_salir(&err, &file_path_destino)?;
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, err)));}
    };

    match bomba {
        Celda::Bomba { representacion: _, alcance, de_traspaso } => {
            match Explosion::new(*alcance as i32, *de_traspaso).iniciar_explosion(&mut mapa, y, x) {
                Ok(_) => {},
                Err(_) => {return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Error al crear la explosion")));}
            }
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