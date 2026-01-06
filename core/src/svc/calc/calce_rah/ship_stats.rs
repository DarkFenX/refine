use crate::misc::{DmgKinds, PValue, Value};

pub(super) struct RahShipStats {
    pub(super) resos: DmgKinds<Value>,
    pub(super) total_hp: PValue,
}
