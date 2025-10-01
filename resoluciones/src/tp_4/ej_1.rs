pub fn contar_primos(vector: &Vec<i32>) -> usize {
    vector.iter().filter(|x| x.es_primo()).count()
}

trait EsPrimo {
    fn es_primo(&self) -> bool;
}

impl EsPrimo for i32 {
    fn es_primo(&self) -> bool {
        let mut divisores = 0;
        for i in 1..=*self {
            if self % i == 0 {
                divisores += 1;
            }
            if divisores > 2 {
                break
            }
        }
        divisores == 2
    }
}

#[test]
fn test_contar_primo() {
    let vector = vec![1, 7, 5, 6, 9];
    let cantidad_primos = contar_primos(&vector);
    assert_eq!(cantidad_primos, 2);
}