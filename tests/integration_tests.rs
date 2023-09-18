    use tp_individual::celda;
    use tp_individual::explosion;
    use tp_individual::mapa;

    #[cfg(test)]
    mod test_integracion {
        use crate::celda::Celda;
        use crate::explosion::Explosion;
        use crate::mapa::Mapa;

        
        // testea que se traspasen las rocas con una bomba de traspaso pero no las parededs, tambien que una rafaga no pueda golpear
        // mas de una vez al mismo enemigo
        #[test] 
        fn test_integrador_1() {
            let mut mapa = Mapa::new(vec![
                vec![Celda::bomba_traspaso(10), Celda::enemigo(2, 0), Celda::desvio('L').clone()],
                vec![Celda::bomba_normal(10), Celda::vacio(), Celda::vacio()],
                vec![Celda::desvio('R'), Celda::roca(), Celda::enemigo(2, 1)]
                ]);
            Explosion::explotar("./mapa_post/mapa.txt", &mut mapa, 0, 0).unwrap();
            assert_eq!(mapa.obtener_celda(0, 1).unwrap(), &mut Celda::enemigo(1, 0));
            assert_eq!(mapa.obtener_celda(2, 2).unwrap(), &mut Celda::enemigo(1, 1));
        }

        #[test] 
        fn test_integrador_2() {
            let mut mapa = Mapa::new(vec![
                vec![Celda::bomba_traspaso(1), Celda::vacio(), Celda::desvio('L').clone()],
                vec![Celda::enemigo(1, 0), Celda::vacio(), Celda::vacio()],
                vec![Celda::desvio('R'), Celda::roca(), Celda::enemigo(2, 1)]
                ]);
                Explosion::explotar("./mapa_post/mapa.txt", &mut mapa, 0, 0).unwrap();
                assert_eq!(mapa.obtener_celda(1, 0).unwrap(), &mut Celda::vacio());
                assert_eq!(mapa.obtener_celda(0, 0).unwrap(), &mut Celda::vacio());
        }
    }