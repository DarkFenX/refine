use crate::{ad::generator::rels::KeyPart, ed::EItem};

/// Foreign key to an item.
///
/// Items which do not meet the condition count as missing for the key. Cleanup does not restore
/// rows whose required keys are missing.
#[derive(Copy, Clone)]
pub(in crate::ad::generator) struct ItemFk {
    pub(in crate::ad::generator) key: KeyPart,
    pub(in crate::ad::generator) cond: ItemFkCond,
    // If True, entity carrying this FK needs alive target to be restored itself
    pub(in crate::ad::generator) required: bool,
}
impl ItemFk {
    pub(in crate::ad::generator) fn any(key: KeyPart) -> Self {
        Self {
            key,
            cond: ItemFkCond::Any,
            required: false,
        }
    }
    pub(in crate::ad::generator) fn dynamic(key: KeyPart) -> Self {
        Self {
            key,
            cond: ItemFkCond::Dynamic,
            required: false,
        }
    }
    pub(in crate::ad::generator) fn non_dynamic(key: KeyPart) -> Self {
        Self {
            key,
            cond: ItemFkCond::NonDynamic,
            required: false,
        }
    }
    pub(in crate::ad::generator) fn required(mut self) -> Self {
        self.required = true;
        self
    }
}

#[derive(Copy, Clone)]
pub(in crate::ad::generator) enum ItemFkCond {
    Any,
    Dynamic,
    NonDynamic,
}
impl ItemFkCond {
    fn bit(self) -> u8 {
        match self {
            Self::Any => 1 << 0,
            Self::Dynamic => 1 << 1,
            Self::NonDynamic => 1 << 2,
        }
    }
}

/// Set of item foreign key conditions.
#[derive(Copy, Clone, Default)]
pub(in crate::ad::generator) struct ItemFkConds(u8);
impl ItemFkConds {
    /// Conditions the item meets.
    pub(in crate::ad::generator) fn met_by(e_item: &EItem) -> Self {
        let mut conds = Self::default();
        conds.insert(ItemFkCond::Any);
        conds.insert(match e_item.is_dynamic {
            true => ItemFkCond::Dynamic,
            false => ItemFkCond::NonDynamic,
        });
        conds
    }
    pub(in crate::ad::generator) fn insert(&mut self, cond: ItemFkCond) {
        self.0 |= cond.bit();
    }
    pub(in crate::ad::generator) fn contains(self, cond: ItemFkCond) -> bool {
        self.0 & cond.bit() != 0
    }
    pub(in crate::ad::generator) fn intersects(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }
}
