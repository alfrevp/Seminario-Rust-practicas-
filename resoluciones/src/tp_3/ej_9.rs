use std::collections::VecDeque;
use super::ej_3::Fecha;

enum TipoAnimal {
    Perro,
    Gato,
    Caballo,
    Otros,
}
struct Mascota {
    nombre: String,
    edad: u32,
    tipo: TipoAnimal,
    dueño: Dueño,
}
impl Mascota {
    fn new(nombre:String, edad:u32, tipo:TipoAnimal, dueño:Dueño) -> Mascota {
        Mascota {nombre, edad, tipo, dueño}
    }

    fn es_igual(&self, otra_mascota:&Mascota) -> bool {
        self.nombre == otra_mascota.nombre &&
        self.edad == otra_mascota.edad &&
        self.tipo_comparable() == otra_mascota.tipo_comparable() &&
        self.dueño.es_igual(&otra_mascota.dueño)
    }

    fn tipo_comparable(&self) -> &str {
        match self.tipo {
            TipoAnimal::Caballo => "caballo",
            TipoAnimal::Gato => "gato",
            TipoAnimal::Perro => "perro",
            TipoAnimal::Otros => "otros",
        }
    }
}

struct Dueño {
    nombre: String,
    direccion: String,
    telefono: u32,
}
impl Dueño {
    fn new(nombre:String, direccion:String, telefono:u32) -> Dueño {
        Dueño{nombre, direccion, telefono}
    }

    fn es_igual(&self,  otro_dueño:&Dueño) -> bool {
        self.nombre == otro_dueño.nombre &&
        self.direccion == otro_dueño.direccion &&
        self.telefono == otro_dueño.telefono
    }
}

struct Atencion {
    mascota: Mascota,
    diagnostico: String,
    tratamiento: String,
    prox_visita: Option<Fecha>,
}
impl Atencion {
    fn new(mascota:Mascota, diagnostico:String, tratamiento:String, prox_visita:Option<Fecha>) -> Self {
        Atencion {
            mascota, 
            diagnostico, 
            tratamiento, 
            prox_visita,
            }
    }
}

struct Veterinaria {
    nombre: String,
    direccion: String,
    id: u32,
    cola_atencion: VecDeque<Mascota>,
    registro_atenciones: Vec<Atencion>,
}
impl Veterinaria {
    fn new(nombre:String, direccion:String, id:u32) -> Veterinaria {
        Veterinaria {nombre, direccion, id, cola_atencion: VecDeque::new(), registro_atenciones: Vec::new()}
    }

    fn agregar_mascota(&mut self, mascota:Mascota) {
        self.cola_atencion.push_back(mascota);
    }

    fn agregar_mascota_prioritaria(&mut self, mascota:Mascota) {
        self.cola_atencion.push_front(mascota);
    }

    fn atender_proxima(&mut self) -> Option<Mascota> {
        self.cola_atencion.pop_front()
    }

    fn eliminar_mascota(&mut self, mascota:Mascota) {
        if let Some(pos) = self.cola_atencion.iter().position(|m| m.es_igual(&mascota)) {
            self.cola_atencion.remove(pos);
        }
    }

    fn registrar_atencion(&mut self, atencion:Atencion) {
        self.registro_atenciones.push(atencion);
    }

    fn buscar_atencion(&self, nombre_mascota:&str, nombre_dueño:&str, tel:u32) -> Option<&Atencion> {
        self.registro_atenciones.iter().find(|a| a.mascota.nombre == nombre_mascota && a.mascota.dueño.nombre == nombre_dueño && a.mascota.dueño.telefono == tel)
    }

    fn modificar_diagnostico(&mut self, mascota:&Mascota, nuevo_diagnostico: String) -> bool {
        if let Some(pos) = self.registro_atenciones.iter().position(|a| a.mascota.es_igual(mascota)) {
            self.registro_atenciones[pos].diagnostico = nuevo_diagnostico;
            true
        } else {
            false
        }
    }

    fn modificar_fecha(&mut self, mascota:&Mascota, nueva_fecha:Option<Fecha>) -> bool {
        if let Some(pos) = self.registro_atenciones.iter().position(|a| a.mascota.es_igual(mascota)) {
            self.registro_atenciones[pos].prox_visita = nueva_fecha;
            true
        } else {
            false
        }
    }

    fn eliminar_atencion(&mut self, mascota:&Mascota) -> Option<Atencion> {
        if let Some(pos) = self.registro_atenciones.iter().position(|a| a.mascota.es_igual(mascota)) {
            Some(self.registro_atenciones.remove(pos))
        } else {
            None
        }
    }
}


