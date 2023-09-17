use std::fs::File;
use std::io::Write;
use std::error::Error;
use std::env;


mod celda;
use celda::Celda;
mod mapa;
use mapa::Mapa;
mod explosion;
use explosion::Explosion;
mod enemigo;
use enemigo::Enemigo;


// guarda el error mensaje de error en el archivo destino indicado por file_path_destino y retorna
fn guardar_error_y_salir(mensaje: &str, file_path_destino: &str) -> Result<(), Box<dyn Error>> {
    let mut archivo = File::create(file_path_destino)?;
    let mensaje_formateado = format!("ERROR: [{}]", mensaje);
    archivo.write_all(mensaje_formateado.as_bytes())?; // No tiene sentido tratar de escribir este error si el anterior no se pudo escribir
    Ok(())
}


// pasa una coordenada numerica en formato de &str a i32 
fn formatear_coordenada(argumento: &str, file_path_destino: &str) -> Result<i32, Box::<dyn Error>>{
    match argumento.parse::<i32>().map_err(|err| format!("Error al parsear {} a i32, {}", argumento, err)) {
        Ok(x) => Ok(x),
        Err(err) => {
            guardar_error_y_salir(&err, file_path_destino)?;
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, err)))
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "La forma de llamar al programa es: cargo run -- mapa.txt /path/to/output_dir/ x y")));
    }
    let file_path_origen = args[1].to_string();
    let file_path_destino: String = format!("{}{}", args[2], args[1]);
    let x = formatear_coordenada(&args[3], &file_path_destino)?;
    let y = formatear_coordenada(&args[4], &file_path_destino)?;
    let mut mapa = match Mapa::crear_mapa(&file_path_origen).map_err(|err| format!("Error al crear el mapa. {}", err)) {
        Ok(mapa) => mapa,
        Err(err) => {
            guardar_error_y_salir(&err, &file_path_destino)?;
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, err)));}
    };
    mapa.mostrar_mapa();      
    match mapa.obtener_celda(y as usize, x as usize).map_err(|err| format!("{}", err)) {
        Ok(bomba) => { //
            match bomba {
                Celda::Bomba { representacion: _, alcance, de_traspaso } => {
                    Explosion::new(*alcance as i32, *de_traspaso).iniciar_explosion(&mut mapa, y, x)?;},
                _ => {
                    guardar_error_y_salir("No hay una bomba en la posicion elegida", &file_path_destino)?; 
                    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "No hay una bomba en la posicion elegida")));
                }
            }
        
        },
        Err(err) => {
            guardar_error_y_salir(&err, &file_path_destino)?;
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, err)));}
    }
    mapa.guardar_mapa(&file_path_destino)?;
    Ok(())
}
