use tp_individual::explosion::Explosion;
use tp_individual::io;
use tp_individual::mapa::Mapa;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (file_path_origen, file_path_destino, x, y) = io::obtener_input()?;

    let mut mapa = match Mapa::crear_mapa(&file_path_origen)
        .map_err(|err| format!("Error al crear el mapa. {}", err))
    {
        Ok(mapa) => mapa,
        Err(err) => {
            io::guardar_error_y_salir(&err, &file_path_destino)?;
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                err,
            )));
        }
    };
    mapa.mostrar_mapa();
    Explosion::explotar(&file_path_destino, &mut mapa, x, y)?;
    mapa.guardar_mapa(&file_path_destino)?;
    Ok(())
}
