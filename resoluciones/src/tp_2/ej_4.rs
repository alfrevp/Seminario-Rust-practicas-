pub fn cantidad_impares (nums: [i32;6]) -> i32 {
    let mut cant = 0;
    for i in nums {
        if i % 2 != 0 {
            cant += 1;
        }        
    }
    cant
}