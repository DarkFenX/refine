use std::hash::{BuildHasher, Hash};

use crate::{
    sol::svc::{
        AttrSpec,
        calc::{Context, CtxModifier},
    },
    util::{HMapHSet, MapSet},
};

pub(super) fn add_ctx_modifier<K, H1, H2>(
    main_storage: &mut MapSet<K, CtxModifier, H1, H2>,
    key: K,
    ctx_modifier: CtxModifier,
    attr_spec_storage: &mut HMapHSet<AttrSpec, CtxModifier>,
) where
    K: Eq + Hash,
    H1: BuildHasher + Default,
    H2: BuildHasher + Default,
{
    main_storage.add_entry(key, ctx_modifier);
    if let Some(affector_a_attr_id) = ctx_modifier.raw.get_affector_a_attr_id() {
        let affector_spec = AttrSpec {
            item_id: ctx_modifier.raw.affector_item_id,
            a_attr_id: affector_a_attr_id,
        };
        attr_spec_storage.add_entry(affector_spec, ctx_modifier);
    }
    if let (Some(resist_a_attr_id), Context::Item(ctx_item_id)) = (ctx_modifier.raw.resist_a_attr_id, ctx_modifier.ctx)
    {
        let affector_spec = AttrSpec {
            item_id: ctx_item_id,
            a_attr_id: resist_a_attr_id,
        };
        attr_spec_storage.add_entry(affector_spec, ctx_modifier);
    }
    if let Some(optimal_a_attr_id) = ctx_modifier.raw.optimal_a_attr_id {
        let affector_spec = AttrSpec {
            item_id: ctx_modifier.raw.affector_item_id,
            a_attr_id: optimal_a_attr_id,
        };
        attr_spec_storage.add_entry(affector_spec, ctx_modifier);
    }
    if let Some(falloff_a_attr_id) = ctx_modifier.raw.falloff_a_attr_id {
        let affector_spec = AttrSpec {
            item_id: ctx_modifier.raw.affector_item_id,
            a_attr_id: falloff_a_attr_id,
        };
        attr_spec_storage.add_entry(affector_spec, ctx_modifier);
    }
}

pub(super) fn remove_ctx_modifier<K, H1, H2>(
    main_storage: &mut MapSet<K, CtxModifier, H1, H2>,
    key: &K,
    ctx_modifier: &CtxModifier,
    attr_spec_storage: &mut HMapHSet<AttrSpec, CtxModifier>,
) where
    K: Eq + Hash,
    H1: BuildHasher + Default,
    H2: BuildHasher + Default,
{
    main_storage.remove_entry(key, ctx_modifier);
    if let Some(affector_a_attr_id) = ctx_modifier.raw.get_affector_a_attr_id() {
        let affector_spec = AttrSpec {
            item_id: ctx_modifier.raw.affector_item_id,
            a_attr_id: affector_a_attr_id,
        };
        attr_spec_storage.remove_entry(&affector_spec, ctx_modifier);
    }
    if let (Some(resist_a_attr_id), Context::Item(ctx_item_id)) = (ctx_modifier.raw.resist_a_attr_id, ctx_modifier.ctx)
    {
        let affector_spec = AttrSpec {
            item_id: ctx_item_id,
            a_attr_id: resist_a_attr_id,
        };
        attr_spec_storage.remove_entry(&affector_spec, ctx_modifier);
    }
    if let Some(optimal_a_attr_id) = ctx_modifier.raw.optimal_a_attr_id {
        let affector_spec = AttrSpec {
            item_id: ctx_modifier.raw.affector_item_id,
            a_attr_id: optimal_a_attr_id,
        };
        attr_spec_storage.remove_entry(&affector_spec, ctx_modifier);
    }
    if let Some(falloff_a_attr_id) = ctx_modifier.raw.falloff_a_attr_id {
        let affector_spec = AttrSpec {
            item_id: ctx_modifier.raw.affector_item_id,
            a_attr_id: falloff_a_attr_id,
        };
        attr_spec_storage.remove_entry(&affector_spec, ctx_modifier);
    }
}
