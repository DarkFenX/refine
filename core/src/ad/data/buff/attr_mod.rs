use crate::{consts::ModAfeeFilter, defs::EAttrId, util::Named};

/// A buff-specific attribute modifier.
///
/// Unlike effect modifiers, buff modifiers carry less data, since some of it resides on its parent
/// buff and some on the entity applying the buff.
#[derive(Debug)]
pub struct ABuffAttrMod {
    /// Defines an affectee filter, that is a filter which defines which items will be affected.
    pub afee_filter: ModAfeeFilter,
    /// Refers an attribute, whose value will be affected on the affectee.
    pub afee_attr_id: EAttrId,
}
impl ABuffAttrMod {
    /// Make a new buff-specific attribute modifier out of passed data.
    pub(crate) fn new(afee_filter: ModAfeeFilter, afee_attr_id: EAttrId) -> Self {
        Self {
            afee_filter,
            afee_attr_id,
        }
    }
}
impl Named for ABuffAttrMod {
    fn get_name() -> &'static str {
        "ABuffAttrMod"
    }
}
