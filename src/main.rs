use std::fs::File;
use std::io::{self, BufRead};
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


// Como implementar un error
impl fmt::Display for ErrorTypeNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tipo de objeto no reconocido!")
    }
}
#[derive(Debug)]
struct ErrorTypeNotFound;
impl Error for ErrorTypeNotFound {
    fn description(&self) -> &str {
        "El objeto no fue reconocido"
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let file_path = "mapa.txt";
    // let args: Vec<String> = env::args().collect();
    let mut  mapa = Mapa::crear_mapa(file_path)?;
    mapa.mostrar_mapa();
    let bomba = mapa.obtener_celda(0, 0);
    match bomba {
        Celda::Bomba { representacion: _, alcance, de_traspaso } => {
            Explosion::new(*alcance as i32, *de_traspaso).iniciar_explosion(&mut mapa, 0, 0)
        },
        _ => {println!("Error")}
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