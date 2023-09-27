use crate::celda::TipoDeBomba;

use super::celda::Celda;
use super::io;
use super::mapa::Mapa;
use std::error::Error;

/// Simula una explosion

const ARRIBA: char = 'U';
const ABAJO: char = 'D';
const IZQUIERDA: char = 'L';
const DERECHA: char = 'R';

pub struct Explosion {
    /// Alcance de la explosión
    alcance: i32,

    /// Indicador de si la explosión fue generada por una bomba de traspaso
    de_traspaso: bool,

    /// Indica el sentido de la explosion en todo momento
    sentido: char,

    /// Posicion de Enemigos afectados por la explosión, se utiliza para no golpear dos veces al mismo enemigo con la misma explosión
    enemigos_afectados: Vec<(usize, usize)>,
}

impl Explosion {
    /// Crea una nueva instancia de Explosión, con alcance y característica de traspaso    
    /// La explosion comienza con sentido "hacia abajo".
    /// # Tabla de equivalencias:
    ///
    /// * `D`: sentido hacia abajo.
    /// * `U`: sentido hacia arriba.
    /// * `L`: sentido hacia la izquierda.
    /// * `R`: sentido hacia la derecha.
    ///    
    /// # Argumentos
    ///
    /// * `alcance`: representa el alcance que tendrá la explosión .
    /// * `de_traspaso`: indica si la explosión traspasa o no ciertos objetos .
    ///
    /// # Ejemplo
    ///
    /// ```
    /// use tp_individual::explosion::Explosion;
    ///
    /// let explosion = Explosion::new(3, true);
    /// ```
    pub fn new(alcance: i32, de_traspaso: bool) -> Explosion {
        Explosion {
            alcance,
            de_traspaso,
            sentido: ABAJO, // Empieza explotando hacia abajo
            enemigos_afectados: Vec::new(),
        }
    }

    /// Obtiene la celda indicada en el mapa indicado y luego, en caso de que haya una bomba, en base a sus atributos inicia una explosión
    ///
    /// # Argumentos
    ///
    /// * `file_path_destino`: directorio sobre el cual se creará el archivo y se escribirá el error en caso de que haya uno.
    /// * `mapa`: mapa sobre el cual se realizará la explosión.
    /// * `x`: columna sobre la cual se iniciara la explosión.
    /// * `y`: fila sobre la cual se iniciara la explosión .
    ///
    /// # Returns
    /// Result vacio o error
    ///
    /// # Ejemplo
    ///
    /// ``` ejemplo
    /// use tp_individual::explosion::Explosion;
    /// use tp_individual::mapa::Mapa;
    /// let mapa = Mapa::new(vec![vec!['B1']]);
    /// let file_path = "/home/usr/carpeta";
    ///
    /// Explosion::iniciar_explosion(&file_path, mapa, 0, 0)?;
    /// ```

