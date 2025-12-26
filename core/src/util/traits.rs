use ordered_float::OrderedFloat;

pub(crate) trait Named {
    fn get_name() -> &'static str;
}

pub(crate) trait GetId<T> {
    fn get_id(&self) -> T;
}

pub(crate) trait Limit {
    fn limit(&mut self, limit: OrderedFloat<f64>);
}
