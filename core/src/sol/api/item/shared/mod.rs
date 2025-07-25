pub(in crate::sol::api) use iter_proj::iter_projectee_keys;
pub use mutation::{
    AddMutationError, AttrMutateRawError, EffectiveMutation, EffectiveMutationMut, FullMAttr, FullMAttrIter,
    FullMAttrMut, GetRawMAttrError, IncompleteMutation, IncompleteMutationMut, Mutation, MutationMut, RawMAttr,
    RawMAttrIter, RawMAttrMut,
};
pub(in crate::sol::api) use r_ship_axt::get_r_ship_axt;
pub(in crate::sol::api) use ranged_proj::iter_ranged_projs;
pub use ranged_proj::{AddRangedProjError, GetRangedProjError, RangedProj, RangedProjIter, RangedProjMut};

mod iter_proj;
mod mutation;
mod r_ship_axt;
mod ranged_proj;
