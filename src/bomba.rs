#[derive(Debug, Clone, PartialEq)]
/// Enum que representa los tipos de bombas disponibles
pub enum TipoDeBomba { 
    /// Bomba comun, sin traspaso
    BombaOrdinaria,
    /// Bomba con capacidad de traspasar Rocas
    BombaDeTraspaso
}