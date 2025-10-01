enum Color {
    Rojo,
    Verde,
    Azul,
    Amarillo,
    Blanco,
    Negro,
}

struct Auto {
    marca: String,
    modelo: String,
    anio: u32,
    precio_bruto: f64,
    color: Color,
}

impl Auto {
    fn new(marca:String, modelo:String, anio:u32, precio_bruto:f64, color:Color) -> Auto {
        Auto {marca, modelo, precio_bruto, anio, color}
    }

    fn calcular_precio(&self) -> f64 {
        let mut total = self.precio_bruto;
        if self.color_comparable() == "rojo" || self.color_comparable() == "amarillo" || self.color_comparable() == "azul" {
            let recargo_color = self.precio_bruto * 0.25;
            total += recargo_color;  
        } else {
            let descuento_color = self.precio_bruto * 0.10;
            total -= descuento_color; 
        }
        if self.marca == "BMW" {
            let recargo_marca = self.precio_bruto as f64 * 0.15;
            total += recargo_marca;
        }
        if self.anio < 2000 {
            let descuento_anio = self.precio_bruto as f64 * 0.05;
            total -= descuento_anio;
        }
        total
    }

    fn es_igual(&self, auto:&Auto) -> bool {
        self.marca == auto.marca &&
        self.modelo == auto.modelo &&
        self.anio == auto.anio &&
        self.precio_bruto == auto.precio_bruto &&
        self.color_comparable() == auto.color_comparable()
    }

    fn color_comparable(&self) -> &str {
        match self.color {
            Color::Amarillo => "amarillo",
            Color::Azul => "azul",
            Color::Blanco => "blanco",
            Color::Negro => "negro",
            Color::Rojo => "rojo",
            Color::Verde => "verde",
        }
    }
}

struct ConcesionarioAuto {
    nombre: String,
    dir: String,
    cap_max: usize,
    lista_autos: Vec<Auto>,
}

impl ConcesionarioAuto {
    fn new(nombre:String, dir:String, cap_max:usize) -> ConcesionarioAuto {
        ConcesionarioAuto {
            nombre,
            dir,
            cap_max,
            lista_autos: Vec::new(),
        }
    }

    fn agregar_auto (&mut self, auto:Auto) -> bool {
        if self.lista_autos.len() < self.cap_max {
            self.lista_autos.push(auto);
            return true
        }
        false
    }

    fn eliminar_auto(&mut self, auto:Auto) {
        if let Some(pos) = self.lista_autos.iter().position(|elem| elem.es_igual(&auto)) { 
            self.lista_autos.remove(pos);
        } 
    }

    fn buscar_auto(&self, auto:&Auto) -> Option<&Auto> {
        self.lista_autos.iter().find(|elem| elem.es_igual(auto))
    }
   
}



#[test]
fn test_agregar_auto() {
    let mut concesionario: ConcesionarioAuto = ConcesionarioAuto::new("Motosport".to_string(), "Algun lado".to_string(), 2);
    let auto1 = Auto::new("Marca1".to_string(), "ModeloA".to_string(), 2015, 4500000.00, Color::Rojo);
    let auto2 = Auto::new("Marca2".to_string(), "ModeloB".to_string(), 1999, 51656165.50, Color::Blanco);
    let auto3 = Auto::new("Marca3".to_string(), "ModeloC".to_string(), 2023, 98489699.00, Color::Azul);
    assert!(concesionario.agregar_auto(auto1));
    assert!(concesionario.agregar_auto(auto2));
    assert!(!concesionario.agregar_auto(auto3));
    assert_eq!(concesionario.lista_autos[1].marca, "Marca2");
    assert_eq!(concesionario.lista_autos[1].modelo, "ModeloB");
    assert_eq!(concesionario.lista_autos[1].anio, 1999);
    assert_eq!(concesionario.lista_autos[1].precio_bruto, 51656165.50);
    assert_eq!(concesionario.lista_autos[1].color_comparable(), "blanco");
    assert_eq!(concesionario.lista_autos.len(), 2);
}

#[test]
fn test_buscar_y_eliminar_auto() {
    let mut concesionario = ConcesionarioAuto::new("Motosport".to_string(), "Algun lado".to_string(), 3);
    let auto1 = Auto::new("Marca1".to_string(), "ModeloA".to_string(), 2015, 4500000.00, Color::Rojo);
    let auto2 = Auto::new("Marca2".to_string(), "ModeloB".to_string(), 1999, 51656165.50, Color::Blanco);
    let auto3 = Auto::new("Marca3".to_string(), "ModeloC".to_string(), 2023, 98489699.00, Color::Azul);
    concesionario.agregar_auto(auto1);
    concesionario.agregar_auto(auto2);
    concesionario.agregar_auto(auto3);
    assert_eq!(concesionario.lista_autos.len(), 3);
    
    let auto_existente = Auto::new("Marca2".to_string(), "ModeloB".to_string(), 1999, 51656165.50, Color::Blanco);
    let mut auto_encontrado = concesionario.buscar_auto(&auto_existente);
    assert!(auto_encontrado.is_some());
    assert!(auto_encontrado.unwrap().es_igual(&auto_existente));

    let auto_inexistente = Auto::new("MarcaX".to_string(), "ModeloX".to_string(), 9999, 99999999.99, Color::Negro);
    auto_encontrado = concesionario.buscar_auto(&auto_inexistente);
    assert!(auto_encontrado.is_none());
    
    let auto_a_eliminar = Auto::new("Marca2".to_string(), "ModeloB".to_string(), 1999, 51656165.50, Color::Blanco);
    concesionario.eliminar_auto(auto_a_eliminar);

    assert_eq!(concesionario.lista_autos.len(), 2);
}

#[test]
fn test_calcular_precio() {
    let auto1 = Auto::new("Marca1".to_string(), "ModeloA".to_string(), 2015, 10000.00, Color::Rojo);
    assert_eq!(auto1.calcular_precio(), 12500.00);

    let auto2 = Auto::new("BMW".to_string(), "ModeloX".to_string(), 2001, 10000.00, Color::Blanco);
    assert_eq!(auto2.calcular_precio(), 10500.00);

    let auto3 = Auto::new("Marca3".to_string(), "ModeloC".to_string(), 1999, 10000.00, Color::Azul);
    assert_eq!(auto3.calcular_precio(), 12000.00);
}
