pub fn min_one(val: i32) -> i32 {
    if val < 1 {
        return 1;
    }
    val
}

pub fn min_one_f(val: f32) -> f32 {
    if val < 0.11f32 {
        return 0.1f32;
    }
    val
}