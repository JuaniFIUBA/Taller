use crate::Objeto; 
#[derive(Clone, Copy)]
pub struct Obstaculo {
    representacion: char
}

impl Obstaculo {
    pub fn new(representacion: char) -> Obstaculo {
        Obstaculo {
            representacion
        }
    }
}


impl Objeto for Obstaculo {
    fn obtener_representacion(&self) -> String {
        String::from(self.representacion)
    }
    fn obtener_tipo(&self) -> char{
        self.representacion
    }
    fn obtener_atributo(&self) -> usize {
        0
    }
}