use std::collections::HashSet;

use crate::consts::{itemcats, itemgrps};

use super::data::{Data, Support};
use crate::cg::data::Pk;

const MAX_CYCLES: i32 = 500;

fn move_data<T, F>(src_vec: &mut Vec<T>, dst_vec: &mut Vec<T>, filter: F) -> bool
where
    F: FnMut(&mut T) -> bool,
{
    let drained: Vec<T> = src_vec.drain_filter(filter).collect();
    let changes = drained.len() > 0;
    dst_vec.extend(drained);
    changes
}

fn trash_all(alive: &mut Data, trash: &mut Data) {
    move_data(&mut alive.items, &mut trash.items, |_| true);
    move_data(&mut alive.item_groups, &mut trash.item_groups, |_| true);
    move_data(&mut alive.attrs, &mut trash.attrs, |_| true);
    move_data(&mut alive.item_attrs, &mut trash.item_attrs, |_| true);
    move_data(&mut alive.effects, &mut trash.effects, |_| true);
    move_data(&mut alive.item_effects, &mut trash.item_effects, |_| true);
    move_data(&mut alive.fighter_abils, &mut trash.fighter_abils, |_| true);
    move_data(&mut alive.item_fighter_abils, &mut trash.item_fighter_abils, |_| true);
    move_data(&mut alive.buffs, &mut trash.buffs, |_| true);
    move_data(&mut alive.item_skill_reqs, &mut trash.item_skill_reqs, |_| true);
    move_data(&mut alive.muta_item_convs, &mut trash.muta_item_convs, |_| true);
    move_data(&mut alive.muta_attr_mods, &mut trash.muta_attr_mods, |_| true);
}

fn restore_core_items(alive: &mut Data, trash: &mut Data, support: &Support) {
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
    for (&grp, cat) in support.grp_cat_map.iter() {
        if cats.contains(cat) {
            grps.push(grp);
        }
    }
    move_data(&mut trash.items, &mut alive.items, |v| grps.contains(&v.group_id));
}

fn restore_item_data(alive: &mut Data, trash: &mut Data) -> bool {
    let mut item_ids = HashSet::new();
    for item in alive.items.iter() {
        item_ids.extend(item.get_pk());
    }
    // Here we intentionally do not use FKs defined on those entities, since we need the data which
    // describes our items directly
    move_data(&mut trash.item_attrs, &mut alive.item_attrs, |v| {
        item_ids.contains(&v.item_id)
    }) | move_data(&mut trash.item_effects, &mut alive.item_effects, |v| {
        item_ids.contains(&v.item_id)
    }) | move_data(&mut trash.item_fighter_abils, &mut alive.item_fighter_abils, |v| {
        item_ids.contains(&v.item_id)
    }) | move_data(&mut trash.item_skill_reqs, &mut alive.item_skill_reqs, |v| {
        item_ids.contains(&v.item_id)
    }) | move_data(&mut trash.muta_item_convs, &mut alive.muta_item_convs, |v| {
        item_ids.contains(&v.in_item_id)
    })
}

pub(super) fn clean_unused(mut alive: &mut Data, support: &Support) {
    let mut trash = Data::new();
    trash_all(&mut alive, &mut trash);
    restore_core_items(&mut alive, &mut trash, &support);

    let mut counter = 0;
    let mut changes = true;
    while changes {
        counter += 1;
        if counter > MAX_CYCLES {
            // TODO: throw an error
            break;
        }
        changes = restore_item_data(&mut alive, &mut trash);
    }

    // println!("items: {} {}", alive.items.len(), trash.items.len());
    // println!("item_groups: {} {}", alive.item_groups.len(), trash.item_groups.len());
    // println!("attrs: {} {}", alive.attrs.len(), trash.attrs.len());
    // println!("item_attrs: {} {}", alive.item_attrs.len(), trash.item_attrs.len());
    // println!("effects: {} {}", alive.effects.len(), trash.effects.len());
    // println!("item_effects: {} {}", alive.item_effects.len(), trash.item_effects.len());
    // println!("fighter_abils: {} {}", alive.fighter_abils.len(), trash.fighter_abils.len());
    // println!("buffs: {} {}", alive.buffs.len(), trash.buffs.len());
    // println!("item_skill_reqs: {} {}", alive.item_skill_reqs.len(), trash.item_skill_reqs.len());
    // println!("muta_item_convs: {} {}", alive.muta_item_convs.len(), trash.muta_item_convs.len());
    // println!("muta_attr_mods: {} {}", alive.muta_attr_mods.len(), trash.muta_attr_mods.len());
}
