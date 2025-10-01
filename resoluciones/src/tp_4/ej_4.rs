use std::collections::HashMap;


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Producto<'a> {
    nombre: &'a str,
    categoria: &'a str,
    precio_base: u32, // para evitar tener que implementar manualmente hash o eq para el f64
    descuento: Option<u32>,
}

impl<'a> Producto<'a> {
    fn new(nombre:&'a str, categoria:&'a str, precio_base:u32, descuento:Option<u32>) -> Producto<'a> {
        Producto{nombre, categoria, precio_base, descuento}
    }
}

struct Vendedor<'a> {
    nombre: &'a str,
    apellido: &'a str,
    direccion: &'a str,
    dni: u32,
    legajo: u32,
    antigüedad: u8,
    salario: f64,
}
impl<'a> Vendedor<'a> {
    fn new(nombre:&'a str, apellido:&'a str, direccion:&'a str, dni:u32, legajo:u32, antigüedad:u8, salario:f64) -> Vendedor<'a>{
        Vendedor{nombre, apellido, direccion, dni, legajo, antigüedad, salario}
    }
}

#[derive(Clone)]
struct Cliente<'a> {
    nombre: &'a str,
    apellido: &'a str,
    direccion: &'a str,
    dni: u32,
    mail: Option<&'a str>,
}
impl<'a> Cliente<'a>{
    fn new(nombre:&'a str, apellido:&'a str, direccion:&'a str, dni:u32, mail:Option<&'a str>) -> Cliente<'a>{
        Cliente {nombre, apellido, direccion, dni, mail}
    }
}

enum MedioDePago {
    Credito,
    Debito,
    Transferencia,
    Efectivo,
}

struct Venta<'a>{
    fecha: &'a str,
    cliente: Cliente<'a>,
    vendedor: Vendedor<'a>,
    productos: HashMap<Producto<'a>, u8>,
    medio_pago: MedioDePago,
}
impl <'a> Venta<'a>{
    fn new(fecha:&'a str, cliente:Cliente<'a>, vendedor:Vendedor<'a>, medio_pago:MedioDePago, productos:HashMap<Producto<'a>,u8>) -> Venta<'a> {
        Venta {fecha, cliente, vendedor, medio_pago, productos,}
    }

    fn calcular_precio_final(&self, sistema_ventas:&SistemaVentas<'a>, descuento_newsletter:u8) -> f64 {
        let mut total = 0.0;
        for (producto, cantidad) in &self.productos {
            let mut precio = producto.precio_base as f64 * *cantidad as f64;
            if let Some(descuento) = sistema_ventas.descuentos_categorias.get(producto.categoria) {
                let descuento_decimal = *descuento as f64 / 100.0;
                precio *= 1.0 - descuento_decimal;
            }
            total += precio;
        }
        if self.cliente.mail.is_some() {
            let descuento_newsletter_decimal = descuento_newsletter as f64 / 100.0;
            total -= total * descuento_newsletter_decimal;
        }
        total
    }
}

