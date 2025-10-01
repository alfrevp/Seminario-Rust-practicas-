struct Producto {
    nombre: String,
    precio_bruto: f64,
    id: u32,
}

impl Producto {

    fn new(nombre: String, precio_bruto:f64, id:u32) -> Self {
        Producto {
            nombre,
            precio_bruto,
            id,
        }
    }

    fn calcular_impuestos(&self, porcentaje_de_impuestos: f64) -> f64 {
        self.precio_bruto * (porcentaje_de_impuestos / 100.0)
    }

    fn aplicar_descuento(&self, porcentaje_de_descuento: f64) -> f64 {
        self.precio_bruto * (porcentaje_de_descuento / 100.0)
    }

    fn calcular_precio_total(&self, porcentaje_de_impuestos: Option<f64>, porcentaje_de_descuento: Option<f64>) -> f64 {
        let impuestos = match porcentaje_de_impuestos {
            Some(porcentaje) => self.calcular_impuestos(porcentaje),
            None => 0.0,
        };
        let descuento = match porcentaje_de_descuento {
            Some(porcentaje) => self.aplicar_descuento(porcentaje),
            None => 0.0,
        };
        self.precio_bruto + impuestos - descuento
    }
}

#[test]
    fn test_calcular_impuestos() {
        let producto = Producto::new("Producto A".to_string(), 100.0, 1);
        assert_eq!(producto.calcular_impuestos(10.0), 10.0);
    }

    #[test]
    fn test_aplicar_descuento() {
        let producto = Producto::new("Producto B".to_string(), 100.0, 2);
        assert_eq!(producto.aplicar_descuento(20.0), 20.0);
    }

    #[test]
    fn test_calcular_precio_total_con_impuestos_y_descuento() {
        let producto = Producto::new("Producto C".to_string(), 100.0, 3);
        assert_eq!(producto.calcular_precio_total(Some(10.0), Some(20.0)), 90.0);
    }

    #[test]
    fn test_calcular_precio_total_con_impuestos() {
        let producto = Producto::new("Producto D".to_string(), 100.0, 4);
        assert_eq!(producto.calcular_precio_total(Some(10.0), None), 110.0);
    }

    #[test]
    fn test_calcular_precio_total_con_descuento() {
        let producto = Producto::new("Producto E".to_string(), 100.0, 5);
        assert_eq!(producto.calcular_precio_total(None, Some(20.0)), 80.0);
    }

    #[test]
    fn test_calcular_precio_total_sin_impuestos_ni_descuento() {
        let producto = Producto::new("Producto F".to_string(), 100.0, 6);
        assert_eq!(producto.calcular_precio_total(None, None), 100.0);
    }