#[derive(Clone, Copy, PartialEq)]
pub struct Enemigo {
    representacion: char,
    pv: usize,
    id: u32 // Sirve para que Rust distinga entre  distintas instancias
}

impl Enemigo {
    pub fn new(representacion: char, pv: usize, id: u32) -> Enemigo {
        Enemigo {representacion, pv, id}
    }
    pub fn obtener_representacion(&self) -> String {
        self.representacion.to_string() + &self.pv.to_string()
    }
    pub fn herir(&mut self) {
        self.pv -= 1;
    }
    pub fn esta_vivo(&self) -> bool {
        self.pv != 0
    }
}