#[test]
fn test_agregar_mascota() {
    let mut vet = Veterinaria::new("Vet1".to_string(), "Dirección 123".to_string(), 1);
    let dueño = Dueño::new("Juan".to_string(), "Calle Falsa 123".to_string(), 5551234);
    let mascota = Mascota::new("Fido".to_string(), 3, TipoAnimal::Perro, dueño);

    vet.agregar_mascota(mascota);

    assert_eq!(vet.cola_atencion.len(), 1);
    let dueño_agregado = Dueño::new("Juan".to_string(), "Calle Falsa 123".to_string(), 5551234);
    let mascota_agregada = Mascota::new("Fido".to_string(), 3, TipoAnimal::Perro, dueño_agregado);
    assert!(vet.cola_atencion.front().unwrap().es_igual(&mascota_agregada));
}
 
#[test]
fn test_agregar_mascota_prioritaria() {
    let mut vet = Veterinaria::new("Vet1".to_string(), "Dirección 123".to_string(), 1);
    let dueño1 = Dueño::new("Juan".to_string(), "Calle Falsa 123".to_string(), 5551234);
    let mascota1 = Mascota::new("Fido".to_string(), 3, TipoAnimal::Perro, dueño1);
    let dueño2 = Dueño::new("Maria".to_string(), "Calle Verdadera 456".to_string(), 5555678);
    let mascota2 = Mascota::new("Luna".to_string(), 2, TipoAnimal::Gato, dueño2);

    vet.agregar_mascota(mascota1);
    vet.agregar_mascota_prioritaria(mascota2);

    assert_eq!(vet.cola_atencion.len(), 2);
    let dueño_prioritario = Dueño::new("Maria".to_string(), "Calle Verdadera 456".to_string(), 5555678);
    let mascota_prioritaria = Mascota::new("Luna".to_string(), 2, TipoAnimal::Gato, dueño_prioritario);
    assert!(vet.cola_atencion.front().unwrap().es_igual(&mascota_prioritaria));
}


#[test]
fn test_atender_proxima() {
    let mut vet = Veterinaria::new("Vet1".to_string(), "Dirección 123".to_string(), 1);
    let dueño = Dueño::new("Juan".to_string(), "Calle Falsa 123".to_string(), 5551234);
    let mascota = Mascota::new("Fido".to_string(), 3, TipoAnimal::Perro, dueño);

    vet.agregar_mascota(mascota);
    let atendida = vet.atender_proxima().unwrap();

    let dueño_mascota_a_atender = Dueño::new("Juan".to_string(), "Calle Falsa 123".to_string(), 5551234);
    let mascota_a_atender = Mascota::new("Fido".to_string(), 3, TipoAnimal::Perro, dueño_mascota_a_atender);
    assert!(atendida.es_igual(&mascota_a_atender));
    assert_eq!(vet.cola_atencion.len(), 0);
}

#[test]
fn test_eliminar_mascota() {
    let mut vet = Veterinaria::new("Vet1".to_string(), "Dirección 123".to_string(), 1);
    let dueño = Dueño::new("Juan".to_string(), "Calle Falsa 123".to_string(), 5551234);
    let mascota = Mascota::new("Fido".to_string(), 3, TipoAnimal::Perro, dueño);

    vet.agregar_mascota(mascota);
    assert_eq!(vet.cola_atencion.len(), 1);

    let dueño_mascota_a_eliminar = Dueño::new("Juan".to_string(), "Calle Falsa 123".to_string(), 5551234);
    let mascota_a_eliminar = Mascota::new("Fido".to_string(), 3, TipoAnimal::Perro, dueño_mascota_a_eliminar);
    vet.eliminar_mascota(mascota_a_eliminar);
    assert_eq!(vet.cola_atencion.len(), 0);
}

#[test]
fn test_registrar_atencion() {
    let mut vet = Veterinaria::new("Vet1".to_string(), "Dirección 123".to_string(), 1);
    let dueño = Dueño::new("Juan".to_string(), "Calle Falsa 123".to_string(), 5551234);
    let mascota = Mascota::new("Fido".to_string(), 3, TipoAnimal::Perro, dueño);
    let atencion = Atencion::new(mascota, "Diagnóstico".to_string(), "Tratamiento".to_string(), None);

    vet.registrar_atencion(atencion);

    assert_eq!(vet.registro_atenciones.len(), 1);
    let dueño_mascota_atendida = Dueño::new("Juan".to_string(), "Calle Falsa 123".to_string(), 5551234);
    let mascota_atendida = Mascota::new("Fido".to_string(), 3, TipoAnimal::Perro, dueño_mascota_atendida);
    assert!(vet.registro_atenciones[0].mascota.es_igual(&mascota_atendida));
}

