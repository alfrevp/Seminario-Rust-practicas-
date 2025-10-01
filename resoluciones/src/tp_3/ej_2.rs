struct Rectangulo {
    longitud: u32,
    ancho: u32,
}

impl Rectangulo {
    
    fn new (longitud: u32, ancho: u32) -> Rectangulo {
        Rectangulo {
            longitud,
            ancho,
        }
    }

    fn calcular_area(&self) -> u32 {
        self.longitud * self.ancho
    }

    fn calcular_perimetro(&self) -> u32 {
        2*self.longitud + 2*self.ancho
    }

    fn es_cuadrado(&self) -> bool {
        self.longitud == self.ancho
    }
}

#[test]
fn test_rectangulo() {
    let rectangulo1 = Rectangulo::new(5, 7);
    assert_eq!(35, rectangulo1.calcular_area());
    assert_eq!(24, rectangulo1.calcular_perimetro());
    assert!(!rectangulo1.es_cuadrado());
}