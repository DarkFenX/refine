use super::item_info::ItemInfo;
use crate::{misc::DmgKinds, num::Value};

pub(super) struct ItemData {
    pub(super) info: ItemInfo,
    pub(super) taken_dmg: DmgKinds<Value>,
}
impl ItemData {
    pub(super) fn new(info: ItemInfo) -> Self {
        Self {
            info,
            taken_dmg: DmgKinds {
                em: Value::ZERO,
                thermal: Value::ZERO,
                kinetic: Value::ZERO,
                explosive: Value::ZERO,
            },
        }
    }
}
