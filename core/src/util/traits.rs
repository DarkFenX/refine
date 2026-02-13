pub(crate) trait LibNamed {
    fn lib_get_name() -> &'static str;
}

pub(crate) trait LibGetId<T> {
    fn lib_get_id(&self) -> T;
}

pub(crate) const trait LibDefault {
    fn lib_default() -> Self;
}

pub(crate) trait LibMax<Rhs = Self> {
    fn lib_max(self, rhs: Rhs) -> Self;
}

pub(crate) trait LibIncrement {
    fn lib_increment(&mut self);
}

pub(crate) trait LibConverter<I, O> {
    fn lib_convert(&mut self, input: I) -> O;
}
