use crate::{
    misc::DmgKinds,
    num::{PValue, Value},
};

pub(super) struct RahShipStats {
    pub(super) resos: DmgKinds<Value>,
    pub(super) total_hp: PValue,
}
