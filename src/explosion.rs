use crate::Mapa;
use crate::Celda;
use crate::Enemigo;
use std::error::Error;
pub struct Explosion {
    alcance: i32,
    de_traspaso: bool,
    enemigos_afectados: Vec<Enemigo>
}

impl Explosion {
    pub fn new(alcance: i32, de_traspaso: bool) -> Explosion {
        Explosion {
            alcance,
            de_traspaso,
            enemigos_afectados: Vec::new()
        }
    }
    pub fn iniciar_explosion(&mut self, mapa: &mut Mapa ,fila: i32, columna: i32) -> Result<(), Box<dyn Error>> {        
        mapa.borrar(fila as usize, columna as usize);
        self.explotar_al_sur(mapa, fila, columna, self.alcance)?;
        self.explotar_al_norte(mapa, fila, columna, self.alcance)?;
        self.explotar_al_este(mapa, fila, columna, self.alcance)?;
        self.explotar_al_oeste(mapa , fila, columna, self.alcance)?;
        Ok(())
    }

    fn explotar_al_sur(&mut self, mapa: &mut Mapa, fila: i32, columna: i32, alcance: i32) -> Result<(), Box<dyn Error>>{
        let mut fila_actual = fila as usize + 1;
        let mut cont: i32 = 1;
        while fila_actual < mapa.obtener_largo() && cont <= alcance{
            if !self.actuar(mapa, fila_actual as usize, columna as usize, &mut cont)? {
                break;
            }
            cont += 1;
            fila_actual += 1;
        }
        Ok(()) 
    }
    fn explotar_al_norte(&mut self, mapa: &mut Mapa, fila: i32, columna: i32, alcance: i32) -> Result<(), Box<dyn Error>> {
        let mut fila_actual = fila - 1;
        let mut cont: i32 = 1; // Empieza en 1 porque estoy en el siguiente casillero de la expl
        while fila_actual > 0 && cont  <= alcance{
            if !self.actuar(mapa, fila_actual as usize, columna as usize, &mut cont)? {
                break;
            }
            cont += 1;
            fila_actual -= 1;
        }    
        Ok(()) 
    }
    fn explotar_al_este(&mut self, mapa: &mut Mapa, fila: i32, columna: i32, alcance: i32) -> Result<(), Box<dyn Error>> {
        let mut columna_actual = columna as usize + 1;
        let mut cont: i32 = 1;
        while columna_actual < mapa.obtener_largo() && cont <= alcance {
            if !self.actuar(mapa, fila as usize, columna_actual as usize, &mut cont)? {
                break;
            }
            cont += 1;
            columna_actual += 1;
        } 
        Ok(())
    } 
    fn explotar_al_oeste(&mut self, mapa: &mut Mapa, fila: i32, columna: i32, alcance: i32) -> Result<(), Box<dyn Error>>{
        let mut columna_actual= columna - 1;
        let mut cont: i32= 1;
        while columna_actual > 0 && cont <= alcance{
            if !self.actuar(mapa, fila as usize, columna_actual as usize, &mut cont)? {
                break;
            }
            cont += 1;
            columna_actual -= 1;
        } 
        Ok(())
    }

    fn actuar(&mut self, mapa: &mut Mapa, fila: usize, columna: usize, cont: &mut i32) -> Result<bool, Box<dyn std::error::Error>> {
        let objeto = mapa.obtener_celda(fila, columna).map_err(|err| format!("Error al obtener la celda {}", err))?;
        match objeto {
            Celda::Vacio { representacion: _ } => {Ok(true)},
            Celda::Bomba { representacion: _, alcance, de_traspaso } => {
                Explosion::new(*alcance as i32, *de_traspaso).iniciar_explosion(mapa, fila as i32, columna as i32)?;
                Ok(false)
            }
            Celda::Obstaculo { representacion } => {
                match representacion {
                    'W' => {Ok(false)},
                    'R' => {if self.de_traspaso {Ok(true)} else{Ok(false)}}
                    _ => {Ok(false)}
                }                
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
                    'U' => {self.explotar_al_norte(mapa, fila as i32, columna as i32, self.alcance - *cont)?;},
                    'D' => {self.explotar_al_sur(mapa, fila as i32, columna as i32, self.alcance - *cont)?;},
                    'R' => {self.explotar_al_este(mapa, fila as i32, columna as i32, self.alcance - *cont)?;},
                    'L' => {self.explotar_al_oeste(mapa, fila as i32, columna as i32, self.alcance - *cont)?;},
                    _ => {return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Error al obtener la direccion del desvio")));}
                }
                Ok(false)
            }
        }
    }
}