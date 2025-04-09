pub(crate) trait Named {
    fn get_name() -> &'static str;
}

pub(crate) trait GetId<T> {
    fn get_id(&self) -> T;
}
