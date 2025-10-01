// Alfredo Moracho, legajo 15080/8, discord: alfrevp

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

    // DESDE ACA LO CORRESPONDIENTE AL E2-T2 :

    fn listar_autos_por_marca(&self, marca:String) -> ReporteGeneral {
        let mut cant = 0;
        let mut listado: Vec<ReporteAuto> = Vec::new();
        for i in &self.lista_autos {
            if i.marca == marca {
                let reporte_auto = ReporteAuto::new(i.modelo.clone(), i.anio, i.color_comparable().to_string(), i.calcular_precio());
                listado.push(reporte_auto);
                cant += 1;
            }
        }
        let reporte_general = ReporteGeneral::new(marca, listado, cant);
        reporte_general
    }
}

struct ReporteGeneral {
    marca: String,
    listado: Vec<ReporteAuto>,
    total_autos: i32,
}
impl ReporteGeneral {
    fn new(marca:String, listado:Vec<ReporteAuto>, total_autos:i32) -> Self {
        ReporteGeneral {marca, listado, total_autos}
    }
}


struct ReporteAuto {
    modelo: String,
    anio: u32,
    color: String,
    precio: f64,
}
impl ReporteAuto {
    fn new(modelo:String, anio:u32, color:String, precio:f64) -> Self {
        ReporteAuto {modelo, anio, color, precio}
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

#[test]
fn test_listar_autos_por_marca() {
    let mut concesionario = ConcesionarioAuto::new("Motosport".to_string(), "Algun lado".to_string(), 5);
    let auto1 = Auto::new("Marca1".to_string(), "ModeloA".to_string(), 2015, 4500000.00, Color::Rojo);
    let auto2 = Auto::new("Marca2".to_string(), "ModeloB".to_string(), 1999, 51656165.50, Color::Blanco);
    let auto3 = Auto::new("Marca1".to_string(), "ModeloC".to_string(), 2023, 98489699.00, Color::Azul);
    let auto4 = Auto::new("Marca3".to_string(), "ModeloD".to_string(), 2021, 7500000.00, Color::Verde);
    let auto5 = Auto::new("Marca1".to_string(), "ModeloE".to_string(), 2018, 8600000.00, Color::Amarillo);
    
    concesionario.agregar_auto(auto1);
    concesionario.agregar_auto(auto2);
    concesionario.agregar_auto(auto3);
    concesionario.agregar_auto(auto4);
    concesionario.agregar_auto(auto5);
    
    let reporte = concesionario.listar_autos_por_marca("Marca1".to_string());
    
    assert_eq!(reporte.marca, "Marca1");
    assert_eq!(reporte.total_autos, 3);
    assert_eq!(reporte.listado.len(), 3);
    
    assert!(reporte.listado.iter().any(|auto| auto.modelo == "ModeloA" && auto.anio == 2015 && auto.color == "rojo" && auto.precio == 4500000.00 * 1.25));
    assert!(reporte.listado.iter().any(|auto| auto.modelo == "ModeloC" && auto.anio == 2023 && auto.color == "azul" && auto.precio == 98489699.00 * 1.25));
    assert!(reporte.listado.iter().any(|auto| auto.modelo == "ModeloE" && auto.anio == 2018 && auto.color == "amarillo" && auto.precio == 8600000.00 * 1.25));
}