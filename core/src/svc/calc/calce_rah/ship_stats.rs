use crate::{def::AttrVal, misc::DmgKinds};

pub(super) struct RahShipStats {
    pub(super) resos: DmgKinds<AttrVal>,
    pub(super) total_hp: AttrVal,
}
