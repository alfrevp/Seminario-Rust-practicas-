pub fn longitud_de_cadenas(cadenas:[String;3]) -> [usize;3] {
    let mut longitudes = [0; 3];
    for i in 0..cadenas.len() {
        longitudes[i] = cadenas[i].len();
    }
    longitudes
}