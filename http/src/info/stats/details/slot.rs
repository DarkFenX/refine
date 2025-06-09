#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HStatSlot {
    used: rc::Count,
    total: Option<rc::Count>,
}
impl From<rc::stats::StatSlot> for HStatSlot {
    fn from(core_stat: rc::stats::StatSlot) -> Self {
        Self {
            used: core_stat.used,
            total: core_stat.total,
        }
    }
}
