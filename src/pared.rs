use crate::Objeto; 
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
    fn obtener_tipo(&self) -> char{
        self.representacion
    }
}