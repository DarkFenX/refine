use std::hash::{Hash, Hasher};

use crate::{num::U32NonMax, ud::UItemId, util::DenseId};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Main ID used in calc
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Template and affector item for modifiers which come from templates, generated storage slot for
/// the rest.
#[derive(Copy, Clone)]
pub(in crate::svc::calc) struct RawModId {
    index: U32NonMax,
    // No item means that the index is a storage slot
    item_uid: Option<UItemId>,
}
impl RawModId {
    pub(super) fn from_template(template_id: RawModTemplateId, item_uid: UItemId) -> Self {
        Self {
            index: template_id.0,
            item_uid: Some(item_uid),
        }
    }
    pub(in crate::svc::calc) fn from_slot(slot: RawModSlot) -> Self {
        Self {
            index: slot.0,
            item_uid: None,
        }
    }
    pub(in crate::svc::calc) fn is_template(self) -> bool {
        self.item_uid.is_some()
    }
    pub(in crate::svc::calc) fn kind(self) -> RawModIdKind {
        match self.item_uid {
            Some(item_uid) => RawModIdKind::Template(RawModTemplateId(self.index), item_uid),
            None => RawModIdKind::Stored(RawModSlot(self.index)),
        }
    }
    pub(in crate::svc::calc) fn expect_slot(self) -> RawModSlot {
        // Should be used for IDs known to be generated
        match self.kind() {
            RawModIdKind::Stored(slot) => slot,
            RawModIdKind::Template(..) => unreachable!("raw modifier is expected to be stored"),
        }
    }
    fn into_raw(self) -> u64 {
        let index = self.index.into_u32() as u64;
        let item = self.item_uid.map_or(UItemId::NICHE, UItemId::into_raw) as u64;
        index << 32 | item
    }
}
impl Eq for RawModId {}
impl PartialEq for RawModId {
    fn eq(&self, other: &Self) -> bool {
        // One comparison, derived impl does up to 3
        self.into_raw() == other.into_raw()
    }
}
impl Hash for RawModId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // One hash write, derived impl does up to 3
        state.write_u64(self.into_raw());
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Secondary IDs
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc::calc) enum RawModIdKind {
    Template(RawModTemplateId, UItemId),
    Stored(RawModSlot),
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub(in crate::svc::calc) struct RawModTemplateId(U32NonMax);
impl DenseId for RawModTemplateId {
    fn new(index: usize) -> Self {
        Self(U32NonMax::from_usize(index))
    }
    fn index(self) -> usize {
        self.0.into_usize()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc::calc) struct RawModSlot(U32NonMax);
impl DenseId for RawModSlot {
    fn new(index: usize) -> Self {
        Self(U32NonMax::from_usize(index))
    }
    fn index(self) -> usize {
        self.0.into_usize()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "dev")]
mod debug {
    use super::*;
    use crate::{dbg::DebugResult, svc::calc::registers::RawModStorage, ud::UData};

    impl RawModId {
        pub(in crate::svc::calc) fn consistency_check(&self, u_data: &UData, rmods: &RawModStorage) -> DebugResult {
            let Some(rmod) = rmods.try_get_rmod(u_data.r_data.get_rmod_templates(), *self) else {
                return Err(Default::default());
            };
            rmod.consistency_check(u_data)?;
            // Nothing is stored for modifiers which come from templates, so ones left in registers
            // after their effect stopped are only seen here
            if let RawModIdKind::Template(..) = self.kind()
                && !u_data
                    .items
                    .get(rmod.affector_espec.item_uid)
                    .get_reffs()
                    .is_some_and(|reffs| reffs.contains(rmod.affector_espec.effect_rid))
            {
                return Err(Default::default());
            }
            Ok(())
        }
    }
}
