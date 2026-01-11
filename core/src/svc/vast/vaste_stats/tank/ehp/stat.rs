use crate::num::PValue;

#[derive(Copy, Clone)]
pub struct StatEhp {
    pub shield: Option<StatEhpLayer>,
    pub armor: Option<StatEhpLayer>,
    pub hull: Option<StatEhpLayer>,
}

#[derive(Copy, Clone)]
pub struct StatEhpLayer {
    pub buffer: PValue,
    pub ancil_local: PValue,
    pub ancil_remote: PValue,
    pub mult: PValue,
}
