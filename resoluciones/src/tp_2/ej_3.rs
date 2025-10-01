pub fn suma_pares (nums: [i32; 6]) -> i32 {
    let mut suma = 0;
    for i in nums {
        if i % 2 == 0 {
            suma += i;
        }
    }
    suma
}