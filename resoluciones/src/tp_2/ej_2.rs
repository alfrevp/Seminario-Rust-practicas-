pub fn es_primo (num: i32) -> bool {
    let mut divisores = 0;
    for i in 1..=num {
        if num % i == 0 {
            divisores += 1;
        }
        if divisores > 2 {
            break
        }
    }
    divisores == 2
}