struct SistemaVentas<'a> {
    historial_ventas: Vec<Venta<'a>>,
    descuentos_categorias: HashMap<&'a str, u8>,
}
impl<'a> SistemaVentas<'a>{
    fn new(descuentos_categorias:HashMap<&'a str, u8>) -> SistemaVentas<'a>{
        SistemaVentas{historial_ventas: Vec::new(), descuentos_categorias}
    }

    fn agregar_venta_al_historial(&mut self, venta:Venta<'a>) {
        self.historial_ventas.push(venta);
    }

    fn reporte_ventas_por_categoria(&self) -> HashMap<&str, usize> {
        let mut ventas_por_categoria: HashMap<&str,usize> = HashMap::new();
        for venta in &self.historial_ventas {
            for (prod,cant) in &venta.productos {
                *ventas_por_categoria.entry(prod.categoria).or_insert(0) += *cant as usize;
            }
        }
        ventas_por_categoria
    }

    fn reporte_ventas_por_vendedor(&self) -> HashMap<&str, usize> {
        let mut ventas_por_vendedor: HashMap<&str, usize> = HashMap::new();
        for venta in &self.historial_ventas {
            *ventas_por_vendedor.entry(&venta.vendedor.nombre).or_insert(0) += 1;
        }
        ventas_por_vendedor
    }
}


#[test]
fn  test_cargar_venta() {
    let mut sistema_ventas = SistemaVentas::new(HashMap::new());
    let cliente = Cliente::new("fulano", "detal", "callefalsa", 16485678, None);
    let vendedor = Vendedor::new("juan", "perez", "algunlado", 87654321, 1234, 5, 300000.00);
    let producto = Producto { nombre: "ProdA", categoria: "Cat 1", precio_base: 100, descuento: None };
        
    let mut productos = HashMap::new();
    productos.insert(producto, 2);
    let venta = Venta::new("2023-06-05", cliente, vendedor, MedioDePago::Efectivo, productos);

    assert_eq!(sistema_ventas.historial_ventas.len(), 0);        
    sistema_ventas.agregar_venta_al_historial(venta);
    assert_eq!(sistema_ventas.historial_ventas.len(), 1);

}

#[test]
fn test_reporte_ventas_por_categoria() {
    let mut sistema_ventas = SistemaVentas::new(HashMap::new());
    let cliente = Cliente::new("fulano", "detal", "callefalsa", 16485678, None);
    let vendedor = Vendedor::new("juan", "perez", "algunlado", 87654321, 1234, 5, 300000.00);
    let producto1 = Producto { nombre: "ProdA", categoria: "Cat 1", precio_base: 100, descuento: None };
    let producto2 = Producto { nombre: "ProdB", categoria: "Cat 2", precio_base: 150, descuento: None };
        
    let mut productos = HashMap::new();
    productos.insert(producto1, 2);
    productos.insert(producto2, 1);
    let venta = Venta::new("2023-06-05", cliente, vendedor, MedioDePago::Efectivo, productos);
    sistema_ventas.agregar_venta_al_historial(venta);

    let reporte_categoria = sistema_ventas.reporte_ventas_por_categoria();
    assert_eq!(reporte_categoria.get("Cat 1"), Some(&2));
    assert_eq!(reporte_categoria.get("Cat 2"), Some(&1));
    assert_eq!(reporte_categoria.get("Cat 3"), None); // No se han vendido productos en esta categoría
}

#[test]
    fn test_reporte_ventas_por_vendedor() {
        let mut sistema_ventas = SistemaVentas::new(HashMap::new());
        let cliente = Cliente::new("fulano", "detal", "callefalsa", 16485678, None);
        let vendedor1 = Vendedor::new("juan", "perez", "algunlado", 87654321, 1234, 5, 300000.00);
        let vendedor2 = Vendedor::new("maria", "gomez", "otrolado", 98765432, 5678, 3, 250000.00);
        let producto = Producto { nombre: "ProdA", categoria: "Cat 1", precio_base: 100, descuento: None };
            
        let mut productos = HashMap::new();
        productos.insert(producto, 2);
        let venta1 = Venta::new("2023-06-05", cliente.clone(), vendedor1, MedioDePago::Efectivo, productos.clone());
        let venta2 = Venta::new("2023-06-06", cliente, vendedor2, MedioDePago::Efectivo, productos);
        sistema_ventas.agregar_venta_al_historial(venta1);
        sistema_ventas.agregar_venta_al_historial(venta2);

        let reporte_vendedor = sistema_ventas.reporte_ventas_por_vendedor();
        assert_eq!(reporte_vendedor.get("juan"), Some(&1));
        assert_eq!(reporte_vendedor.get("maria"), Some(&1));
        assert_eq!(reporte_vendedor.get("pedro"), None); // No se han vendido productos por este vendedor
}



#[test]
fn test_calcular_precio_final_sin_descuentos() {
    let cliente = Cliente::new("fulano", "detal", "callefalsa", 16485678, None);
    let vendedor = Vendedor::new("juan", "perez", "algunlado", 87654321, 1234, 5, 300000.00);
    let producto = Producto::new("ProdA", "Cat 1", 100, None);

    let mut productos = HashMap::new();
    productos.insert(producto.clone(), 2);
    
    let venta = Venta::new("2023-06-05", cliente, vendedor, MedioDePago::Efectivo, productos);
    assert_eq!(venta.calcular_precio_final(&SistemaVentas::new(HashMap::new()), 0), 200.0);
}

#[test]
fn test_calcular_precio_final_con_descuentos() {
    let cliente = Cliente::new("fulano", "detal", "callefalsa", 16485678, None);
    let vendedor = Vendedor::new("juan", "perez", "algunlado", 87654321, 1234, 5, 300000.00);
    let producto1 = Producto::new("ProdA", "Cat 1", 100, Some(10));
    let producto2 = Producto::new("ProdB", "Cat 2", 150, Some(20));

    let mut descuentos_categorias = HashMap::new();
    descuentos_categorias.insert("Cat 1", 10);
    descuentos_categorias.insert("Cat 2", 20);

    let mut productos = HashMap::new();
    productos.insert(producto1.clone(), 2);
    productos.insert(producto2.clone(), 1);

    let venta = Venta::new("2023-06-05", cliente, vendedor, MedioDePago::Efectivo, productos);
    assert_eq!(venta.calcular_precio_final(&SistemaVentas::new(descuentos_categorias), 0), 300.0);
}

#[test]
fn test_calcular_precio_final_con_descuento_newsletter() {
    let cliente = Cliente::new("fulano", "detal", "callefalsa", 16485678, Some("fulano@example.com"));
    let vendedor = Vendedor::new("juan", "perez", "algunlado", 87654321, 1234, 5, 300000.00);
    let producto = Producto::new("ProdA", "Cat 1", 100, None);

    let mut productos = HashMap::new();
    productos.insert(producto.clone(), 2);

    let venta = Venta::new("2023-06-05", cliente, vendedor, MedioDePago::Efectivo, productos);
    assert_eq!(venta.calcular_precio_final(&SistemaVentas::new(HashMap::new()), 10), 180.0);
}

#[test]
fn test_calcular_precio_final_con_descuentos_y_newsletter() {
    let cliente = Cliente::new("fulano", "detal", "callefalsa", 16485678, Some("fulano@example.com"));
    let vendedor = Vendedor::new("juan", "perez", "algunlado", 87654321, 1234, 5, 300000.00);
    let producto1 = Producto::new("ProdA", "Cat 1", 100, Some(10));
    let producto2 = Producto::new("ProdB", "Cat 2", 150, Some(20));

    let mut descuentos_categorias = HashMap::new();
    descuentos_categorias.insert("Cat 1", 10);
    descuentos_categorias.insert("Cat 2", 20);

    let mut productos = HashMap::new();
    productos.insert(producto1.clone(), 2);
    productos.insert(producto2.clone(), 1);

    let venta = Venta::new("2023-06-05", cliente, vendedor, MedioDePago::Efectivo, productos);
    assert_eq!(venta.calcular_precio_final(&SistemaVentas::new(descuentos_categorias), 10), 270.0);
}
