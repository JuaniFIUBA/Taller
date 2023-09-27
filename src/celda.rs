use super::enemigo::Enemigo;
#[derive(Debug, Clone, PartialEq)]

/// Enum que representa los tipos de bombas disponibles
pub enum TipoDeBomba { 
    /// Bomba comun, sin traspaso
    BombaOrdinaria,
    /// Bomba con capacidad de traspasar Rocas
    BombaDeTraspaso
}
/// Enum que representa los tipos de celdas/casilleros en el juego Bomberman R.
#[derive(Debug, PartialEq, Clone)]
pub enum Celda {
    /// Celda vacia con una representación de un carácter.
    Vacio { representacion: char },

    /// Celda de bomba con una representación de un carácter, alcance y un tipo.
    Bomba {
        representacion: char,
        alcance: usize,
        tipo_bomba: TipoDeBomba,
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
    /// Crea una celda vacía con una representación por defecto.
    ///
    /// # Returns
    ///
    /// Celda vacia
    pub fn vacio() -> Celda {
        Celda::Vacio {
            representacion: '_',
        }
    }

    /// Crea una celda de pared.
    ///
    /// # Returns
    ///
    /// Celda con pared

    pub fn pared() -> Celda {
        Celda::Obstaculo {
            representacion: 'W',
        }
    }

    /// Crea una celda de roca.
    ///
    /// # Returns
    ///
    /// Celda con roca
    pub fn roca() -> Celda {
        Celda::Obstaculo {
            representacion: 'R',
        }
    }

    /// Crea una celda de bomba normal con una representación, alcance y sin propiedad de traspaso.
    ///
    /// # Argumentos
    ///
    /// *`alcance`: alcance de la bomba
    ///
    /// # Returns
    ///
    /// Celda con bomba normal
    pub fn bomba_normal(alcance: usize) -> Celda {
        Celda::Bomba {
            representacion: 'B',
            alcance,
            tipo_bomba: TipoDeBomba::BombaOrdinaria,
        }
    }

    /// Crea una celda de bomba de traspaso con una representación, alcance y propiedad de traspaso.
    /// # Argumentos
    ///
    /// *`alcance`: alcance de la bomba
    ///
    /// # Returns
    ///
    /// Celda con bomba de traspaso
    pub fn bomba_traspaso(alcance: usize) -> Celda {
        Celda::Bomba {
            representacion: 'S',
            alcance,
            tipo_bomba: TipoDeBomba::BombaDeTraspaso,
        }
    }

    /// Crea una celda de enemigo con puntos de vida y un identificador.
    /// # Argumentos
    ///
    /// *`pv`: puntos de vida del enemigo
    /// *`id`: identificador del enemigo
    ///
    /// # Returns
    ///
    /// Celda con enemigo

    pub fn enemigo(pv: usize) -> Celda {
        Celda::Enemigo {
            enemigo: Enemigo::new('F', pv),
        }
    }

    /// Crea una celda de desvío con una representación y dirección.
    /// # Argumentos
    ///
    /// *`desvio`: direccion del desvio
    ///
    /// # Returns
    ///
    /// Celda con desvio
    pub fn desvio(direccion: char) -> Celda {
        Celda::Desvio {
            representacion: 'D',
            direccion,
        }
    }

}


trait Display {
    fn display(&self) -> String;
}

// Un trait que define el comportamiento de ObtenerRepresentacion
pub trait ObtenerRepresentacion {
    fn obtener_representacion(&self) -> String;
}
impl ObtenerRepresentacion for Celda {
    fn obtener_representacion(&self) -> String {
        match self {
            Celda::Vacio { representacion } => representacion.to_string(),
            Celda::Bomba {
                representacion,
                alcance,
                tipo_bomba: _,
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
impl std::fmt::Display for Celda {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let representacion = self.obtener_representacion();
        write!(f, "{}", representacion)
    }
}   