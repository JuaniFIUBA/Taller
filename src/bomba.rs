use crate::Objeto;
#[derive(Clone, Copy)]
pub struct Bomba {
    representacion: char,
    alcance: usize,
    de_traspaso: bool,
    exploto: bool,
}

impl Bomba {
    pub fn new(representacion: char, alcance: usize) -> Bomba {
            if representacion == 'B' {
                Bomba {
                    representacion,
                    alcance,
                    de_traspaso: false,
                    exploto: false,
                }                
            } else {
                Bomba {
                    representacion,
                    alcance,
                    de_traspaso: true,
                    exploto: false,
                }
            }
        }
}


impl Objeto for Bomba {
    fn explotar(&mut self) {
        self.exploto = true;
        self.representacion = '_';
    }
    fn obtener_representacion(&self) -> String {
        self.representacion.to_string() + &self.alcance.to_string()
    }
    fn obtener_tipo(&self) -> char {
        self.representacion
    }
    fn exploto(&self) -> bool {
        self.exploto
    }
    fn obtener_atributo(&self) -> usize {
        self.alcance
    }
    fn traspasa(&self) -> bool {
        self.de_traspaso
    }
}