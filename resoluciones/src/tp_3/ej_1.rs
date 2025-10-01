struct Persona {
    nombre: String,
    edad: u32,
    dir: Option<String>,
}

impl Persona {
    
    fn new (nombre: String, edad: u32, dir: Option<String>) -> Persona {
        Persona {
            nombre,
            edad,
            dir,
        }
    }

    fn to_string (&self) -> String {
        let dir = match &self.dir {
            Some(dir) => dir.clone(),
            None => "Desconocida".to_string(),
        };
        format!("Nombre: {}. Edad: {}. Direccion: {}.", self.nombre, self.edad, dir)
    }

    fn obtener_edad (&self) -> u32 {
        self.edad
    }

    fn actualizar_direccion (&mut self, nueva: Option<String>) {
        self.dir = nueva;
    }
}

#[test]
fn test_persona() {
    let mut persona1 = Persona::new("Alfredo".to_string(), 32, Some("Diagonal 79 870".to_string()));
    assert_eq!(32, persona1.obtener_edad());
    persona1.actualizar_direccion(Some("otra direccion".to_string()));
    assert_ne!(Some("Diagonal 79 870".to_string()), persona1.dir);
}

#[test]
fn test_persona_sin_dir() {
    let persona2 = Persona::new("Galle".to_string(), 29, None);
    assert_eq!(29, persona2.obtener_edad());
    assert_eq!("Nombre: Galle. Edad: 29. Direccion: Desconocida.", persona2.to_string());
}
