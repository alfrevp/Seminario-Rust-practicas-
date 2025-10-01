pub fn reemplazar_pares(nums: &mut [i32;6]) {
    for i in 0..nums.len() {
        if nums[i] % 2 == 0 {
            nums[i] = -1;
        }
    }
}