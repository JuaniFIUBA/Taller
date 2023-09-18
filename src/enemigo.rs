/// Representa a un enemigo en algún contexto.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Enemigo {
    /// Carácter que representa visualmente al enemigo.
    representacion: char,

    /// Puntos de vida del enemigo.
    pv: usize,

    /// Identificador único para distinguir entre distintas instancias de enemigos.
    id: u32,
}

impl Enemigo {
    /// Crea una nueva instancia de `Enemigo` con la representación visual, puntos de vida
    /// y un identificador único.
    ///
    /// # Argumentos
    ///
    /// * `representacion`: Carácter que representa visualmente al enemigo.
    /// * `pv`: Puntos de vida del enemigo.
    /// * `id`: Identificador único para distinguir entre distintas instancias de enemigos.
    ///
    /// # Ejemplo
    ///
    /// ```
    /// use tp_individual::enemigo::Enemigo;
    ///
    /// let enemigo = Enemigo::new('E', 4, 1);
    /// ```
    pub fn new(representacion: char, pv: usize, id: u32) -> Enemigo {
        Enemigo {
            representacion,
            pv,
            id,
        }
    }

    /// Obtiene una representación del enemigo como una cadena de caracteres que combina
    /// la representación visual y los puntos de vida.
    ///
    /// # Ejemplo
    ///
    /// ```
    /// use tp_individual::enemigo::Enemigo;
    ///
    /// let enemigo = Enemigo::new('E', 4, 1);
    /// let representacion = enemigo.obtener_representacion();
    /// assert_eq!(representacion, "E4");
    /// ```
    pub fn obtener_representacion(&self) -> String {
        self.representacion.to_string() + &self.pv.to_string()
    }

    /// Reduce los puntos de vida del enemigo en 1, simbolizando daño recibido.
    pub fn herir(&mut self) {
        self.pv -= 1;
    }

    /// Verifica si el enemigo todavía tiene puntos de vida y está vivo.
    ///
    /// # Returns
    ///
    /// Devuelve `true` si el enemigo tiene puntos de vida restantes, o `false` si no.
    ///
    /// # Ejemplo
    ///
    /// ```
    /// use tp_individual::enemigo::Enemigo;
    ///
    /// let mut enemigo = Enemigo::new('E', 2, 1);
    /// assert!(enemigo.esta_vivo());
    ///
    /// enemigo.herir();
    /// assert!(enemigo.esta_vivo());
    /// ```
    pub fn esta_vivo(&self) -> bool {
        self.pv != 0
    }
}
