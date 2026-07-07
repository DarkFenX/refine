use crate::{misc::DmgKinds, num::PValue};

pub(super) struct RahShipStats {
    pub(super) resos: DmgKinds<PValue>,
    pub(super) breacher_reso: PValue,
    pub(super) total_hp: PValue,
}
