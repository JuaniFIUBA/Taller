use std::fs::File;
use std::io::{self, Write, BufRead};
use crate::Celda;
use crate::Enemigo;
pub struct Mapa {
    grilla: Vec<Vec<Celda>>,
}


impl Mapa {
    pub fn  crear_mapa(file_path: &str) -> Result<Mapa, Box<dyn std::error::Error>> {
        // Abre el archivo en modo lectura
        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);
        let mut filas: Vec<Vec<Celda>> = Vec::new();
        let mut cant_enemigos: u32 = 0;
        // Itera sobre las líneas del archivo
        for line in reader.lines() {
            let line = line?;
    
            let mut cols: Vec<Celda> = Vec::new();
            
            // Divide la línea en palabras usando espacios como separadores        
            let palabras: Vec<&str> = line.split_whitespace().collect();
            // Itera a través de las palabras e imprímelas
            
            for palabra in palabras {
                cols.push(crear_objeto(palabra, &mut cant_enemigos)?);
            }
            filas.push(cols);
        }
        Ok(Mapa{grilla: filas})
        // Ok(filas)
    }
    
    pub fn obtener_largo(&self) -> usize {
        self.grilla.len()
    }
    pub fn obtener_celda(&mut self, fila: usize, columna: usize) -> &mut Celda{
        &mut self.grilla[fila][columna]
    }

    pub fn mostrar_mapa(&self) {
        for fila in &self.grilla {
            for columna in fila {
                print!(" {}", columna.obtener_representacion());
            }
            println!("");
        }
    }
    pub fn borrar(&mut self, fila: usize, columna: usize) {
        self.grilla[fila][columna] = Celda::Vacio{ representacion: '_' };
    }

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

fn crear_objeto(palabra: &str, cant_enemigos: &mut u32) -> Result<Celda, Box<dyn std::error::Error>> {
    let obj = palabra.chars().nth(0).ok_or("La cadena esta vacia")?;
    match obj {
        '_' => {Ok(Celda::Vacio { representacion: '_' })},
        'B' | 'S' => {
            let alcance = palabra.chars().nth(1).ok_or("Formato de bomba inválido")?;
            let valor_alcance = alcance.to_digit(10).ok_or("Fallo al convertir el alcance a entero")?;
            match obj {
                'B' => {Ok(Celda::Bomba { representacion: obj, alcance: valor_alcance as usize, de_traspaso: false })}, 
                'S' => {Ok(Celda::Bomba { representacion: obj, alcance: valor_alcance as usize, de_traspaso: true })},
                _ => {Ok(Celda::Vacio{representacion: '_'})} // Aca deberia ir un error
            }
        }
        'R' | 'W' => {  
            Ok(Celda::Obstaculo { representacion: obj })
        },
        'F' => {
            let pv = palabra.chars().nth(1).ok_or("Puntos de vida inválidos")?;
            let valor_pv = pv.to_digit(10).ok_or("Fallo al convertir el alcance a entero")?;
            *cant_enemigos += 1;
            Ok(Celda::Enemigo { enemigo: Enemigo::new(obj, valor_pv as  usize, *cant_enemigos)})
        },
        'D' => {
            let dir = palabra.chars().nth(1).ok_or("Dirección inválida")?;
            Ok(Celda::Desvio { representacion: obj, direccion: dir })
        },
        _ => {Ok(Celda::Vacio{ representacion: obj })} // aca va error
    }
}