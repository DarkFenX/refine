use serde_tuple::Serialize_tuple;

#[derive(Serialize_tuple)]
pub(crate) struct HStatSlot {
    used: u32,
    total: Option<u32>,
}
impl From<rc::stats::StatSlot> for HStatSlot {
    fn from(core_stat: rc::stats::StatSlot) -> Self {
        Self {
            used: core_stat.used.into_u32(),
            total: core_stat.total.map(|v| v.into_u32()),
        }
    }
}
