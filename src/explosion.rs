use crate::Mapa;
use crate::Celda;
use crate::Enemigo;
use std::error::Error;
use super::*;
/// Simula una explosion 

pub struct Explosion {
    /// Alcance de la explosión
    alcance: i32,
    
    /// Indicador de si la explosión fue generada por una bomba de traspaso
    de_traspaso: bool,

    /// Enemigos afectados por la explosión, se utiliza para no golpear dos veces al mismo enemigo con la misma explosión
    enemigos_afectados: Vec<Enemigo>
}

impl Explosion {
    /// Crea una nueva instancia de Explosión, con alcance y característica de traspaso    
    /// 
    /// # Argumentos
    ///
    /// * `alcance`: representa el alcance que tendrá la explosión .
    /// * `de_traspaso`: indica si la explosión traspasa o no ciertos objetos .
    ///
    /// # Ejemplo
    ///
    /// ```
    /// use explosion::Explosion;
    ///
    /// let explosion = Explosion::new(3, true);
    /// ```
    pub fn new(alcance: i32, de_traspaso: bool) -> Explosion {
        Explosion {
            alcance,
            de_traspaso,
            enemigos_afectados: Vec::new()
        }
    }

    /// Simula la explosión de una bomba en el lugar indicado, borra la posición inicial ya que 
    /// es una bomba, luego "expande" la explosión hacia sus costados
    /// El caso en el que no es una bomba está contemplado fuera de la explosión, entonces una precondición será
    /// que el lugar indicado sea una bomba efectivamente.
    /// 
    /// # Argumentos
    ///
    /// * `mapa`: mapa sobre el cual se realizará la explosión .
    /// * `fila`: fila sobre la cual se iniciara la explosión .
    /// * `columna`: columna sobre la cual se iniciara la explosión .
    ///
    /// # Returns
    /// Result vacio o error
    /// 
    /// # Ejemplo
    ///
    /// ``` ejemplo
    /// use explosion::Explosion;
    /// use mapa::Mapa;
    /// let mapa = Mapa::new(vec![vec!['B1']]);
    /// 
    /// Explosion::new(3, true).iniciar_explosion(mapa, 0, 0);
    /// ```

