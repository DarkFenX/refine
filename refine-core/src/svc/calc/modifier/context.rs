use std::hash::{Hash, Hasher};

use crate::ud::{UFitId, UItemId};

#[derive(Copy, Clone)]
pub(in crate::svc::calc) struct ModContext {
    fit_uid: Option<UFitId>,
    item_uid: Option<UItemId>,
}
impl ModContext {
    pub(in crate::svc::calc) fn new() -> Self {
        Self {
            fit_uid: None,
            item_uid: None,
        }
    }
    pub(in crate::svc::calc) fn with_item(item_uid: UItemId) -> Self {
        Self {
            fit_uid: None,
            item_uid: Some(item_uid),
        }
    }
    pub(in crate::svc::calc) fn with_fit(fit_uid: UFitId) -> Self {
        Self {
            fit_uid: Some(fit_uid),
            item_uid: None,
        }
    }
    pub(in crate::svc::calc) fn with_fit_item(fit_uid: UFitId, item_uid: UItemId) -> Self {
        Self {
            fit_uid: Some(fit_uid),
            item_uid: Some(item_uid),
        }
    }
    pub(in crate::svc::calc) fn get_fit_uid(self) -> Option<UFitId> {
        self.fit_uid
    }
    pub(in crate::svc::calc) fn get_item_uid(self) -> Option<UItemId> {
        self.item_uid
    }
    fn into_raw(self) -> u64 {
        let fit = self.fit_uid.map_or(UFitId::NICHE, UFitId::into_raw) as u64;
        let item = self.item_uid.map_or(UItemId::NICHE, UItemId::into_raw) as u64;
        fit << 32 | item
    }
}
impl Eq for ModContext {}
impl PartialEq for ModContext {
    fn eq(&self, other: &Self) -> bool {
        // One comparison, derived impl does up to 4
        self.into_raw() == other.into_raw()
    }
}
impl Hash for ModContext {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // One hash write, derived impl does up to 4
        state.write_u64(self.into_raw());
    }
}
