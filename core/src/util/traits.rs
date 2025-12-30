pub(crate) trait Named {
    fn get_name() -> &'static str;
}

pub(crate) trait GetId<T> {
    fn get_id(&self) -> T;
}

pub(crate) trait ConvertExtend<X, R> {
    fn convert_extend(self, xt: X) -> R;
}
