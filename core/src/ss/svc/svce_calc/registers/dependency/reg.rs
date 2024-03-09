use crate::{util::KeyedStorage2L, EAttrId, SsItemId};

use super::attr_spec::AttrSpec;

// Intended to hold direct dependencies between attributes, which are not covered by regular
// modifiers
pub(in crate::ss::svc::svce_calc) struct DependencyRegister {
    data: KeyedStorage2L<SsItemId, EAttrId, AttrSpec>,
}
impl DependencyRegister {
    pub(in crate::ss::svc::svce_calc) fn new() -> Self {
        Self {
            data: KeyedStorage2L::new(),
        }
    }
    // Query methods
    // Modification methods
}
