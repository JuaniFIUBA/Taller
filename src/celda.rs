use crate::Enemigo;
#[derive(Debug, PartialEq)]
pub enum Celda {
    Vacio{representacion: char}, 
    Bomba{representacion: char, alcance: usize, de_traspaso: bool},
    Obstaculo {representacion: char},
    Enemigo{enemigo: Enemigo},
    Desvio{representacion: char, direccion: char}
}

impl Celda {
    pub fn obtener_representacion(&self) -> String {
        match self {
            Celda::Vacio { representacion } => {representacion.to_string()},
            Celda::Bomba { representacion, alcance, de_traspaso: _} => {representacion.to_string() + &alcance.to_string()}
            Celda::Obstaculo { representacion } => {representacion.to_string()}
            Celda::Enemigo { enemigo } => {enemigo.obtener_representacion()},
            Celda::Desvio { representacion, direccion } => {representacion.to_string() + &direccion.to_string()}
        }
    }
}

    