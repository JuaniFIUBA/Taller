use std::fs::File;
use std::io::{self, Write, BufRead};
use super::celda::Celda;

/// Representa a un mapa que contiene celdas con objetos
pub struct Mapa {
    /// Grilla que representa el mapa
    grilla: Vec<Vec<Celda>>,
}



impl Mapa {    
    /// Dada una grilla crea una instanc}ia de Mapa
    pub fn new(grilla: Vec<Vec<Celda>>) -> Mapa {
        Mapa { grilla }
    }

    /// Instancia un mapa con sus elementos a partir de un archivo indicado en los argumentos
    /// 
    /// # Argumentos
    ///    
    /// * `file_path`: Referencia a str que contiene la direccion del mapa
    ///
    /// # Returns
    /// 
    /// Devuelve una instancia de Mapa o error

    pub fn  crear_mapa(file_path: &str) -> Result<Mapa, Box<dyn std::error::Error>> {
        // Abre el archivo en modo lectura
        let file = File::open(file_path).map_err(|err| format!("Error al abrir el archivo{}", err))?;
        let reader = io::BufReader::new(file);
        let mut filas: Vec<Vec<Celda>> = Vec::new();
        let mut cant_enemigos: u32 = 0;
        // Itera sobre las líneas del archivo
        for line in reader.lines() {
            let line = line?;
    
            let mut cols: Vec<Celda> = Vec::new();
            
            // Divide la línea en palabras usando espacios como separadores        
            let palabras: Vec<&str> = line.split_whitespace().collect();
            // Itera a través de las palabras y las pushea al el vector cols
            
            for palabra in palabras {
                cols.push(crear_objeto(palabra, &mut cant_enemigos)?);
            }
            filas.push(cols);
        }
        Ok(Mapa{grilla: filas})
    }

    /// Devuelve el largo del mapa, que al ser cuadrado coincide en x e y
    /// # Ejemplo
    /// Lo que sea referido como celda°n, hace referencia a una instancia de celda 
    /// ```
    /// use tp_individual::mapa::Mapa;
    /// use tp_individual::celda::Celda;
    /// let celda = Celda::Vacio{ representacion: '_' };
    /// let mapa = Mapa::new(vec![
    ///                         vec![celda.clone(), celda.clone()],
    ///                         vec![celda.clone(), celda.clone()]]);
    /// assert_eq!(mapa.obtener_largo(), 2);
    /// ```
    
    pub fn obtener_largo(&self) -> usize {
        self.grilla.len()
    }

    /// Obtiene una referencia mutable a una celda contenida en un mapa 
    /// 
    /// # Argumentos: 
    ///    
    /// * `&mut self`: Autorreferencia
    /// * `fila`: Valor de la fila que se quiere obtener (equivale a x)
    /// * `columna`: Valor de la columna que se quiere obtener (equivale a y)
    /// 
    /// # Returns 
    /// 
    /// Resultado que contiene una referencia mutable a la celda indicada, o error.
    ///
    /// # Ejemplo
    /// Lo que sea referido como celda°n, hace referencia a una instancia de celda 
    /// ```
    /// use tp_individual::mapa::Mapa;
    /// use tp_individual::celda::Celda;
    /// let mut celda = Celda::Vacio{ representacion: '_' };
    /// let mut mapa = Mapa::new(vec![
    ///                         vec![celda.clone(), celda.clone()],
    ///                         vec![celda.clone(), celda.clone()]]);
    /// assert_eq!(mapa.obtener_celda(0, 0).unwrap(), &celda);
    /// ```

