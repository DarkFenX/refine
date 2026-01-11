use crate::num::PValue;

#[derive(Copy, Clone)]
pub struct StatHp {
    pub shield: StatHpLayer,
    pub armor: StatHpLayer,
    pub hull: StatHpLayer,
}

#[derive(Copy, Clone)]
pub struct StatHpLayer {
    pub buffer: PValue,
    pub ancil_local: PValue,
    pub ancil_remote: PValue,
}