    pub fn explotar(file_path_destino: &str, mapa: &mut Mapa, x: i32, y: i32) -> Result<(), Box<dyn Error>> { 
        match mapa.obtener_celda(y as usize, x as usize).map_err(|err| format!("{}", err)) {
            Ok(bomba) => { //
                match bomba {
                    Celda::Bomba { representacion: _, alcance, de_traspaso } => {
                        Explosion::new(*alcance as i32, *de_traspaso).iniciar_explosion(mapa, y, x)?;},
                    _ => {
                        guardar_error_y_salir("No hay una bomba en la posicion elegida", file_path_destino)?; 
                        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "No hay una bomba en la posicion elegida")));
                    }
                }
            
            },
            Err(err) => {
                guardar_error_y_salir(&err, file_path_destino)?;
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, err)));}
        }
        Ok(())
    }

    fn iniciar_explosion(&mut self, mapa: &mut Mapa ,fila: i32, columna: i32) -> Result<(), Box<dyn Error>> {        
        mapa.borrar(fila as usize, columna as usize);
        self.explotar_abajo(mapa, fila, columna, self.alcance)?;
        self.explotar_arriba(mapa, fila, columna, self.alcance)?;
        self.explotar_derecha(mapa, fila, columna, self.alcance)?;
        self.explotar_izquierda(mapa , fila, columna, self.alcance)?;
        Ok(())
    }

    // Itera hacia abajo del mapa y "explota" las celdas encontradas, lo mismo para las explposiones 
    // hacia otros lados
    fn explotar_abajo(&mut self, mapa: &mut Mapa, fila: i32, columna: i32, alcance: i32) -> Result<(), Box<dyn Error>>{
        let mut fila_actual = fila as usize + 1;
        let mut cont: i32 = 1;
        while fila_actual < mapa.obtener_largo() && cont <= alcance{
            if !self.explotar_celda(mapa, fila_actual, columna as usize, &mut cont)? {
                break;
            }
            cont += 1;
            fila_actual += 1;
        }
        Ok(()) 
    }
    
    fn explotar_arriba(&mut self, mapa: &mut Mapa, fila: i32, columna: i32, alcance: i32) -> Result<(), Box<dyn Error>> {
        let mut fila_actual = fila - 1;
        let mut cont: i32 = 1; // Empieza en 1 porque estoy en el siguiente casillero de la expl
        while fila_actual >= 0 && cont  <= alcance{
            if !self.explotar_celda(mapa, fila_actual as usize, columna as usize, &mut cont)? {
                break;
            }
            cont += 1;
            fila_actual -= 1;
        }    
        Ok(()) 
    }
    
    fn explotar_derecha(&mut self, mapa: &mut Mapa, fila: i32, columna: i32, alcance: i32) -> Result<(), Box<dyn Error>> {
        let mut columna_actual = columna as usize + 1;
        let mut cont: i32 = 1;
        while columna_actual < mapa.obtener_largo() && cont <= alcance {
            if !self.explotar_celda(mapa, fila as usize, columna_actual, &mut cont)? {
                break;
            }
            cont += 1;
            columna_actual += 1;
        } 
        Ok(())
    } 
    
    fn explotar_izquierda(&mut self, mapa: &mut Mapa, fila: i32, columna: i32, alcance: i32) -> Result<(), Box<dyn Error>>{
        let mut columna_actual= columna - 1;
        let mut cont: i32= 1;
        while columna_actual >= 0 && cont <= alcance{
            if !self.explotar_celda(mapa, fila as usize, columna_actual as usize, &mut cont)? {
                break;
            }
            cont += 1;
            columna_actual -= 1;
        } 
        Ok(())
    }
    
    // cont se utiliza para indicar cuantas iteraciones realizó para una determinada dirección 
    // el valor de retorno bool responde la pregunta de si la explosion deberia seguir o no
    fn explotar_celda(&mut self, mapa: &mut Mapa, fila: usize, columna: usize, cont: &mut i32) -> Result<bool, Box<dyn std::error::Error>> {
        let objeto = mapa.obtener_celda(fila, columna).map_err(|err| format!("Error al obtener la celda {}", err))?;
        match objeto {
            Celda::Vacio { representacion: _ } => {Ok(true)},
            Celda::Bomba { representacion: _, alcance, de_traspaso } => {
                Explosion::new(*alcance as i32, *de_traspaso).iniciar_explosion(mapa, fila as i32, columna as i32)?;
                Ok(false)
            }
            Celda::Obstaculo { representacion } => {
                if *representacion == 'W' {
                    Ok(false)
                } else {if self.de_traspaso {Ok(true)} else {Ok(false)}}             
            }
            Celda::Enemigo { enemigo } => {
                if !self.enemigos_afectados.contains(enemigo){
                    enemigo.herir();
                    self.enemigos_afectados.push(*enemigo);
                    if !enemigo.esta_vivo() {mapa.borrar(fila, columna)}
                } 
                Ok(true)},
            Celda::Desvio { representacion: _, direccion } => {
                match direccion {
                    'U' => {self.explotar_arriba(mapa, fila as i32, columna as i32, self.alcance - *cont)?;},
                    'D' => {self.explotar_abajo(mapa, fila as i32, columna as i32, self.alcance - *cont)?;},
                    'R' => {self.explotar_derecha(mapa, fila as i32, columna as i32, self.alcance - *cont)?;},
                    'L' => {self.explotar_izquierda(mapa, fila as i32, columna as i32, self.alcance - *cont)?;},
                    _ => {return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Error al obtener la direccion del desvio")));}
                }
                Ok(false)
            }
        }
    }
}


// #[cfg(test)]

// mod test {
//     use super::*;

//     fn mapa_3_x_3 () -> Mapa {
//         let enemigo = Celda::Enemigo { enemigo: Enemigo::new('F', 1, 0)  };
//         let vacio = Celda::Vacio { representacion: '_' };
//         let bomba = Celda::Bomba { representacion: 'B', alcance: 1, de_traspaso: false };

//         Mapa::new(vec![
//             vec![vacio.clone(), enemigo.clone(), vacio.clone()],
//             vec![enemigo.clone(), bomba.clone(), enemigo.clone()],
//             vec![vacio.clone(), enemigo.clone(), vacio.clone()]
//             ])
//     }
//     fn mapa_3_x_3_con_obstaculos() -> Mapa {
//         let enemigo = Celda::Enemigo { enemigo: Enemigo::new('F', 1, 0)  };
//         let vacio = Celda::Vacio { representacion: '_' };
//         let bomba = Celda::Bomba { representacion: 'S', alcance: 3, de_traspaso: false };
//         let roca = Celda::Obstaculo { representacion: 'R' };
//         let pared = Celda::Obstaculo { representacion: 'W' };

