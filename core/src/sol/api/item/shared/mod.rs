pub(in crate::sol::api) use iter_proj::iter_projectee_keys;
pub use mutation::{
    AddMutationError, AttrMutateRawError, EffectiveMutation, EffectiveMutationMut, FullMAttr, FullMAttrIter,
    FullMAttrMut, GetRawMAttrError, IncompleteMutation, IncompleteMutationMut, Mutation, MutationMut, RawMAttr,
    RawMAttrIter, RawMAttrMut,
};
pub(in crate::sol::api) use ranged_proj::iter_ranged_projs;
pub use ranged_proj::{AddProjError, GetRangedProjError, RangedProj, RangedProjIter, RangedProjMut};
pub(in crate::sol::api) use ship_axt::get_ship_axt;

mod iter_proj;
mod mutation;
mod ranged_proj;
mod ship_axt;
