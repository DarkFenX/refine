#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HStatJamApplied {
    chance: rc::AttrVal,
    uptime: rc::AttrVal,
}
impl From<rc::stats::StatJamApplied> for HStatJamApplied {
    fn from(core_stat: rc::stats::StatJamApplied) -> Self {
        Self {
            chance: core_stat.chance,
            uptime: core_stat.uptime,
        }
    }
}
