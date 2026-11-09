use crate::{
    ad::AItemId,
    rd::{RAttrId, REffect, REffectId, REffectResist, RItem, RMutasPregen, RcItem, RcMuta},
    util::{PDense, RMap, RSet},
};

/// Moves remote resistance attribute reference from items to effects whenever possible.
///
/// This can happen only if all items which use this effect reference the same attribute, or do not
/// specify it at all. When effect carries remote resistance ID, it makes calculations easier.
pub(in crate::rd::data) fn fill_effect_remote_resists(
    effects: &mut PDense<REffectId, REffect>,
    items: &RMap<AItemId, RcItem>,
    mutas: &RMap<AItemId, RcMuta>,
    muta_pregen: &RMutasPregen,
) {
    let mut effect_resists = RMap::new();
    let mutated_type_aids: RSet<_> = mutas
        .values()
        .flat_map(|muta| muta.item_map.values().copied())
        .collect();
    // Fill data for non-mutated items
    for (type_aid, r_item) in items.iter() {
        if !mutated_type_aids.contains(type_aid) {
            add_item_remote_resist(&mut effect_resists, r_item, r_item.flex_data.remote_resist_attr_rid);
        }
    }
    // Fill data for mutated items
    for r_muta in mutas.values() {
        for (base_type_aid, mutated_type_aid) in r_muta.item_map.iter() {
            if let Some(r_item) = items.get(mutated_type_aid)
                && let Some(rifd) = muta_pregen.get_flex_data(*base_type_aid, r_muta.id)
            {
                add_item_remote_resist(&mut effect_resists, r_item, rifd.remote_resist_attr_rid);
            }
        }
    }
    // Transfer full matches to effects which rely on remote resist attr ID
    for (&effect_rid, effect_resist) in effect_resists.iter() {
        if let EffectItemsResist::Same(attr_rid) = effect_resist
            && let Some(proj_mod) = &mut effects.get_mut(effect_rid).unwrap().proj_mod
            && let Some(REffectResist::RemoteResistance) = proj_mod.resist
        {
            proj_mod.resist = attr_rid.map(REffectResist::Attr);
        }
    }
}

fn add_item_remote_resist(
    effect_resists: &mut RMap<REffectId, EffectItemsResist>,
    r_item: &RItem,
    attr_rid: Option<RAttrId>,
) {
    for &effect_rid in r_item.base.effect_rids.iter() {
        effect_resists
            .entry(effect_rid)
            .and_modify(|effect_resist| {
                if *effect_resist != EffectItemsResist::Same(attr_rid) {
                    *effect_resist = EffectItemsResist::Different;
                }
            })
            .or_insert(EffectItemsResist::Same(attr_rid));
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum EffectItemsResist {
    Same(Option<RAttrId>),
    Different,
}
