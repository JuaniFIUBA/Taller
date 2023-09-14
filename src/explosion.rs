use crate::Mapa;
use crate::Celda;
use crate::Enemigo;
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
    pub fn iniciar_explosion(&mut self, mapa: &mut Mapa ,fila: i32, columna: i32) {        
        mapa.borrar(fila as usize, columna as usize);
        println!("{}", self.alcance);
        self.explotar_al_sur(mapa, fila, columna, self.alcance);
        self.explotar_al_norte(mapa, fila, columna, self.alcance);
        self.explotar_al_este(mapa, fila, columna, self.alcance);
        self.explotar_al_oeste(mapa , fila, columna, self.alcance);
    }

    fn explotar_al_sur(&mut self, mapa: &mut Mapa, fila: i32, columna: i32, alcance: i32){
        let mut fila_actual = fila as usize + 1;
        let mut cont: i32 = 1;
        while fila_actual < mapa.obtener_largo() && cont <= alcance{
            if !self.actuar(mapa, fila_actual as usize, columna as usize, &mut cont) {
                break;
            }
            cont += 1;
            fila_actual += 1;
        } 
    }
    fn explotar_al_norte(&mut self, mapa: &mut Mapa, fila: i32, columna: i32, alcance: i32){
        let mut fila_actual = fila - 1;
        let mut cont: i32 = 1; // Empieza en 1 porque estoy en el siguiente casillero de la expl
        while fila_actual > 0 && cont  <= alcance{
            if !self.actuar(mapa, fila_actual as usize, columna as usize, &mut cont) {
                break;
            }
            cont += 1;
            fila_actual -= 1;
        }    
    }
    fn explotar_al_este(&mut self, mapa: &mut Mapa, fila: i32, columna: i32, alcance: i32){
        let mut columna_actual = columna as usize + 1;
        let mut cont: i32 = 1;
        while columna_actual < mapa.obtener_largo() && cont <= alcance {
            if !self.actuar(mapa, fila as usize, columna_actual as usize, &mut cont) {
                break;
            }
            cont += 1;
            columna_actual += 1;
        } 
    } 
    fn explotar_al_oeste(&mut self, mapa: &mut Mapa, fila: i32, columna: i32, alcance: i32){
        let mut columna_actual= columna - 1;
        let mut cont: i32= 1;
        while columna_actual > 0 && cont <= alcance{
            if !self.actuar(mapa, fila as usize, columna_actual as usize, &mut cont) {
                break;
            }
            cont += 1;
            columna_actual -= 1;
        } 
    }

    fn actuar(&mut self, mapa: &mut Mapa, fila: usize, columna: usize, cont: &mut i32) -> bool {
        let objeto = mapa.obtener_celda(fila, columna);
        match objeto {
            Celda::Vacio { representacion: _ } => {true},
            Celda::Bomba { representacion: _, alcance, de_traspaso } => {
                Explosion::new(*alcance as i32, *de_traspaso).iniciar_explosion(mapa, fila as i32, columna as i32);
                false
            }
            Celda::Obstaculo { representacion } => {
                match representacion {
                    'W' => {false},
                    'R' => {if self.de_traspaso {true} else{false}}
                    _ => todo!()
                }                
            }
            Celda::Enemigo { enemigo } => {
                if !self.enemigos_afectados.contains(enemigo){
                    enemigo.herir();
                    self.enemigos_afectados.push(*enemigo);
                    if !enemigo.esta_vivo() {mapa.borrar(fila, columna)}
                } 
                true},
            Celda::Desvio { representacion: _, direccion } => {
                println!("Contador: {}", cont);
                match direccion {
                    'U' => {self.explotar_al_norte(mapa, fila as i32, columna as i32, self.alcance - *cont);},
                    'D' => {self.explotar_al_sur(mapa, fila as i32, columna as i32, self.alcance - *cont);},
                    'R' => {self.explotar_al_este(mapa, fila as i32, columna as i32, self.alcance - *cont);},
                    'L' => {self.explotar_al_oeste(mapa, fila as i32, columna as i32, self.alcance - *cont);},
                    _ => todo!()
                }
                false
            }
        }
    }
}