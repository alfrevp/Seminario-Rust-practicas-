use std::{collections::HashMap, u32};

use super::ej_3::Fecha;

enum Genero {
    Novela,
    Infantil,
    Tecnico,
    Otros
}

struct Libro<'a> {
    isbn: u32,
    titulo: &'a str,
    autor: &'a str,
    paginas: u32,
    genero: Genero,
}
impl<'a> Libro<'a> {
    fn new(isbn:u32, titulo:&'a str, autor:&'a str, paginas:u32, genero:Genero) -> Libro<'a> {
        Libro {isbn, titulo, autor, paginas, genero}
    }
}

#[derive(PartialEq, Clone)]
struct Cliente<'a> {
    nombre: &'a str,
    tel: u32,
    mail: &'a str,
}
impl<'a> Cliente<'a> {
    fn new(nombre:&'a str, tel:u32, mail:&'a str) -> Cliente<'a> {
        Cliente {nombre, tel, mail}
    }
}

#[derive(Clone)]
enum EstadoPrestamo {
    Devuelto,
    EnPrestamo,
}
#[derive(Clone)]
struct Prestamo<'a> {
    isbn_libro: u32,
    cliente: Cliente<'a>,
    vencimiento: Fecha,
    fecha_devolucion: Option<Fecha>,
    estado: EstadoPrestamo,
}
impl<'a> Prestamo<'a> {
    fn new(isbn_libro:u32, cliente:Cliente<'a>, vencimiento:Fecha, fecha_devolucion:Option<Fecha>) -> Prestamo<'a> {
        Prestamo {isbn_libro, cliente, vencimiento, fecha_devolucion, estado:EstadoPrestamo::EnPrestamo}
    }

    fn fue_devuelto(&self) -> bool {
        match &self.estado {
            EstadoPrestamo::Devuelto => true,
            EstadoPrestamo::EnPrestamo => false,
        }
    }
}

struct Biblioteca<'a> {
    nombre: &'a str,
    direccion: &'a str,
    stock_libros: HashMap<u32, u8>,
    historial_prestamos: Vec<Prestamo<'a>>,
}
impl<'a> Biblioteca<'a> {
    fn new(nombre:&'a str, direccion:&'a str) -> Biblioteca<'a> {
        Biblioteca {nombre, direccion, stock_libros: HashMap::new(), historial_prestamos: Vec::new()}
    }

    fn incrementar_copias(&mut self, isbn:u32) {
        self.stock_libros.entry(isbn).and_modify(|cant| *cant += 1).or_insert(1);
    }

    fn decrementar_copias(&mut self, isbn:u32) -> bool {
        if let Some(cant) = self.stock_libros.get_mut(&isbn) {
            if *cant > 0 {
                *cant -= 1;
                true
            } else {
                false
            }
        } else {
            false   
        }
    }

    fn obtener_cantidad_copias(&self, isbn:u32) -> Option<u8> {
        if let Some(cant) = self.stock_libros.get(&isbn).copied() {
            return Some(cant)
        }
        None
    }

    fn contar_prestamos(&self, cliente:&Cliente<'a>) -> u8 {
        let mut cant = 0;
        for p in &self.historial_prestamos {
            if p.cliente == *cliente {
                if !p.fue_devuelto() {
                    cant += 1;
                }
            }
        }
        cant
    }

    fn realizar_prestamo(&mut self, cliente:&Cliente<'a>, isbn:u32, fecha:Fecha) -> bool {
        let prestamos = self.contar_prestamos(cliente);
        if prestamos <= 5 {
            if self.obtener_cantidad_copias(isbn).unwrap_or(0) > 0 {
                let prestamo = Prestamo::new(isbn, cliente.clone(), fecha, None);
                self.decrementar_copias(isbn);
                self.agregar_prestamo_historial(&prestamo);
                return true
            }
        }
        false
    }

    fn agregar_prestamo_historial(&mut self, prestamo:&Prestamo<'a>) {
        self.historial_prestamos.push(prestamo.clone());
    }

    fn ver_prestamos_por_vencer(&self, dias:u8, mut fecha_actual: Fecha) -> Vec<Prestamo> {
        let mut lista = Vec::new();
        for p in &self.historial_prestamos {
            if fecha_actual.sumar_dias(dias as u32).es_mayor(&p.vencimiento) {
                lista.push(p.clone());
            }
        }
        lista
    }

    fn ver_prestamos_vencidos(&self, fecha_actual: Fecha) -> Vec<Prestamo> {
        let mut lista = Vec::new();
        for p in &self.historial_prestamos {
            if fecha_actual.es_mayor(&p.vencimiento) {
                lista.push(p.clone());
            }
        }
        lista
    }

    fn buscar_prestamo<'b>(&'b mut self, libro:&Libro, cliente:&Cliente) -> Option<&'b mut Prestamo<'a>> {
        if let Some(pres) = self.historial_prestamos.iter_mut().find(|p| p.isbn_libro == libro.isbn && p.cliente == *cliente) {
            Some(pres)
        } else {
            None
        }
    }

    fn devolver_libro(&mut self, libro:&Libro, cliente:&Cliente, dia:u32, mes:u32, anio:u32) -> bool {
        if let Some(pres) = self.buscar_prestamo(libro, cliente) {
            pres.estado = EstadoPrestamo::Devuelto;
            pres.fecha_devolucion = Some(Fecha::new(dia, mes, anio));
            let isbn = pres.isbn_libro;
            self.incrementar_copias(isbn);
            return true
        }
        false
    }

}

