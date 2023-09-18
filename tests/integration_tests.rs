use tp_individual::celda;
use tp_individual::explosion;
use tp_individual::mapa;
use tp_individual::enemigo;

#[cfg(test)]
mod test_integracion {
    use crate::celda::Celda;
    use crate::explosion::Explosion;
    use crate::mapa::Mapa;
    use crate::enemigo::Enemigo;
    fn mapa_3_x_3_con_desvios() -> Mapa {
        let enemigo = Celda::Enemigo { enemigo: Enemigo::new('F', 2, 0)  };
        let vacio = Celda::Vacio { representacion: '_' };
        let bomba = Celda::Bomba { representacion: 'S', alcance: 10, de_traspaso: false };
        let desvio_izquierda = Celda::Desvio { representacion: 'D', direccion: 'L' };
        let roca = Celda::Obstaculo { representacion: 'R' };
        let pared = Celda::Obstaculo { representacion: 'W' };
        Mapa::new(vec![
            vec![bomba.clone(), enemigo.clone(), desvio_izquierda.clone()],
            vec![bomba.clone(), vacio.clone(), vacio.clone()],
            vec![enemigo.clone(), vacio.clone(), vacio.clone()]
            ])
    }
    #[test]
    fn test() {
        

        assert!(true)
    }
}