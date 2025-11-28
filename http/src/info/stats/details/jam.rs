#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HStatOutgoingJam {
    chance: rc::AttrVal,
    uptime: rc::AttrVal,
}
impl From<rc::stats::StatOutgoingJam> for HStatOutgoingJam {
    fn from(core_stat: rc::stats::StatOutgoingJam) -> Self {
        Self {
            chance: core_stat.chance,
            uptime: core_stat.uptime,
        }
    }
}