    pub fn explotar(
        file_path_destino: &str,
        mapa: &mut Mapa,
        x: i32,
        y: i32,
    ) -> Result<(), Box<dyn Error>> {
        match mapa
            .obtener_celda(y as usize, x as usize)
            .map_err(|err| format!("{}", err))
        {
            Ok(bomba) => {
                //
                match bomba {
                    Celda::Bomba {
                        representacion: _,
                        alcance,
                        tipo_bomba,
                    } => {
                        if *tipo_bomba == TipoDeBomba::BombaDeTraspaso {
                            Explosion::new(*alcance as i32, true)
                            .iniciar_explosion(mapa, y, x)?;    
                        } else {
                            Explosion::new(*alcance as i32, false)
                            .iniciar_explosion(mapa, y, x)?;

                        }
                    }
                    _ => {
                        io::guardar_error_y_salir(
                            "No hay una bomba en la posicion elegida",
                            file_path_destino,
                        )?;
                        return Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "No hay una bomba en la posicion elegida",
                        )));
                    }
                }
            }
            Err(err) => {
                io::guardar_error_y_salir(&err, file_path_destino)?;
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    err,
                )));
            }
        }
        Ok(())
    }

    #[cfg(test)]
    pub fn explotar_test(mapa: &mut Mapa, x: i32, y: i32) -> Result<(), Box<dyn Error>> {
        match mapa
            .obtener_celda(y as usize, x as usize)
            .map_err(|err| format!("{}", err))
        {
            Ok(bomba) => {
                //
                match bomba {
                    Celda::Bomba {
                        representacion: _,
                        alcance,
                        tipo_bomba,
                    } => {
                        if *tipo_bomba == TipoDeBomba::BombaDeTraspaso {
                            Explosion::new(*alcance as i32, true)
                            .iniciar_explosion(mapa, y, x)?;    
                        } else {
                            Explosion::new(*alcance as i32, false)
                            .iniciar_explosion(mapa, y, x)?;

                        }
                    }
                    _ => {
                        return Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "No hay una bomba en la posicion elegida",
                        )));
                    }
                }
            }
            Err(err) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    err,
                )));
            }
        }
        Ok(())
    }

    // Precondicion: en el lugar hay una bomba
    // borra la celda para que en caso de que se detone otra bomba, la explosion no la haga volver a explotar y generar recursion infinita.
    // Luego llama a 4 funciones que iteran hacia las 4 direcciones posibles, detonando las celdas correspondientes al alcance.
    fn iniciar_explosion(
        &mut self,
        mapa: &mut Mapa,
        fila: i32,
        columna: i32,
    ) -> Result<(), Box<dyn Error>> {
        mapa.borrar(fila as usize, columna as usize);
        self.explotar_abajo(mapa, fila, columna, &mut 1)?;
        self.explotar_arriba(mapa, fila, columna, &mut 1)?;
        self.explotar_derecha(mapa, fila, columna, &mut 1)?;
        self.explotar_izquierda(mapa, fila, columna, &mut 1)?;
        Ok(())
    }

    // Itera hacia abajo del mapa y "explota" las celdas encontradas, idem para las explposiones hacia otras direcciones.
    fn explotar_abajo(
        &mut self,
        mapa: &mut Mapa,
        fila: i32,
        columna: i32,
        iteracion: &mut i32,
    ) -> Result<(), Box<dyn Error>> {
        self.sentido = ABAJO;
        let mut fila_actual = fila as usize + 1;
        while fila_actual < mapa.obtener_largo() && *iteracion <= self.alcance {
            if !self.explotar_celda(mapa, fila_actual, columna as usize, iteracion)? {
                break;
            }
            *iteracion += 1;
            fila_actual += 1;
        }
        Ok(())
    }

    fn explotar_arriba(
        &mut self,
        mapa: &mut Mapa,
        fila: i32,
        columna: i32,
        iteracion: &mut i32,
    ) -> Result<(), Box<dyn Error>> {
        self.sentido = ARRIBA;
        let mut fila_actual = fila - 1;
        while fila_actual >= 0 && *iteracion <= self.alcance {
            if !self.explotar_celda(mapa, fila_actual as usize, columna as usize, iteracion)? {
                break;
            }
            *iteracion += 1;
            fila_actual -= 1;
        }
        Ok(())
    }

    fn explotar_derecha(
        &mut self,
        mapa: &mut Mapa,
        fila: i32,
        columna: i32,
        iteracion: &mut i32,
    ) -> Result<(), Box<dyn Error>> {
        self.sentido = DERECHA;
        let mut columna_actual = columna as usize + 1;
        while columna_actual < mapa.obtener_largo() && *iteracion <= self.alcance {
            if !self.explotar_celda(mapa, fila as usize, columna_actual, iteracion)? {
                break;
            }
            *iteracion += 1;
            columna_actual += 1;
        }
        Ok(())
    }

    fn explotar_izquierda(
        &mut self,
        mapa: &mut Mapa,
        fila: i32,
        columna: i32,
        iteracion: &mut i32,
    ) -> Result<(), Box<dyn Error>> {
        self.sentido = IZQUIERDA;
        let mut columna_actual = columna - 1;
        while columna_actual >= 0 && *iteracion <= self.alcance {
            if !self.explotar_celda(mapa, fila as usize, columna_actual as usize, iteracion)? {
                break;
            }
            *iteracion += 1;
            columna_actual -= 1;
        }
        Ok(())
    }

    // cont se utiliza para indicar cuantas iteraciones realizó para una determinada dirección
    // el valor de retorno bool responde la pregunta de si la explosion deberia seguir o no
    fn explotar_celda(
        &mut self,
        mapa: &mut Mapa,
        fila: usize,
        columna: usize,
        cont: &mut i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let objeto = mapa
            .obtener_celda(fila, columna)
            .map_err(|err| format!("Error al obtener la celda {}", err))?;
        match objeto {
            Celda::Vacio { representacion: _ } => Ok(true),
            Celda::Bomba {
                representacion: _,
                alcance,
                tipo_bomba,
            } => {
                if *tipo_bomba == TipoDeBomba::BombaDeTraspaso {
                    Explosion::new(*alcance as i32, true).iniciar_explosion(
                        mapa,
                        fila as i32,
                        columna as i32,
                    )?;
                } else {
                    Explosion::new(*alcance as i32, false).iniciar_explosion(
                        mapa,
                        fila as i32,
                        columna as i32,
                    )?;

                }
                Ok(true)
            }
            Celda::Obstaculo { representacion } => {
                if *representacion == 'W' {
                    Ok(false)
                } else if self.de_traspaso && *representacion == 'R' {
                    Ok(true)
                } else {
                    Ok(false)
                } // check en caso de que traspase y se trate de una roca
            }
            Celda::Enemigo { enemigo } => {
                if !self.enemigos_afectados.contains(&(fila, columna)) {
                    enemigo.herir();
                    self.enemigos_afectados.push((fila, columna));
                    if !enemigo.esta_vivo() {
                        mapa.borrar(fila, columna)  
                    }
                }
                Ok(true)
            }
            Celda::Desvio {
                representacion: _,
                direccion,
            } => {
                let dir_actual = self.sentido;
                let copia_direccion_desvio = *direccion;
                match *direccion {
                    // desref y ref no mutable para comparar
                    ARRIBA => {
                        *cont += 1;
                        self.explotar_arriba(
                            mapa,
                            fila as i32,
                            columna as i32,
                            cont,
                        )?;
                    }
                    ABAJO => {
                        *cont += 1;
                        self.explotar_abajo(
                            mapa,
                            fila as i32,
                            columna as i32,
                            cont,
                        )?;
                    }
                    DERECHA => {
                        *cont += 1;
                        self.explotar_derecha(
                            mapa,
                            fila as i32,
                            columna as i32,
                            cont,
                        )?;
                    }
                    IZQUIERDA => {
                        *cont += 1;
                        self.explotar_izquierda(
                            mapa,
                            fila as i32,
                            columna as i32,
                            cont,
                        )?;
                    }
                    _ => {
                        return Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "Error al obtener la direccion del desvio",
                        )));
                    }
                }
                if dir_actual == copia_direccion_desvio {
                    Ok(true)
                } else {
                    Ok(false)
                }
                // si la direccion antes del desvio es igual a la direccion del desvio, debera seguir con la explosion
            }
        }
    }
}

