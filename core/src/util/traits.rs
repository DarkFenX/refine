pub(crate) trait LibNamed {
    fn lib_get_name() -> &'static str;
}

pub(crate) trait LibGetId<T> {
    fn lib_get_id(&self) -> T;
}

pub(crate) const trait LibDefault {
    fn lib_default() -> Self;
}

pub(crate) trait LibConvertExtend<X, R> {
    fn lib_convert_extend(self, xt: X) -> R;
}

pub(crate) trait LibMax<Rhs = Self> {
    fn lib_max(self, rhs: Rhs) -> Self;
}

pub(crate) trait LibIncrement {
    fn lib_increment(&mut self);
}
