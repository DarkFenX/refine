use crate::{
    ad::AAttrId,
    api::{AttrId, EffectiveMutationMut, IncompleteMutationMut, MutationMut, RawMAttrMut},
    err::basic::ItemMAttrNotFoundError,
    num::UnitInterval,
    sol::SolarSystem,
    ud::UItemId,
};

impl<'a> MutationMut<'a> {
    /// Mutate an attribute with a roll quality.
    ///
    /// Accepts roll of any attribute, even if it is not defined by item mutator. In this case, roll
    /// will be stored, and its effect won't be applied.
    pub fn mutate_raw(&mut self, attr_id: AttrId, roll: UnitInterval) -> Result<RawMAttrMut<'_>, AttrMutateRawError> {
        match self {
            Self::Effective(effective_mutation) => effective_mutation.mutate_raw(attr_id, roll),
            Self::Incomplete(incomplete_mutation) => incomplete_mutation.mutate_raw(attr_id, roll),
        }
    }
}

impl<'a> EffectiveMutationMut<'a> {
    /// Mutate an attribute with a roll quality.
    ///
    /// Accepts roll of any attribute, even if it is not defined by item mutator. In this case, roll
    /// will be stored, and its effect won't be applied.
    pub fn mutate_raw(&mut self, attr_id: AttrId, roll: UnitInterval) -> Result<RawMAttrMut<'_>, AttrMutateRawError> {
        mutate_raw(self.sol, self.item_uid, attr_id.into_aid(), roll)
    }
}

impl<'a> IncompleteMutationMut<'a> {
    /// Mutate an attribute with a roll quality.
    ///
    /// Accepts roll of any attribute, even if it is not defined by item mutator. In this case, roll
    /// will be stored, and its effect won't be applied.
    pub fn mutate_raw(&mut self, attr_id: AttrId, roll: UnitInterval) -> Result<RawMAttrMut<'_>, AttrMutateRawError> {
        mutate_raw(self.sol, self.item_uid, attr_id.into_aid(), roll)
    }
}

fn mutate_raw(
    sol: &mut SolarSystem,
    item_uid: UItemId,
    attr_aid: AAttrId,
    roll: UnitInterval,
) -> Result<RawMAttrMut<'_>, AttrMutateRawError> {
    match sol
        .u_data
        .items
        .get(item_uid)
        .get_mutation_data()
        .unwrap()
        .get_attr_rolls()
        .get(&attr_aid)
    {
        Some(_) => Err(ItemMAttrNotFoundError {
            item_id: sol.u_data.items.xid_by_iid(item_uid),
            attr_id: AttrId::from_aid(attr_aid),
        }
        .into()),
        None => {
            let mut raw_mattr = RawMAttrMut::new(sol, item_uid, attr_aid);
            raw_mattr.set_roll(roll);
            Ok(raw_mattr)
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AttrMutateRawError {
    #[error("{0}")]
    AlreadyMutated(#[from] ItemMAttrNotFoundError),
}