#[test]
fn test_buscar_atencion() {
    let mut vet = Veterinaria::new("Vet1".to_string(), "Dirección 123".to_string(), 1);
    let dueño = Dueño::new("Juan".to_string(), "Calle Falsa 123".to_string(), 5551234);
    let mascota = Mascota::new("Fido".to_string(), 3, TipoAnimal::Perro, dueño);
    let atencion = Atencion::new(mascota, "Diagnóstico".to_string(), "Tratamiento".to_string(), None);

    vet.registrar_atencion(atencion);

    let resultado = vet.buscar_atencion("Fido", "Juan", 5551234);
    assert!(resultado.is_some());
    assert!(resultado.unwrap().mascota.nombre == "Fido");
}

#[test]
fn test_modificar_diagnostico() {
    let mut vet = Veterinaria::new("Vet1".to_string(), "Dirección 123".to_string(), 1);
    let dueño = Dueño::new("Juan".to_string(), "Calle Falsa 123".to_string(), 5551234);
    let mascota = Mascota::new("Fido".to_string(), 3, TipoAnimal::Perro, dueño);
    let atencion = Atencion::new(mascota, "Diagnóstico".to_string(), "Tratamiento".to_string(), None);

    vet.registrar_atencion(atencion);
    let nuevo_diagnostico = "Nuevo Diagnóstico".to_string();

    let dueño_mascota_a_modificar = Dueño::new("Juan".to_string(), "Calle Falsa 123".to_string(), 5551234);
    let mascota_a_modificar = Mascota::new("Fido".to_string(), 3, TipoAnimal::Perro, dueño_mascota_a_modificar);
    let resultado = vet.modificar_diagnostico(&mascota_a_modificar, nuevo_diagnostico.clone());
    assert!(resultado);
    assert_eq!(vet.registro_atenciones[0].diagnostico, nuevo_diagnostico);
}

#[test]
fn test_modificar_fecha() {
    let mut vet = Veterinaria::new("Vet1".to_string(), "Dirección 123".to_string(), 1);
    let dueño = Dueño::new("Juan".to_string(), "Calle Falsa 123".to_string(), 5551234);
    let mascota = Mascota::new("Fido".to_string(), 3, TipoAnimal::Perro, dueño);
    let atencion = Atencion::new(mascota, "Diagnóstico".to_string(), "Tratamiento".to_string(), None);

    vet.registrar_atencion(atencion);
    let nueva_fecha = Some(Fecha::new(2024, 6, 10));

    let dueño_mascota_a_modificar = Dueño::new("Juan".to_string(), "Calle Falsa 123".to_string(), 5551234);
    let mascota_a_modificar_fecha = Mascota::new("Fido".to_string(), 3, TipoAnimal::Perro, dueño_mascota_a_modificar);
    let resultado = vet.modificar_fecha(&mascota_a_modificar_fecha, nueva_fecha);
    assert!(resultado);
    let fecha_modificada = Some(Fecha::new(2024, 6, 10));
    assert_eq!(vet.registro_atenciones[0].prox_visita, fecha_modificada);
}

#[test]
fn test_eliminar_atencion() {
    let mut vet = Veterinaria::new("Vet1".to_string(), "Dirección 123".to_string(), 1);
    let dueño = Dueño::new("Juan".to_string(), "Calle Falsa 123".to_string(), 5551234);
    let mascota = Mascota::new("Fido".to_string(), 3, TipoAnimal::Perro, dueño);
    let atencion = Atencion::new(mascota, "Diagnóstico".to_string(), "Tratamiento".to_string(), None);

    vet.registrar_atencion(atencion); 
    let dueño_mascota_a_eliminar = Dueño::new("Juan".to_string(), "Calle Falsa 123".to_string(), 5551234);
    let mascota_a_eliminar = Mascota::new("Fido".to_string(), 3, TipoAnimal::Perro, dueño_mascota_a_eliminar);
    let eliminada = vet.eliminar_atencion(&mascota_a_eliminar);

    assert!(eliminada.is_some());
    assert_eq!(vet.registro_atenciones.len(), 0);
    assert!(eliminada.unwrap().mascota.es_igual(&mascota_a_eliminar));
}