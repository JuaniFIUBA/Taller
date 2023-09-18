    use tp_individual::celda;
    use tp_individual::explosion;
    use tp_individual::mapa;
    use tp_individual::enemigo;

    #[cfg(test)]
    mod test_integracion {
        use tp_individual::enemigo;

        use crate::celda::Celda;
        use crate::explosion::Explosion;
        use crate::mapa::Mapa;
        use crate::enemigo::Enemigo;
        fn bomba_traspaso(alcance: usize) -> Celda {
            Celda::Bomba { representacion: 'B' , alcance: alcance, de_traspaso: true }
        }
        fn enemigo_con_pv(pv: usize, id: u32) -> Celda {
            Celda::Enemigo { enemigo: Enemigo::new('F', pv, id) }
        } 
        fn bomba_normal(alcance: usize) -> Celda { 
            Celda::Bomba { representacion: 'B', alcance: alcance, de_traspaso: false }
        }
        fn mapa_3_x_3_con_desvios() -> Mapa {
            let vacio = Celda::Vacio { representacion: '_' };
            let desvio_izquierda = Celda::Desvio { representacion: 'D', direccion: 'L' };
            let desvio_derecho = Celda::Desvio { representacion: 'D', direccion: 'R' };
            let roca = Celda::Obstaculo { representacion: 'R' };
            let pared = Celda::Obstaculo { representacion: 'W' };
            Mapa::new(vec![
                vec![bomba_traspaso(10).clone(), enemigo_con_pv(2, 0).clone(), desvio_izquierda.clone()],
                vec![bomba_normal(10).clone(), vacio.clone(), vacio.clone()],
                vec![desvio_derecho.clone(), roca.clone(), enemigo_con_pv(2, 1).clone()]
                ])
        }
        fn mapa2() -> Mapa {
            let enemigo_con_1pv = enemigo_con_pv(1, 0);
            let enemigo_con_2pv = enemigo_con_pv(2, 1);
            let vacio = Celda::Vacio { representacion: '_' };
            let desvio_izquierda = Celda::Desvio { representacion: 'D', direccion: 'L' };
            let desvio_derecho = Celda::Desvio { representacion: 'D', direccion: 'R' };
            let roca = Celda::Obstaculo { representacion: 'R' };
            let pared = Celda::Obstaculo { representacion: 'W' };
            Mapa::new(vec![
                vec![bomba_normal(10).clone(), enemigo_con_2pv.clone(), desvio_izquierda.clone()],
                vec![roca.clone(), vacio.clone(), vacio.clone()],
                vec![desvio_derecho.clone(), pared.clone(), enemigo_con_1pv.clone()]
                ])
        }
        #[test]
        fn test() {
            
            // testea que se traspasen las rocas con una bomba de traspaso pero no las parededs, tambien que una rafaga no pueda golpear
            // mas de una vez al mismo enemigo
            #[test] 
            fn test_integrador_1() {
                let mut mapa = mapa_3_x_3_con_desvios();
                Explosion::explotar("./mapa_post/mapa.txt", &mut mapa, 0, 0).unwrap();
                let mut enemigo1 = enemigo_con_pv(1, 0);
                let mut enemigo2 = enemigo_con_pv(1, 1);
                assert_eq!(mapa.obtener_celda(0, 1).unwrap(), &mut enemigo1);
                assert_eq!(mapa.obtener_celda(3, 3).unwrap(), &mut enemigo1);
            }
        }

        #[test] 
        fn test_integrador_2() {

        }
    }