#[cfg(test)]

mod test {
    // use super::*;
    use crate::celda::Celda;
    use crate::explosion::Explosion;
    use crate::mapa::Mapa;
    fn mapa_3_x_3() -> Mapa {
        Mapa::new(vec![
            vec![Celda::vacio(), Celda::enemigo(1), Celda::vacio()],
            vec![
                Celda::enemigo(1),
                Celda::bomba_normal(1),
                Celda::enemigo(1),
            ],
            vec![Celda::vacio(), Celda::enemigo(1), Celda::vacio()],
        ])
    }
    fn mapa_3_x_3_con_obstaculos() -> Mapa {
        Mapa::new(vec![
            vec![
                Celda::bomba_traspaso(3),
                Celda::roca(),
                Celda::enemigo(1),
            ],
            vec![
                Celda::bomba_traspaso(3),
                Celda::pared(),
                Celda::enemigo(1),
            ],
            vec![Celda::vacio(), Celda::vacio(), Celda::vacio()],
        ])
    }
    fn mapa_3_x_3_con_desvios() -> Mapa {
        Mapa::new(vec![
            vec![
                Celda::bomba_traspaso(3),
                Celda::enemigo(2),
                Celda::desvio('L'),
            ],
            vec![Celda::vacio(), Celda::vacio(), Celda::vacio()],
            vec![Celda::vacio(), Celda::vacio(), Celda::vacio()],
        ])
    }
    #[test]
    fn test_explotar_arriba() {
        let mut mapa: Mapa = mapa_3_x_3();
        let mut expl = Explosion::new(1, false);
        expl.explotar_arriba(&mut mapa, 1, 1, &mut 1).unwrap();
        assert_eq!(
            mapa.obtener_celda(0, 1).unwrap(),
            &mut Celda::Vacio {
                representacion: '_'
            }
        );
    }
    #[test]
    fn test_explotar_abajo() {
        let mut mapa: Mapa = mapa_3_x_3();
        let mut expl = Explosion::new(1, false);
        expl.explotar_abajo(&mut mapa, 1, 1, &mut 1).unwrap();
        assert_eq!(
            mapa.obtener_celda(2, 1).unwrap(),
            &mut Celda::Vacio {
                representacion: '_'
            }
        );
    }
    #[test]
    fn test_explotar_izquierda() {
        let mut mapa: Mapa = mapa_3_x_3();
        let mut expl = Explosion::new(1, false);
        expl.explotar_izquierda(&mut mapa, 1, 1, &mut 1).unwrap();
        assert_eq!(
            mapa.obtener_celda(1, 0).unwrap(),
            &mut Celda::Vacio {
                representacion: '_'
            }
        );
    }
    #[test]
    fn test_explotar_derecha() {
        let mut mapa: Mapa = mapa_3_x_3();
        let mut expl = Explosion::new(1, false);
        expl.explotar_derecha(&mut mapa, 1, 1, &mut 1).unwrap();
        assert_eq!(
            mapa.obtener_celda(1, 2).unwrap(),
            &mut Celda::Vacio {
                representacion: '_'
            }
        );
    }
    #[test]
    fn test_explosion_de_traspaso_traspasa_rocas() {
        let mut mapa = mapa_3_x_3_con_obstaculos(); // tiene bomba, roca, enemigo en la primer fila
        let mut expl = Explosion::new(3, true);
        expl.explotar_derecha(&mut mapa, 0, 0, &mut 1).unwrap();
        assert_eq!(
            mapa.obtener_celda(0, 2).unwrap(),
            &mut Celda::Vacio {
                representacion: '_'
            }
        );
    }
    #[test]
    fn test_misma_explosion_no_golpea_dos_veces_al_enemigo() {
        let mut mapa = mapa_3_x_3_con_desvios();
        let mut expl = Explosion::new(3, false);
        expl.explotar_derecha(&mut mapa, 0, 0, &mut 3).unwrap();
        let enemigo = mapa.obtener_celda(0, 1).unwrap();
        match enemigo {
            Celda::Enemigo { enemigo } => assert!(enemigo.esta_vivo()),
            _ => {
                assert!(false)
            }
        }
    }
}