#[test]
fn test_incrementar_copias() {
    let mut biblioteca = Biblioteca::new("Biblioteca1", "Calle 123");
    biblioteca.incrementar_copias(1234);
    assert_eq!(biblioteca.obtener_cantidad_copias(1234), Some(1));
}

#[test]
fn test_decrementar_copias() {
    let mut biblioteca = Biblioteca::new("Biblioteca1", "Calle 123");
    biblioteca.incrementar_copias(1234);
    assert!(biblioteca.decrementar_copias(1234));
    assert_eq!(biblioteca.obtener_cantidad_copias(1234), Some(0));
}

#[test]
fn test_realizar_prestamo() {
    let mut biblioteca = Biblioteca::new("Biblioteca1", "Calle 123");
    let cliente = Cliente::new("juan perez", 15547852, "juanmail@hotmail.com");
    let fecha = Fecha::new(15, 5, 2024);
    biblioteca.incrementar_copias(1234);
    assert!(biblioteca.realizar_prestamo(&cliente, 1234, fecha.clone()));
    assert_eq!(biblioteca.obtener_cantidad_copias(1234), Some(0));
}

#[test]
fn test_devolver_libro() {
    let mut biblioteca = Biblioteca::new("Biblioteca1", "Calle 123");
    let cliente = Cliente::new("juan perez", 15547852, "juanmail@hotmail.com");
    let fecha_prestamo = Fecha::new(15, 5, 2024);
    let libro = Libro::new(1234, "Un libro", "Algun Autor", 240, Genero::Tecnico);
    biblioteca.incrementar_copias(libro.isbn);
    biblioteca.realizar_prestamo(&cliente, libro.isbn, fecha_prestamo.clone());
    assert!(biblioteca.devolver_libro(&libro, &cliente, 10, 1, 2024));
    assert_eq!(biblioteca.obtener_cantidad_copias(libro.isbn), Some(1));
}

#[test]
fn test_ver_prestamos_por_vencer() {
    let mut biblioteca = Biblioteca::new("Biblioteca1", "Calle 123");
    let cliente = Cliente::new("juan perez", 15547852, "juanmail@hotmail.com");
    let fecha_prestamo = Fecha::new(15, 5, 2024);
    let fecha_actual = Fecha::new(16, 5, 2024);
    let libro = Libro::new(1234, "Un libro", "Algun Autor", 240, Genero::Tecnico);
    biblioteca.incrementar_copias(libro.isbn);
    biblioteca.realizar_prestamo(&cliente, libro.isbn, fecha_prestamo.clone());
    let prestamos_por_vencer = biblioteca.ver_prestamos_por_vencer(3, fecha_actual.clone());
    assert_eq!(prestamos_por_vencer.len(), 1);
}

#[test]
fn test_ver_prestamos_vencidos() {
    let mut biblioteca = Biblioteca::new("Biblioteca1", "Calle 123");
    let cliente = Cliente::new("juan perez", 15547852, "juanmail@hotmail.com");
    let fecha_prestamo = Fecha::new(15, 5, 2024);
    let fecha_actual = Fecha::new(16, 6, 2024);
    let libro = Libro::new(1234, "Un libro", "Algun Autor", 240, Genero::Tecnico);
    biblioteca.incrementar_copias(libro.isbn);
    biblioteca.realizar_prestamo(&cliente, libro.isbn, fecha_prestamo.clone());
    let prestamos_vencidos = biblioteca.ver_prestamos_vencidos(fecha_actual.clone());
    assert_eq!(prestamos_vencidos.len(), 1);
}