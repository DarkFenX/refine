#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info) struct HAdjustableCount {
    current: rc::DefCount,
    max: rc::DefCount,
    overridden: bool,
}
impl From<rc::Adjustable> for HAdjustableCount {
    fn from(core_adj_count: rc::Adjustable) -> Self {
        Self {
            current: core_adj_count.current,
            max: core_adj_count.max,
            overridden: core_adj_count.overridden,
        }
    }
}
