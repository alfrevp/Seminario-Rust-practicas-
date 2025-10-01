struct Triangulo {
    lado_a: f64,
    lado_b: f64,
    lado_c: f64,
}

impl Triangulo {

    fn new(a:f64, b:f64, c:f64) -> Self {
        Triangulo {
            lado_a: a,
            lado_b: b,
            lado_c: c,
        }  
    }

    fn determinar_tipo(&self) -> String {
        if self.lado_a == self.lado_b && self.lado_b == self.lado_c {
            return "Equilatero".to_string();
        } else if self.lado_a == self.lado_b || self.lado_a == self.lado_c || self.lado_b == self.lado_c {
            return "Isosceles".to_string();
        } else {
            return "Escaleno".to_string();
        }
    }

    fn calcular_area(&self) -> f64 {
        let s = (self.lado_a + self.lado_b + self.lado_c) / 2.0;
        let area = (s * (s - self.lado_a) * (s - self.lado_b) * (s - self.lado_c)).sqrt();
        area
    }

    fn calcular_perimetro(&self) -> f64 {
        self.lado_a + self.lado_b + self.lado_c
    }
}

#[test]
    fn test_determinar_tipo() {
        // Triángulo equilátero
        let equilatero = Triangulo::new(3.0, 3.0, 3.0);
        assert_eq!(equilatero.determinar_tipo(), "Equilatero");

        // Triángulo isósceles
        let isosceles = Triangulo::new(3.0, 3.0, 4.0);
        assert_eq!(isosceles.determinar_tipo(), "Isosceles");

        // Triángulo escaleno
        let escaleno = Triangulo::new(3.0, 4.0, 5.0);
        assert_eq!(escaleno.determinar_tipo(), "Escaleno");
    }

    #[test]
    fn test_calcular_area() {
        // Triángulo equilátero
        let equilatero = Triangulo::new(3.0, 3.0, 3.0);
        assert!(equilatero.calcular_area() - 3.8971143170299753 < 0.000001);

        // Triángulo isósceles
        let isosceles = Triangulo::new(3.0, 3.0, 4.0);
        assert!(isosceles.calcular_area() - 4.47213595499958 < 0.000001);

        // Triángulo escaleno
        let escaleno = Triangulo::new(3.0, 4.0, 5.0);
        assert!(escaleno.calcular_area() - 6.0 < 0.000001);
    }

    #[test]
    fn test_calcular_perimetro() {
        // Triángulo equilátero
        let equilatero = Triangulo::new(3.0, 3.0, 3.0);
        assert_eq!(equilatero.calcular_perimetro(), 9.0);

        // Triángulo isósceles
        let isosceles = Triangulo::new(3.0, 3.0, 4.0);
        assert_eq!(isosceles.calcular_perimetro(), 10.0);

        // Triángulo escaleno
        let escaleno = Triangulo::new(3.0, 4.0, 5.0);
        assert_eq!(escaleno.calcular_perimetro(), 12.0);
    }