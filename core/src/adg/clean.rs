use std::collections::HashSet;

use itertools::Itertools;

use crate::{
    adg::{
        rels::{KeyDb, Pk},
        GData, GSupport,
    },
    consts::{itemcats, itemgrps},
    defs::Amount,
    util::{IntError, IntResult, Named},
};

const MAX_CYCLES: Amount = 100;

pub(in crate::adg) fn clean_unused(alive: &mut GData, g_supp: &GSupport) -> IntResult<()> {
    let mut trash = GData::new();
    trash_all(alive, &mut trash);
    restore_core_items(alive, &mut trash, &g_supp);

    let mut counter = 0;
    let mut changes = true;
    while changes {
        counter += 1;
        if counter > MAX_CYCLES {
            let msg = format!("reached limit of {MAX_CYCLES} cycles during cleanup");
            tracing::error!("{msg}");
            return Err(IntError::new(msg));
        }
        changes = restore_item_data(alive, &mut trash) || restore_fk_tgts(alive, &mut trash, &g_supp);
    }
    cleanup_report(alive, &trash);
    Ok(())
}

fn move_data<T, F>(src_vec: &mut Vec<T>, dst_vec: &mut Vec<T>, filter: F) -> bool
where
    F: FnMut(&mut T) -> bool,
{
    let drained = src_vec.drain_filter(filter).collect_vec();
    let changes = drained.len() > 0;
    dst_vec.extend(drained);
    changes
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Initial preparation functions
////////////////////////////////////////////////////////////////////////////////////////////////////
fn trash_all(alive: &mut GData, trash: &mut GData) {
    move_data(&mut alive.items, &mut trash.items, |_| true);
    move_data(&mut alive.groups, &mut trash.groups, |_| true);
    move_data(&mut alive.attrs, &mut trash.attrs, |_| true);
    move_data(&mut alive.item_attrs, &mut trash.item_attrs, |_| true);
    move_data(&mut alive.effects, &mut trash.effects, |_| true);
    move_data(&mut alive.item_effects, &mut trash.item_effects, |_| true);
    move_data(&mut alive.abils, &mut trash.abils, |_| true);
    move_data(&mut alive.item_abils, &mut trash.item_abils, |_| true);
    move_data(&mut alive.buffs, &mut trash.buffs, |_| true);
    move_data(&mut alive.item_srqs, &mut trash.item_srqs, |_| true);
    move_data(&mut alive.muta_items, &mut trash.muta_items, |_| true);
    move_data(&mut alive.muta_attrs, &mut trash.muta_attrs, |_| true);
}

fn restore_core_items(alive: &mut GData, trash: &mut GData, g_supp: &GSupport) {
    let cats = vec![
        itemcats::CHARGE,
        itemcats::DRONE,
        itemcats::FIGHTER,
        itemcats::IMPLANT,
        itemcats::MODULE,
        itemcats::SHIP,
        itemcats::SKILL,
        itemcats::SUBSYSTEM,
    ];
    let mut grps = vec![itemgrps::CHARACTER, itemgrps::EFFECT_BEACON];
    for (&grp, cat) in g_supp.grp_cat_map.iter() {
        if cats.contains(cat) {
            grps.push(grp);
        }
    }
    move_data(&mut trash.items, &mut alive.items, |v| grps.contains(&v.group_id));
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Cyclic restoration functions
////////////////////////////////////////////////////////////////////////////////////////////////////
fn restore_item_data(alive: &mut GData, trash: &mut GData) -> bool {
    let mut item_ids = HashSet::new();
    for item in alive.items.iter() {
        item_ids.extend(item.get_pk());
    }
    // We need the data which describes our items directly, so FKs are avoided deliberately. For
    // instance, having an item-attribute mapping entry restored just because its value refers some
    // item which is already "alive" is undesired.
    //
    // Extra notes on specific entities:
    // - Mutaplasmid item conversions are restored for input items which are alive
    // - Mutaplasmid attribute modifications are restored for alive mutaplasmids
    move_data(&mut trash.item_attrs, &mut alive.item_attrs, |v| {
        item_ids.contains(&v.item_id)
    }) || move_data(&mut trash.item_effects, &mut alive.item_effects, |v| {
        item_ids.contains(&v.item_id)
    }) || move_data(&mut trash.item_abils, &mut alive.item_abils, |v| {
        item_ids.contains(&v.item_id)
    }) || move_data(&mut trash.item_srqs, &mut alive.item_srqs, |v| {
        item_ids.contains(&v.item_id)
    }) || move_data(&mut trash.muta_items, &mut alive.muta_items, |v| {
        item_ids.contains(&v.in_item_id)
    }) || move_data(&mut trash.muta_attrs, &mut alive.muta_attrs, |v| {
        item_ids.contains(&v.muta_id)
    })
}

fn restore_fk_tgts(alive: &mut GData, trash: &mut GData, g_supp: &GSupport) -> bool {
    let fkdb = KeyDb::new_fkdb(alive, g_supp);
    move_data(&mut trash.items, &mut alive.items, |v| fkdb.items.contains(&v.id))
        || move_data(&mut trash.groups, &mut alive.groups, |v| fkdb.groups.contains(&v.id))
        || move_data(&mut trash.attrs, &mut alive.attrs, |v| fkdb.attrs.contains(&v.id))
        || move_data(&mut trash.effects, &mut alive.effects, |v| fkdb.effects.contains(&v.id))
        || move_data(&mut trash.abils, &mut alive.abils, |v| fkdb.abils.contains(&v.id))
        || move_data(&mut trash.buffs, &mut alive.buffs, |v| fkdb.buffs.contains(&v.id))
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Reporting
////////////////////////////////////////////////////////////////////////////////////////////////////
fn cleanup_report(alive: &GData, trash: &GData) {
    vec_report(&alive.items, &trash.items);
    vec_report(&alive.groups, &trash.groups);
    vec_report(&alive.attrs, &trash.attrs);
    vec_report(&alive.item_attrs, &trash.item_attrs);
    vec_report(&alive.effects, &trash.effects);
    vec_report(&alive.item_effects, &trash.item_effects);
    vec_report(&alive.abils, &trash.abils);
    vec_report(&alive.item_abils, &trash.item_abils);
    vec_report(&alive.buffs, &trash.buffs);
    vec_report(&alive.item_srqs, &trash.item_srqs);
    vec_report(&alive.muta_items, &trash.muta_items);
    vec_report(&alive.muta_attrs, &trash.muta_attrs);
}

fn vec_report<T: Named>(alive: &Vec<T>, trash: &Vec<T>) {
    let total = alive.len() + trash.len();
    if total == 0 {
        return;
    }
    let ratio = trash.len() as f64 / total as f64;
    if ratio > 0.0 {
        tracing::info!("cleaned {:.1}% of {}", ratio * 100.0, T::get_name());
    }
}
