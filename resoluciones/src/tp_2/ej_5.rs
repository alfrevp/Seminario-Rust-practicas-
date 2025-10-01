pub fn duplicar_valores(nums:[f32;6]) -> [f32; 6] {
    let mut nue = [0.0; 6];
    for i in 0..nums.len() {
        nue[i] = nums[i]*2.0;
    }
    nue
}