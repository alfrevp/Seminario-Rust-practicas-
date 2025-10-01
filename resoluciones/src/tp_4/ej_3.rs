use std::{clone, collections::HashMap};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum TipoSuscripcion {
    Basic,
    Clasic,
    Super,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum MedioDepago {
    Efectivo,
    MercadoPago,
    TarjetaDeCred,
    Transferencia,
    Cripto,
}
#[derive(Clone, PartialEq, Debug)]
struct Suscripcion {
    tipo: TipoSuscripcion,
    costo_mensual: f64,
    duracion: u8,
    fecha_inicio: String,
}

impl Suscripcion {
    fn new(tipo:TipoSuscripcion, costo_mensual:f64, duracion:u8, fecha_inicio:String) -> Suscripcion {
        Suscripcion {
            tipo,
            costo_mensual,
            duracion,
            fecha_inicio,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Usuario {
    id: u32,
    suscripcion: Option<Suscripcion>,
    medio_pago: Option<MedioDepago>,
}

impl Usuario {
    fn new(id:u32, suscripcion:Suscripcion, medio_pago:MedioDepago) -> Usuario {
        Usuario {
            id,
            suscripcion: Some(suscripcion),
            medio_pago: Some(medio_pago),
        }
    }

    fn upgrade_suscripcion(&mut self) {
        if let Some(ref mut suscripcion) = self.suscripcion {
            suscripcion.tipo = match suscripcion.tipo {
                TipoSuscripcion::Basic => TipoSuscripcion::Clasic,
                TipoSuscripcion::Clasic => TipoSuscripcion::Super,
                TipoSuscripcion::Super => TipoSuscripcion::Super,
            }
        }
    }

    fn downgrade_suscripcion(&mut self) {
        if let Some(ref mut suscripcion) = self.suscripcion {
            suscripcion.tipo = match suscripcion.tipo {
                TipoSuscripcion::Super => TipoSuscripcion::Clasic,
                TipoSuscripcion::Clasic => TipoSuscripcion::Basic,
                TipoSuscripcion::Basic => {
                    self.suscripcion = None;
                    return;
                }
            }
        }
    }

    fn cancelar_suscripcion(&mut self) {
        self.suscripcion = None;
        self.medio_pago = None;
    }
}

struct StreamingRust {
    usuarios: HashMap<u32, Usuario>,
    historial_suscripciones: HashMap<TipoSuscripcion, u32>, // contador historico para tipo de suscripcion
    historial_medio_pago: HashMap<MedioDepago, u32>, // contador historico para medio de pago mas elegido
}

impl StreamingRust {
    fn new() -> StreamingRust {
        StreamingRust {
            usuarios: HashMap::new(),
            historial_suscripciones: HashMap::new(),
            historial_medio_pago: HashMap::new(),
        }
    }

    fn agregar_usuario(&mut self, usuario: Usuario) {
        if let Some(suscripcion) = &usuario.suscripcion {
            self.historial_suscripciones.entry(suscripcion.tipo.clone()).and_modify(|s| *s += 1).or_insert(1);
        }
        if let Some(medio_pago) = &usuario.medio_pago {
            self.historial_medio_pago.entry(medio_pago.clone()).and_modify(|m| *m += 1).or_insert(1);
        }
        self.usuarios.insert(usuario.id, usuario);
    }

    fn pago_mas_utilizado_activos(&self) -> Option<MedioDepago> { // DEBERIA VER LA FORMA DE SOLUCIONAR PARA CUANDO HAY MAS DE UN MAXIMO EN LA ESTRUCTURA
        let mut contador: HashMap<MedioDepago, u32> = HashMap::new();
        for usuario in self.usuarios.values() {
            if let Some(medio) = &usuario.medio_pago {
                    contador.entry(medio.clone()).and_modify(|m| *m += 1).or_insert(1);
                }
            }
        contador.into_iter().max_by_key(|&(_, cant)| cant).map(|(m, _)| m)
    }

    fn suscripcion_mas_contratada_activos(&self) -> Option<TipoSuscripcion> {
        let mut contador: HashMap<TipoSuscripcion, u32> = HashMap::new();
        for usuario in self.usuarios.values() {
            if let Some(suscripcion) = &usuario.suscripcion {
                contador.entry(suscripcion.tipo.clone()).and_modify(|s| *s += 1).or_insert(1);
            }
        }
        contador.into_iter().max_by_key(|&(_, cant)| cant).map(|(s, _)| s)
    }

    fn pago_mas_utilizado_historico(&self) -> Option<MedioDepago> {
        self.historial_medio_pago.clone().into_iter().max_by_key(|&(_, cant)| cant).map(|(m, _)| m)
    }

    fn suscripcion_mas_contratada_historica(&self) -> Option<TipoSuscripcion> {
        self.historial_suscripciones.clone().into_iter().max_by_key(|&(_, cant)| cant).map(|(s, _)| s)
    }

}


#[test]
fn test_usuario_upgrade_suscripcion() {
    let mut usuario = Usuario::new(1, Suscripcion::new(TipoSuscripcion::Basic, 10.0, 1, "2024-06-01".to_string()), MedioDepago::TarjetaDeCred);
    usuario.upgrade_suscripcion();
    assert_eq!(usuario.suscripcion.unwrap().tipo, TipoSuscripcion::Clasic);
}

#[test]
fn test_usuario_downgrade_suscripcion() {
    let mut usuario = Usuario::new(1, Suscripcion::new(TipoSuscripcion::Clasic, 10.0, 1, "2024-06-01".to_string()), MedioDepago::TarjetaDeCred);
    usuario.downgrade_suscripcion();
    assert_eq!(usuario.suscripcion.unwrap().tipo, TipoSuscripcion::Basic);
}

#[test]
fn test_usuario_cancelar_suscripcion() {
    let mut usuario = Usuario::new(1, Suscripcion::new(TipoSuscripcion::Clasic, 10.0, 1, "2024-06-01".to_string()), MedioDepago::TarjetaDeCred);
    usuario.cancelar_suscripcion();
    assert!(usuario.suscripcion.is_none());
    assert!(usuario.medio_pago.is_none());
}

#[test]
fn test_streaming_rust_agregar_usuario() {
    let mut streaming_rust = StreamingRust::new();
    let suscripcion = Suscripcion::new(TipoSuscripcion::Clasic, 10.0, 1, "2024-06-01".to_string());
    let usuario = Usuario::new(1, suscripcion.clone(), MedioDepago::TarjetaDeCred);
    streaming_rust.agregar_usuario(usuario.clone());

    assert_eq!(streaming_rust.usuarios.get(&1), Some(&usuario));
    assert_eq!(streaming_rust.historial_suscripciones.get(&TipoSuscripcion::Clasic), Some(&1));
    assert_eq!(streaming_rust.historial_medio_pago.get(&MedioDepago::TarjetaDeCred), Some(&1));
}

#[test]
fn test_streaming_rust_pago_mas_utilizado_activos() { 
    let mut streaming_rust = StreamingRust::new();
    let suscripcion = Suscripcion::new(TipoSuscripcion::Clasic, 10.0, 1, "2024-06-01".to_string());
    let usuario1 = Usuario::new(1, suscripcion.clone(), MedioDepago::TarjetaDeCred);
    let usuario2 = Usuario::new(2, suscripcion.clone(), MedioDepago::MercadoPago);
    let usuario3 = Usuario::new(3, suscripcion.clone(), MedioDepago::TarjetaDeCred);

    streaming_rust.agregar_usuario(usuario1);
    streaming_rust.agregar_usuario(usuario2);
    streaming_rust.agregar_usuario(usuario3);

    assert_eq!(streaming_rust.pago_mas_utilizado_activos(), Some(MedioDepago::TarjetaDeCred));
}

#[test]
fn test_streaming_rust_suscripcion_mas_contratada_activos() { // FALLA
    let mut streaming_rust = StreamingRust::new();
    let suscripcion1 = Suscripcion::new(TipoSuscripcion::Clasic, 10.0, 1, "2024-06-01".to_string());
    let suscripcion2 = Suscripcion::new(TipoSuscripcion::Super, 15.0, 1, "2024-06-01".to_string());
    let usuario1 = Usuario::new(1, suscripcion1.clone(), MedioDepago::TarjetaDeCred);
    let usuario2 = Usuario::new(2, suscripcion2.clone(), MedioDepago::MercadoPago);
    let usuario3 = Usuario::new(3, suscripcion1.clone(), MedioDepago::Cripto);

    streaming_rust.agregar_usuario(usuario1);
    streaming_rust.agregar_usuario(usuario2);
    streaming_rust.agregar_usuario(usuario3);

    assert_eq!(streaming_rust.suscripcion_mas_contratada_activos(), Some(TipoSuscripcion::Clasic));
}

#[test]
fn test_streaming_rust_pago_mas_utilizado_historico() {
    let mut streaming_rust = StreamingRust::new();
    streaming_rust.historial_medio_pago.insert(MedioDepago::TarjetaDeCred, 10);
    streaming_rust.historial_medio_pago.insert(MedioDepago::MercadoPago, 5);
    streaming_rust.historial_medio_pago.insert(MedioDepago::Transferencia, 8);

    assert_eq!(streaming_rust.pago_mas_utilizado_historico(), Some(MedioDepago::TarjetaDeCred));
}

#[test]
fn test_streaming_rust_suscripcion_mas_contratada_historica() {
    let mut streaming_rust = StreamingRust::new();
    streaming_rust.historial_suscripciones.insert(TipoSuscripcion::Basic, 20);
    streaming_rust.historial_suscripciones.insert(TipoSuscripcion::Clasic, 15);
    streaming_rust.historial_suscripciones.insert(TipoSuscripcion::Super, 25);

    assert_eq!(streaming_rust.suscripcion_mas_contratada_historica(), Some(TipoSuscripcion::Super));
}