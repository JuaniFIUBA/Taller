use crate::Objeto;

pub struct Desvio {
    representacion: char,
    direccion: char,
}

impl Desvio {
    pub fn new(representacion: char, direccion: char) -> Desvio {
        Desvio {representacion, direccion}
    }
}

impl Objeto for Desvio {
    fn obtener_representacion(&self) -> String {
        self.representacion.to_string() + &self.direccion.to_string()
    }
    fn obtener_tipo(&self) -> char {
        self.representacion
    }
    fn obtener_atributo(&self) -> usize {0}
    fn obtener_desvio(&self) -> char {self.direccion}
}
