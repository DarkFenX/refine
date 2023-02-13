use std::ptr;

pub(crate) fn vec_push_opt<T>(vec: &mut Vec<T>, opt: Option<T>) {
    if let Some(v) = opt {
        vec.push(v);
    };
}

pub(crate) fn are_same<T>(first: &T, second: &T) -> bool {
    ptr::eq(first, second)
}
