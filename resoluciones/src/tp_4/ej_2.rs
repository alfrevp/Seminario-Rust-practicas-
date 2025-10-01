#[derive(Debug, PartialEq, Clone, Copy)]
struct Persona<'a>{
    nombre:&'a str,
    apellido:&'a str,
    direccion:&'a str,
    ciudad:&'a str,
    salario:f64,
    edad:u8,
}

impl<'a> Persona<'a>{
    fn new(nombre:&'a str, apellido:&'a str, direccion:&'a str, ciudad:&'a str, salario:f64, edad:u8) -> Persona<'a>{
        Persona {nombre, apellido, direccion, ciudad, salario, edad}
    }
}


fn filtrar_por_salario_minimo(personas: Vec<Persona>, salario: f64) -> Vec<Persona> {  // esta funcion toma el ownership del vector recibido poor parametro
    personas.iter().filter(|x| x.salario > salario).cloned().collect()
} 

fn filtrar_por_edad_y_ciudad(personas: Vec<Persona>, edad:u8, ciudad:String) -> Vec<Persona> {
    personas.iter().filter(|x| x.ciudad == ciudad && x.edad > edad).cloned().collect()
}

fn viven_todas(personas: &Vec<Persona>, ciudad:&str) -> bool {
    !personas.iter().any(|x| x.ciudad != ciudad)
}

fn vive_alguna(personas: &Vec<Persona>, ciudad:&str) -> bool {
    personas.iter().any(|x| x.ciudad == ciudad)
}

fn existe_persona(personas: &Vec<Persona>, per:&Persona) -> bool {
    personas.contains(per)
}

fn obtener_edades(personas: &Vec<Persona>) -> Vec<u8> {
    personas.iter().map(|x| x.edad).collect()
}

fn obtener_menor_y_mayor_salario(personas: Vec<Persona>) -> (Persona, Persona) {
    let mut menor = personas[0];
    let mut mayor = personas[0];
    for elem in personas.iter().skip(1) {
        if elem.salario < menor.salario || (elem.salario == menor.salario && elem.edad > menor.edad) {
            menor = *elem;
        }
        if elem.salario > mayor.salario || (elem.salario == mayor.salario && elem.edad > mayor.edad) {
            mayor = *elem;
        }
    }
    (menor, mayor)
}

#[test]
fn test_filtrar_salario() {
    let p1 = Persona::new("Juan", "Perez", "Calle 7", "La Plata", 350000.00, 32);
    let p2 = Persona::new("Lucas", "Suarez", "Calle 5", "asd", 300000.00, 54);
    let p3 = Persona::new("Pablo", "Ramirez", "algunlugar", "alguna", 250000.00, 35);
    let personas = vec![p1,p2,p3];
    assert_eq!(filtrar_por_salario_minimo(personas, 299000.00), [p1,p2]);
    let personas2 = vec![p1,p2,p3];
    let vacia = filtrar_por_salario_minimo(personas2, 500000.00);
    assert!(vacia.is_empty());
}

#[test]
fn test_filtrar_edad_ciudad() {
    let p1 = Persona::new("Juan", "Perez", "Calle 7", "La Plata", 350000.00, 32);
    let p2 = Persona::new("Lucas", "Suarez", "Calle 5", "asd", 300000.00, 54);
    let p3 = Persona::new("Pablo", "Ramirez", "algunlugar", "La Plata", 250000.00, 35);
    let personas = vec![p1,p2,p3];
    let filtrada = filtrar_por_edad_y_ciudad(personas, 30, "La Plata".to_string());
    assert_eq!(filtrada.len(), 2);
    assert_eq!(filtrada, [p1,p3]);
}

#[test]
fn test_existen_y_viven() {
    let p1 = Persona::new("Juan", "Perez", "Calle 7", "La Plata", 350000.00, 32);
    let p2 = Persona::new("Lucas", "Suarez", "Calle 5", "asd", 300000.00, 54);
    let p3 = Persona::new("Pablo", "Ramirez", "algunlugar", "La Plata", 250000.00, 35);
    let personas = vec![p1,p2,p3];
    assert_eq!(existe_persona(&personas, &p1), true);
    assert_eq!(viven_todas(&personas, "La Plata"), false);
    assert_eq!(vive_alguna(&personas, "La Plata"), true);
}

#[test]
fn test_obtener_edades() {
    let p1 = Persona::new("Juan", "Perez", "Calle 7", "La Plata", 350000.00, 32);
    let p2 = Persona::new("Lucas", "Suarez", "Calle 5", "asd", 300000.00, 54);
    let p3 = Persona::new("Pablo", "Ramirez", "algunlugar", "La Plata", 250000.00, 35);
    let personas = vec![p1,p2,p3];
    assert_eq!(obtener_edades(&personas), [32,54,35]);
}

#[test]
fn test_salario_min_max() {
    let p1 = Persona::new("Juan", "Perez", "Calle 7", "La Plata", 350000.00, 32);
    let p2 = Persona::new("Lucas", "Suarez", "Calle 5", "asd", 300000.00, 54);
    let p3 = Persona::new("Pablo", "Ramirez", "algunlugar", "La Plata", 250000.00, 35);
    let personas = vec![p1,p2,p3];
    assert_eq!(obtener_menor_y_mayor_salario(personas), (p3,p1));
}