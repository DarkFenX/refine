#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info) struct HAdjustableCount {
    current: rc::Count,
    max: rc::Count,
    overridden: bool,
}
impl From<rc::AdjustableCount> for HAdjustableCount {
    fn from(core_adj_count: rc::AdjustableCount) -> Self {
        Self {
            current: core_adj_count.current,
            max: core_adj_count.max,
            overridden: core_adj_count.overridden,
        }
    }
}
