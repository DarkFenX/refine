use std::collections::HashMap;

pub(crate) enum ItemInfo {
    Id(reefast::ReeId),
    Detailed(ItemInfoDetailed),
}

pub(crate) struct ItemInfoDetailed {
    pub(crate) id: reefast::ReeInt,
    pub(crate) original_attrs: HashMap<reefast::ReeInt, reefast::ReeFloat>,
    pub(crate) modified_attrs: HashMap<reefast::ReeInt, reefast::ReeFloat>,
}
impl ItemInfoDetailed {
    pub(crate) fn new(id: reefast::ReeInt) -> Self {
        Self {
            id,
            original_attrs: HashMap::new(),
            modified_attrs: HashMap::new(),
        }
    }
}
