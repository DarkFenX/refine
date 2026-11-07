pub(in crate::ud::item) use base::UItemBase;
pub(crate) use effects_resolver::UEffectUpdates;
use effects_resolver::UItemEffectsResolver;
pub(in crate::ud::item) use mutable::UItemBaseMutable;
pub(crate) use mutable::{
    ItemMutatedError, ItemMutationData, MutatorInplaceError, UAttrMutationRequest, UItemMutationRequest,
};

mod base;
mod effects_resolver;
mod mutable;