//         Mapa::new(vec![
//             vec![bomba.clone(), roca.clone(), enemigo.clone()],
//             vec![bomba.clone(), pared.clone(), enemigo.clone()],
//             vec![vacio.clone(), vacio.clone(), vacio.clone()]
//             ])
//     }
//     fn mapa_3_x_3_con_desvios() -> Mapa {
//         let enemigo = Celda::Enemigo { enemigo: Enemigo::new('F', 2, 0)  };
//         let vacio = Celda::Vacio { representacion: '_' };
//         let bomba = Celda::Bomba { representacion: 'S', alcance: 3, de_traspaso: false };
//         let desvio_izquierda = Celda::Desvio { representacion: 'D', direccion: 'L' };

//         Mapa::new(vec![
//             vec![bomba.clone(), enemigo.clone(), desvio_izquierda.clone()],
//             vec![vacio.clone(), vacio.clone(), vacio.clone()],
//             vec![vacio.clone(), vacio.clone(), vacio.clone()]
//             ])
//     }
//     #[test]
//     fn test_explotar_arriba() -> Result<(), Box<dyn Error>> {
//         let mut mapa: Mapa = mapa_3_x_3();
//         let mut expl = Explosion::new(1, false);
//         expl.explotar_arriba(&mut mapa, 1, 1, 1)?;
//         assert_eq!(mapa.obtener_celda(0, 1)?, &mut Celda::Vacio { representacion: '_' });
//         Ok(())
//     }
//     #[test]
//     fn test_explotar_abajo() -> Result<(), Box<dyn Error>> {
//         let mut mapa: Mapa = mapa_3_x_3();
//         let mut expl = Explosion::new(1, false);
//         expl.explotar_abajo(&mut mapa, 1, 1, 1)?;
//         assert_eq!(mapa.obtener_celda(2, 1)?, &mut Celda::Vacio { representacion: '_' });
//         Ok(())
//     }
//     #[test]
//     fn test_explotar_izquierda() -> Result<(), Box<dyn Error>> {
//         let mut mapa: Mapa = mapa_3_x_3();
//         let mut expl = Explosion::new(1, false);
//         expl.explotar_izquierda(&mut mapa, 1, 1, 1)?;
//         assert_eq!(mapa.obtener_celda(1, 0)?, &mut Celda::Vacio { representacion: '_' });
//         Ok(())
//     }
//     #[test]
//     fn test_explotar_derecha() -> Result<(), Box<dyn Error>> {
//         let mut mapa: Mapa = mapa_3_x_3();
//         let mut expl = Explosion::new(1, false);
//         expl.explotar_derecha(&mut mapa, 1, 1, 1)?;
//         assert_eq!(mapa.obtener_celda(1, 2)?, &mut Celda::Vacio { representacion: '_' });
//         Ok(())
//     }
//     #[test]
//     fn  test_explosion_de_traspaso_traspasa_rocas() -> Result<(), Box<dyn Error>> {
//         let mut mapa = mapa_3_x_3_con_obstaculos();
//         let mut expl = Explosion::new(3, true);
//         expl.iniciar_explosion(&mut mapa, 0, 0)?;
//         assert_eq!(mapa.obtener_celda(0, 2)?, &mut Celda::Vacio { representacion: '_' });
//         assert_eq!(mapa.obtener_celda(1, 2)?, &mut Celda::Enemigo { enemigo: Enemigo::new('F', 1, 0) });        
//         Ok(())
//     }
//     #[test]
//     fn test_misma_explosion_no_golpea_dos_veces_al_enemigo() -> Result<(), Box<dyn Error>> {
//         let mut mapa = mapa_3_x_3_con_desvios();
//         let mut expl = Explosion::new(3, false);
//         expl.iniciar_explosion(&mut mapa, 0, 0)?;
//         let enemigo = mapa.obtener_celda(0, 1)?;
//         match enemigo {
//             Celda::Enemigo { enemigo } => assert!(enemigo.esta_vivo()),
//             _ => {assert!(false)} 
//         }
//         Ok(())
//     }

// }