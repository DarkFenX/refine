pub(crate) fn vec_push_opt<T>(vec: &mut Vec<T>, opt: Option<T>) {
    if let Some(v) = opt {
        vec.push(v);
    };
}

pub(crate) fn f64_to_u32(value: f64) -> u32 {
    value.clamp(u32::MIN as f64, u32::MAX as f64).round() as u32
}
pub(crate) fn f64_to_i32(value: f64) -> i32 {
    value.clamp(i32::MIN as f64, i32::MAX as f64).round() as i32
}
