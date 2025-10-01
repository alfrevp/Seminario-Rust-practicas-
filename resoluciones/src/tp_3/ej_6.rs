struct Examen {
    materia: String,
    nota: f64,
}

struct Estudiante {
    nombre: String,
    id: u32,
    calificaciones: Vec<Examen>,
}

impl Examen {
    fn new(materia: String, nota: f64) -> Examen {
        Examen {
            materia,
            nota,
        }
    }
}

impl Estudiante {
    fn new(nombre: String, id: u32, calificaciones: Vec<Examen>) -> Estudiante {
        Estudiante {nombre, id, calificaciones}
    }

    fn obtener_promedio(&self) -> f64 {
        let cant = self.calificaciones.len() as f64;
        if cant == 0.0 {
            return 0.0
        }
        let mut suma = 0.0;
        for elem in &self.calificaciones {
            suma += elem.nota;
        }
        suma as f64 / cant
    }

    fn obtener_calificacion_mas_alta(&self) -> f64 {
        let mut max: f64 = -1.0;
        for elem in &self.calificaciones {
            if elem.nota > max {
                max = elem.nota
            }
        }
        max
    }

    fn obtener_calificacion_mas_baja(&self) -> f64 {
        let mut min: f64 = 99.0;
        for elem in &self.calificaciones {
            if elem.nota < min {
                min = elem.nota
            }
        }
        min
    }
}


#[test]
    fn test_obtener_promedio_sin_calificaciones() {
        let estudiante = Estudiante::new("Juan".to_string(), 1, Vec::new());
        assert_eq!(estudiante.obtener_promedio(), 0.0);
    }

    #[test]
    fn test_obtener_promedio_con_calificaciones() {
        let calificaciones = vec![
            Examen::new("Matemáticas".to_string(), 90.0),
            Examen::new("Ciencias".to_string(), 85.0),
            Examen::new("Historia".to_string(), 75.0),
        ];
        let estudiante = Estudiante::new("Maria".to_string(), 2, calificaciones);
        assert_eq!(estudiante.obtener_promedio(), 83.33333333333333);
    }

    #[test]
    fn test_obtener_calificacion_mas_alta() {
        let calificaciones = vec![
            Examen::new("Matemáticas".to_string(), 90.0),
            Examen::new("Ciencias".to_string(), 85.0),
            Examen::new("Historia".to_string(), 75.0),
        ];
        let estudiante = Estudiante::new("Maria".to_string(), 2, calificaciones);
        assert_eq!(estudiante.obtener_calificacion_mas_alta(), 90.0);
    }

    #[test]
    fn test_obtener_calificacion_mas_baja() {
        let calificaciones = vec![
            Examen::new("Matemáticas".to_string(), 90.0),
            Examen::new("Ciencias".to_string(), 85.0),
            Examen::new("Historia".to_string(), 75.0),
        ];
        let estudiante = Estudiante::new("Maria".to_string(), 2, calificaciones);
        assert_eq!(estudiante.obtener_calificacion_mas_baja(), 75.0);
    }