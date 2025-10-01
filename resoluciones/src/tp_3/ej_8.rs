
enum Genero {
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros,
}
impl Genero {
    fn es_igual(&self, otro_genero:&Genero) -> bool {
        match (self, otro_genero) {
            (Genero::Jazz, Genero::Jazz) => true,
            (Genero::Pop, Genero::Pop) => true,
            (Genero::Rap, Genero::Rap) => true,
            (Genero::Rock, Genero::Rock) => true,
            (Genero::Otros, Genero::Otros) => true,
            _ => false,
        }
    }
}


struct Cancion {
    titulo: String,
    artista: String,
    genero: Genero,
}

impl Cancion {
    fn new(titulo:String, artista:String, genero:Genero) -> Cancion {
        Cancion {titulo, artista, genero}
    }

    fn es_igual(&self, otra_cancion:&Cancion) -> bool {
        self.titulo == otra_cancion.titulo &&
        self.artista == otra_cancion.artista &&
        self.genero_comparable() == otra_cancion.genero_comparable()
    }

    fn genero_comparable(&self) -> &str {
        match self.genero {
            Genero::Jazz => "jazz",
            Genero::Pop => "pop",
            Genero::Rap => "rap",
            Genero::Rock => "rock",
            Genero::Otros => "otros"
        }
    }
}

struct Playlist {
    lista_canciones: Vec<Cancion>,
    nombre: String,
}

impl Playlist {
    fn new(nombre: String) -> Self {
        Playlist{lista_canciones: Vec::new(), nombre}}

    fn agregar_cancion(&mut self, cancion: Cancion) {
        self.lista_canciones.push(cancion);
    }

    fn eliminar_cancion(&mut self, cancion:&Cancion) {
        if let Some(pos) = self.lista_canciones.iter().position(|c| c.es_igual(&cancion)) {
            self.lista_canciones.remove(pos);
        } 
    }

    fn mover_cancion(&mut self, cancion:&Cancion, nueva_pos: usize) {
        if let Some(pos_actual) = self.lista_canciones.iter().position(|c| c.es_igual(&cancion)) {
            if nueva_pos < self.lista_canciones.len() {
                let cancion = self.lista_canciones.remove(pos_actual);
                self.lista_canciones.insert(nueva_pos, cancion);
            }
        }
    }

    fn buscar_cancion_por_nombre(&self, nombre: String) -> Option<&Cancion> {
        self.lista_canciones.iter().find(|elem| elem.titulo == nombre)
    }
        

    fn obtener_por_genero(&self, genero:Genero) -> Vec<&Cancion> {
        self.lista_canciones.iter().filter(|elem| elem.genero.es_igual(&genero)).collect()
    }

    fn obtener_por_artista(&self, artista: String) -> Vec<&Cancion> {
        self.lista_canciones.iter().filter(|elem| elem.artista == artista).collect()
    }

    fn modificar_titulo(&mut self, nuevo_nombre: String) {
        self.nombre = nuevo_nombre;
    }

    fn vaciar_playlist(&mut self) {
        self.lista_canciones.clear();
    }

}


#[test]
fn test_agregar_cancion() {
    let mut playlist = Playlist::new(String::from("Mi Playlist"));
    let cancion = Cancion::new("Titulo1".to_string(), "Artista1".to_string(), Genero::Rock);
    playlist.agregar_cancion(cancion);
    assert_eq!(playlist.lista_canciones.len(), 1);
    assert_eq!(playlist.lista_canciones[0].titulo, "Titulo1");
}

#[test]
fn test_eliminar_cancion() {
    let mut playlist = Playlist::new(String::from("Mi Playlist"));
    let cancion = Cancion::new("Titulo1".to_string(), "Artista1".to_string(), Genero::Rock);
    playlist.agregar_cancion(cancion);
    playlist.eliminar_cancion(&Cancion::new("Titulo1".to_string(), "Artista1".to_string(), Genero::Rock));
       assert_eq!(playlist.lista_canciones.len(), 0);
}

#[test]
fn test_mover_cancion() {
    let mut playlist = Playlist::new(String::from("Mi Playlist"));
    let cancion1 = Cancion::new("Titulo1".to_string(), "Artista1".to_string(), Genero::Rock);
    let cancion2 = Cancion::new("Titulo2".to_string(), "Artista2".to_string(), Genero::Rap);
    playlist.agregar_cancion(cancion1);
    playlist.agregar_cancion(cancion2);
    playlist.mover_cancion(&Cancion::new("Titulo2".to_string(), "Artista2".to_string(), Genero::Rap), 1);
    assert_eq!(playlist.lista_canciones[1].titulo, "Titulo2");
}

#[test]
fn test_buscar_cancion_por_nombre() {
    let mut playlist = Playlist::new(String::from("Mi Playlist"));
    let cancion = Cancion::new("Titulo1".to_string(), "Artista1".to_string(), Genero::Rock);
    playlist.agregar_cancion(cancion);
    let cancion_a_encontrar = Cancion::new("Titulo1".to_string(), "Artista1".to_string(), Genero::Rock);
    assert!(playlist.buscar_cancion_por_nombre("Titulo1".to_string()).unwrap().es_igual(&cancion_a_encontrar));
}

#[test]
fn test_obtener_por_genero() {
    let mut playlist = Playlist::new(String::from("Mi Playlist"));
    let cancion1 = Cancion::new("Titulo1".to_string(), "Artista1".to_string(), Genero::Rock);
    let cancion2 = Cancion::new("Titulo2".to_string(), "Artista2".to_string(), Genero::Otros);
    
    playlist.agregar_cancion(cancion1.into());
    playlist.agregar_cancion(cancion2.into());

    let canciones_rock = playlist.obtener_por_genero(Genero::Rock);
    assert_eq!(canciones_rock.len(), 1);
    assert_eq!(canciones_rock[0].titulo, "Titulo1");
}

#[test]
fn test_obtener_por_artista() {
    let mut playlist = Playlist::new(String::from("Mi Playlist"));
    let cancion1 = Cancion::new("Titulo1".to_string(), "Artista1".to_string(), Genero::Rock); 
    let cancion2 = Cancion::new("Titulo2".to_string(), "Artista1".to_string(), Genero::Otros);

    playlist.agregar_cancion(cancion1.into());
    playlist.agregar_cancion(cancion2.into());

    let canciones_queen = playlist.obtener_por_artista(String::from("Artista1"));
    assert_eq!(canciones_queen.len(), 2);
    assert_eq!(canciones_queen[0].titulo, "Titulo1");
    assert_eq!(canciones_queen[1].titulo, "Titulo2");
}

#[test]
fn test_modificar_titulo() {
    let mut playlist = Playlist::new("MiPlaylist".to_string());
    playlist.modificar_titulo("Nueva Playlist".to_string());
    assert_eq!(playlist.nombre, String::from("Nueva Playlist"));
}

#[test]
fn test_vaciar_playlist() {
    let mut playlist = Playlist::new(String::from("Mi Playlist"));
    playlist.agregar_cancion(Cancion::new("Titulo1".to_string(), "Artista1".to_string(), Genero::Rock));
    assert_eq!(playlist.lista_canciones.len(), 1) ;
    playlist.vaciar_playlist();
    assert_eq!(playlist.lista_canciones.len(), 0);
}