#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HStatDmg {
    em: rc::AttrVal,
    thermal: rc::AttrVal,
    kinetic: rc::AttrVal,
    explosive: rc::AttrVal,
}
impl From<rc::stats::DmgKinds<rc::AttrVal>> for HStatDmg {
    fn from(core_stat: rc::stats::DmgKinds<rc::AttrVal>) -> Self {
        Self {
            em: core_stat.em,
            thermal: core_stat.thermal,
            kinetic: core_stat.kinetic,
            explosive: core_stat.explosive,
        }
    }
}
