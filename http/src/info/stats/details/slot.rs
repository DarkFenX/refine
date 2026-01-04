#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HStatSlot {
    used: rc::DefCount,
    total: Option<rc::DefCount>,
}
impl From<rc::stats::StatSlot> for HStatSlot {
    fn from(core_stat: rc::stats::StatSlot) -> Self {
        Self {
            used: core_stat.used,
            total: core_stat.total,
        }
    }
}
