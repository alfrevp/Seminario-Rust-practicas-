#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Fecha {
    dia: u32,
    mes: u32,
    anio: u32,
}

impl Fecha {

    pub fn new (dia: u32, mes: u32, anio: u32) -> Fecha {
        Fecha {
            dia,
            mes,
            anio,
        }
    }

    fn es_fecha_valida (&self) -> bool {
        match self.mes {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => self.dia > 0 && self.dia <= 31,
            4 | 6 | 9 | 11 => self.dia > 0 && self.dia <= 30,
            2 => {
                if self.dia > 0 {
                    if self.es_bisiesto() {
                        self.dia <= 29
                    } else {
                        self.dia <= 28
                    }
                } else {
                    false
                } 
                }
            _ => false,
        }
    }

    fn es_bisiesto(&self) -> bool {
        (self.anio % 4 == 0 && self.anio % 100 != 0) || self.anio % 400 == 0
    }

    pub fn sumar_dias(&mut self, dias: u32) -> &Self {
        for _ in 0..dias {
            self.dia += 1;
            if !self.es_fecha_valida() {
                self.dia = 1;
                self.mes += 1;
                if self.mes > 12 {
                    self.mes = 1;
                    self.anio += 1;
                }
            }
        }
        self
    }

    pub fn restar_dias(&mut self, dias: u32) {
        for _ in 0..dias {
            if self.dia > 1 {
                self.dia -= 1;
            } else {
                if self.mes > 1 {
                    self.mes -= 1;
                    match self.mes {
                        1 | 3 | 5 | 7 | 8 | 10 | 12 => self.dia = 31,
                        4 | 6 | 9 | 11 => self.dia = 30,
                        2 => {
                            if self.es_bisiesto() {
                                self.dia = 29;
                            } else {
                                self.dia = 28;
                            }
                        }
                        _ => (),
                    }
                } else {
                    self.mes = 12;
                    self.anio -= 1;
                    self.dia = 31;
                }
            }
        }
    }


    pub fn es_mayor(&self, otra_fecha: &Fecha) -> bool {
        if self.anio > otra_fecha.anio {
            true
        } else if self.anio < otra_fecha.anio {
            false
        } else if self.mes > otra_fecha.mes {
            true
        } else if self.mes < otra_fecha.mes {
            false
        } else {
            self.dia > otra_fecha.dia
        }
    }
}


#[test]
    fn test_es_fecha_valida() {
        // Casos de fechas válidas
        assert!(Fecha::new(1, 1, 2022).es_fecha_valida());
        assert!(Fecha::new(31, 12, 2022).es_fecha_valida());
        assert!(Fecha::new(29, 2, 2024).es_fecha_valida()); // Año bisiesto

        // Casos de fechas inválidas
        assert!(!Fecha::new(0, 1, 2022).es_fecha_valida()); // Día 0
        assert!(!Fecha::new(32, 1, 2022).es_fecha_valida()); // Día 32
        assert!(!Fecha::new(29, 2, 2023).es_fecha_valida()); // Año no bisiesto
        assert!(!Fecha::new(31, 4, 2022).es_fecha_valida()); // Mes 4 no tiene 31 días
    }

    #[test]
    fn test_es_bisiesto() {
        // Casos de años bisiestos
        assert!(Fecha::new(1, 1, 2020).es_bisiesto());
        assert!(Fecha::new(1, 1, 2000).es_bisiesto());

        // Casos de años no bisiestos
        assert!(!Fecha::new(1, 1, 2021).es_bisiesto());
        assert!(!Fecha::new(1, 1, 1900).es_bisiesto());
    }

    #[test]
    fn test_sumar_y_restar_dias() {
        let mut fecha = Fecha::new(1, 1, 2022);
        
        // Sumar y restar días
        fecha.sumar_dias(30);
        assert_eq!(fecha.dia, 31);
        fecha.restar_dias(15);
        assert_eq!(fecha.dia, 16);
    }

    #[test]
    fn test_es_mayor() {
        let fecha1 = Fecha::new(1, 1, 2022);
        let fecha2 = Fecha::new(31, 12, 2021);

        // Fecha1 es mayor que fecha2
        assert!(fecha1.es_mayor(&fecha2));

        // Fecha2 no es mayor que fecha1
        assert!(!fecha2.es_mayor(&fecha1));

        // Misma fecha
        let fecha3 = Fecha::new(1, 1, 2022);
        assert!(!fecha3.es_mayor(&fecha1));
    }