use itertools::Itertools;

pub(crate) fn into_opt<T: Into<U>, U>(v: Option<T>) -> Option<U> {
    v.map(Into::into)
}

pub(crate) fn into_vec<T: Into<U>, U>(v: Vec<T>) -> Vec<U> {
    v.into_iter().map_into().collect()
}
