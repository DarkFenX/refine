pub(in crate::sol::api) use a_ship_xt::get_a_ship_xt;
pub(in crate::sol::api) use iter_proj::iter_projectee_keys;
pub use mutation::{
    AddMutationError, AttrMutateRawError, EffectiveMutation, EffectiveMutationMut, FullMAttr, FullMAttrIter,
    FullMAttrMut, GetRawMAttrError, IncompleteMutation, IncompleteMutationMut, Mutation, MutationMut, RawMAttr,
    RawMAttrIter, RawMAttrMut,
};
pub(in crate::sol::api) use ranged_proj::iter_ranged_projs;
pub use ranged_proj::{AddRangedProjError, GetRangedProjError, RangedProj, RangedProjIter, RangedProjMut};

mod a_ship_xt;
mod iter_proj;
mod mutation;
mod ranged_proj;
