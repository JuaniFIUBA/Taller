use super::enemigo::Enemigo;
/// Enum que representa los tipos de celdas/casilleros en el juego Bomberman R.
#[derive(Debug, PartialEq, Clone)]
pub enum Celda {
    /// Celda vacia con una representación de un carácter.
    Vacio { representacion: char },

    /// Celda de bomba con una representación de un carácter, alcance y propiedad de traspaso.
    Bomba {
        representacion: char,
        alcance: usize,
        de_traspaso: bool,
    },

    /// Celda de obstáculo con una representación de un carácter.
    Obstaculo { representacion: char },

    /// Celda de enemigo que contiene una instancia de Enemigo.
    Enemigo { enemigo: Enemigo },

    /// Celda de desvío con una representación de carácter y dirección.
    Desvio {
        representacion: char,
        direccion: char,
    },
}

impl Celda {
    pub fn vacio() -> Celda {
        Celda::Vacio { representacion: '_' }
    }
    pub fn pared() -> Celda { 
        Celda::Obstaculo { representacion: 'W' }
    }
    pub fn roca() -> Celda { 
        Celda::Obstaculo { representacion: 'R' }
    }
    pub fn bomba_normal(alcance: usize) -> Celda { 
        Celda::Bomba { representacion: 'B', alcance, de_traspaso: false }
    }
    pub fn bomba_traspaso(alcance: usize) -> Celda {
        Celda::Bomba { representacion: 'B' , alcance, de_traspaso: true }
    }
    pub fn enemigo(pv: usize, id: u32) -> Celda {
        Celda::Enemigo { enemigo: Enemigo::new('F', pv, id) }
    }
    pub fn desvio(direccion: char) -> Celda {
        Celda::Desvio { representacion: 'D' , direccion}
    }
    /// Obtiene la representación de la celda como un string.
    pub fn obtener_representacion(&self) -> String {
        match self {
            Celda::Vacio { representacion } => representacion.to_string(),
            Celda::Bomba {
                representacion,
                alcance,
                de_traspaso: _,
            } => representacion.to_string() + &alcance.to_string(),
            Celda::Obstaculo { representacion } => representacion.to_string(),
            Celda::Enemigo { enemigo } => enemigo.obtener_representacion(),
            Celda::Desvio {
                representacion,
                direccion,
            } => representacion.to_string() + &direccion.to_string(),
        }
    }
}