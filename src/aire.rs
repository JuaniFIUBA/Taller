use crate::Objeto;

#[derive(Clone, Copy)]
pub struct Aire {
    representacion: char,
}

impl Aire {
    pub fn new(representacion: char) -> Aire{
        Aire {
            representacion,
        }
    }

}


impl Objeto for Aire {
    fn explotar(&mut self){}
    fn obtener_representacion(&self) -> String {
        self.representacion.to_string()
    }
    fn obtener_tipo(&self) -> char {
        self.representacion
    }
    fn obtener_atributo(&self) -> usize {
        0
    }
}