    // Al ser usize, la fila y columna no pueden ser menores a 0
    pub fn obtener_celda(&mut self, fila: usize, columna: usize) -> Result<&mut Celda, Box<dyn std::error::Error>> {
        if  fila > self.grilla.len() || columna > self.grilla.len() {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "La celda elegida está fuera de rango")))
        } else {
            Ok(&mut self.grilla[fila][columna])
        }
    }

    /// Muestra el mapa en consola
    pub fn mostrar_mapa(&self) {
        for fila in &self.grilla {
            for columna in fila {
                print!(" {}", columna.obtener_representacion());
            }
            println!();
        }
    }

    /// Borra una celda en la posición indicada, para eso transforma lo contenido en la celda con una Celda::Vacio 
    /// 
    /// # Argumentos 
    /// 
    /// * `&mut self`: autorreferencia mutable 
    /// * `fila`: Valor de la fila que se quiere obtener (equivale a x)
    /// * `columna`: Valor de la columna que se quiere obtener (equivale a y)
    pub fn borrar(&mut self, fila: usize, columna: usize) {
        self.grilla[fila][columna] = Celda::Vacio{ representacion: '_' };
    }

    /// Guarda el mapa en la direccion indicada, si no existe el archivo lo crea
    /// 
    /// # Argumentos 
    /// * `&self`: autorreferencia  
    /// * `file_path`: referencia a un string que contiene la direccion de guardado 
    /// 
    /// # Returns 
    /// 
    /// Result de Ok o Error  

    pub fn guardar_mapa(&self, file_path: &str) -> io::Result<()>{
        let mut archivo = File::create(file_path)?;
        for fila in &self.grilla {
            for (i, objeto) in fila.iter().enumerate() {
                archivo.write_all(objeto.obtener_representacion().as_bytes())?;
    
                // Si no es el último string del vector, añade un espacio
                if i < fila.len() - 1 {
                    archivo.write_all(b" ")?;
                }
            }
            // Agrega una nueva línea después de cada vector interno
            archivo.write_all(b"\n")?;
        }
        Ok(())
    }
}

// Crea un tipo de dato perteneciente al enum Celda a partir de un string. 
// El argumento cant_enemigos se utiliza para asignarle un id en caso de que se quiera crear un enemigo.
// Esto se debe a que cuando se quiere comparar entre un enemigo y otro que por ejemplo tienen los mismos
// atributos, rust compara 1 a 1 esos atributos y devuelve true en caso de que coincidan, que no es lo
// que esta bien, ya que podrian tener los mismos puntos de vida y no ser el mismo enemigo
fn crear_objeto(palabra: &str, cant_enemigos: &mut u32) -> Result<Celda, Box<dyn std::error::Error>> {
    // let obj = palabra.chars().next().ok_or("La cadena esta vacia")?;
    let (repr, atrib) = palabra.split_at(1); 
    let obj = repr.chars().next().ok_or("Error al objener la representacion")?;
    match obj {
        '_' => {Ok(Celda::Vacio { representacion: '_' })},
        'B' => {Ok(Celda::bomba_normal(atrib.parse::<usize>()?))},
        'S' => {Ok(Celda::bomba_traspaso(atrib.parse::<usize>()?))},
        'R' => {Ok(Celda::roca())},
        'W' => {Ok(Celda::pared())},
        'F' => {
            let pv = atrib.parse::<usize>()?;
            *cant_enemigos += 1;
            Ok(Celda::enemigo(pv, *cant_enemigos))
        },
        'D' => {
            let dir = atrib.chars().next().ok_or("Dirección de desvío inválida")?;
            Ok(Celda::desvio(dir))
        },
        _ => {Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Objeto no reconocido")))} // aca va error
    }
}


#[cfg(test)] 
mod test {
    use super::*;
    #[test]
    fn test_obtener_celda() -> Result<(), Box<dyn std::error::Error>> {
        let mut mapa = Mapa{grilla: vec![vec![Celda::Vacio { representacion: '_' }]]};
        assert_eq!(mapa.obtener_celda(0, 0)?, &mut Celda::Vacio { representacion: '_' });
        Ok(())
    }
    
    #[test]
    fn test_borrar_celda() -> Result<(), Box<dyn std::error::Error>> {
        let mut mapa = Mapa{grilla: vec![vec![Celda::Obstaculo { representacion: 'R' }]]};
        mapa.borrar(0, 0);
        assert_eq!(mapa.obtener_celda(0, 0)?, &mut Celda::Vacio { representacion: '_' });        
        Ok(())
    }
}