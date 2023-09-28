use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;


/// Obtiene de los argumentos y devuelve los file paths destino y origen, junto con las coordenadas, si la cantidad de argumentos es
/// menor a 5, retorna un error. En caso de error al convertir las coordenadas, escribe en el directorio indicado por
///  file_path_origen, en el caso de que no se pueda, lanza un error

pub fn obtener_input() -> Result<(String, String, i32, i32), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "La forma de llamar al programa es: cargo run -- mapa.txt /path/to/output_dir/ x y",
        )));
    }
    let file_path_origen = args[1].to_string();
    let file_name = obtener_file_name(&args[1])?;
    
    let file_path_destino: String = format!("{}{}", args[2], file_name);
    let x = formatear_coordenada(&args[3], &file_path_destino)?;
    let y = formatear_coordenada(&args[4], &file_path_destino)?;

    Ok((file_path_origen, file_path_destino, x, y))
}

// Funcion para extraer el nombre de un file_path
fn obtener_file_name(file_path: &str) -> Result<String, Box<dyn Error>>{
    let file_path_origen = Path::new(file_path);
    if let Some(nombre_archivo) = file_path_origen.file_name() {
        if let Some(file_name_str) = nombre_archivo.to_str() {
            Ok(file_name_str.to_string())
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error al pasar el nombre del archivo a string",
            )))
        }
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Error al obtener el nombre del archivo",
        )))
    }

}

/// Guarda un mensaje de error en un archivo cuyo nombre coincide con el archivo input, pero puede ser escrito en otro directorio
/// indicado en file_path_destino
///
/// # Argumentos
///* `mensaje`: mensaje de error.
///* `file_path_destino`: directorio en el cual se guardara el mensaje de error.
///
/// # Returns
/// Error en caso de que no se haya podido escribir.
///
/// No se vuelve a llamar a la funcion guardar_error_y_salir en caso de que no se haya podido escribir para evitar una posible
/// recursion infinita.
pub fn guardar_error_y_salir(mensaje: &str, file_path_destino: &str) -> Result<(), Box<dyn Error>> {
    let mut archivo = File::create(file_path_destino)?;
    let mensaje_formateado = format!("ERROR: [{}]", mensaje);
    archivo.write_all(mensaje_formateado.as_bytes())?; // No tiene sentido tratar de escribir este error si el anterior no se pudo escribir
    Ok(())
}

// pasa una coordenada numerica en formato de &str a i32
fn formatear_coordenada(argumento: &str, file_path_destino: &str) -> Result<i32, Box<dyn Error>> {
    match argumento
        .parse::<i32>()
        .map_err(|err| format!("Error al parsear {} a i32, {}", argumento, err))
    {
        Ok(x) => Ok(x),
        Err(err) => {
            guardar_error_y_salir(&err, file_path_destino)?;
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                err,
            )))
        }
    }
}
