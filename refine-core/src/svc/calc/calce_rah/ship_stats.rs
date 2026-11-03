use crate::{PValue, misc::DmgKinds};

pub(super) struct RahShipStats {
    pub(super) resos: DmgKinds<PValue>,
    pub(super) breacher_reso: PValue,
    pub(super) total_hp: PValue,
}
