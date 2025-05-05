use crate::{
    ad,
    err::basic::ItemMAttrNotFoundError,
    sol::{
        AttrId, ItemKey, SolarSystem,
        api::{EffectiveMutationMut, IncompleteMutationMut, MutationMut, RawMAttrMut},
    },
};

impl<'a> MutationMut<'a> {
    pub fn mutate_raw(&mut self, attr_id: AttrId) -> Result<RawMAttrMut, AttrMutateRawError> {
        match self {
            Self::Effective(effective_mutation) => effective_mutation.mutate_raw(attr_id),
            Self::Incomplete(incomplete_mutation) => incomplete_mutation.mutate_raw(attr_id),
        }
    }
}

impl<'a> EffectiveMutationMut<'a> {
    pub fn mutate_raw(&mut self, attr_id: AttrId) -> Result<RawMAttrMut, AttrMutateRawError> {
        mutate_raw(self.sol, self.item_key, attr_id)
    }
}

impl<'a> IncompleteMutationMut<'a> {
    pub fn mutate_raw(&mut self, attr_id: AttrId) -> Result<RawMAttrMut, AttrMutateRawError> {
        mutate_raw(self.sol, self.item_key, attr_id)
    }
}

fn mutate_raw(
    sol: &mut SolarSystem,
    item_key: ItemKey,
    a_attr_id: ad::AAttrId,
) -> Result<RawMAttrMut, AttrMutateRawError> {
    match sol
        .uad
        .items
        .get(item_key)
        .get_mutation_data()
        .unwrap()
        .get_attr_rolls()
        .get(&a_attr_id)
    {
        Some(_) => Err(ItemMAttrNotFoundError {
            item_id: sol.uad.items.id_by_key(item_key),
            attr_id: a_attr_id,
        }
        .into()),
        None => Ok(RawMAttrMut::new(sol, item_key, a_attr_id)),
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AttrMutateRawError {
    #[error("{0}")]
    AlreadyMutated(#[from] ItemMAttrNotFoundError),
}
