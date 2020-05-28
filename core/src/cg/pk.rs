//! Contains facilities which clean up data to ensure no duplicate primary keys exist.

use std::collections::HashSet;

use crate::defines::ReeInt;
use crate::dh;
use crate::util::Named;

use super::Data;

trait Pk {
    fn get_pk(&self) -> Vec<ReeInt>;
}
impl Pk for dh::Item {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.id]
    }
}
impl Pk for dh::ItemGroup {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.id]
    }
}
impl Pk for dh::Attr {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.id]
    }
}
impl Pk for dh::ItemAttr {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.item_id, self.attr_id]
    }
}
impl Pk for dh::Effect {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.id]
    }
}
impl Pk for dh::ItemEffect {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.item_id, self.effect_id]
    }
}
impl Pk for dh::FighterAbil {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.id]
    }
}
impl Pk for dh::ItemFighterAbil {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.item_id, self.abil_id]
    }
}
impl Pk for dh::Buff {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.id]
    }
}
impl Pk for dh::ItemSkillReq {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.item_id, self.skill_id]
    }
}
impl Pk for dh::MutaItemConv {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.muta_id, self.in_item_id]
    }
}
impl Pk for dh::MutaAttrMod {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.muta_id, self.attr_id]
    }
}

fn dedup_pks_vec<T>(vec: &mut Vec<T>, errs: &mut Vec<String>)
where
    T: Pk + Named,
{
    let mut seen_pks = HashSet::new();
    let invalid_iter = vec.drain_filter(|v| {
        let pk = v.get_pk();
        if seen_pks.contains(&pk) {
            true
        } else {
            seen_pks.insert(pk);
            false
        }
    });
    let invalid_len = invalid_iter.count();
    if invalid_len > 0 {
        let msg = format!("cleaned up {} PK duplicates for {}", invalid_len, T::get_name());
        log::warn!("{}", &msg);
        errs.push(msg);
    }
}

pub(super) fn dedup_pks(data: &mut Data, errs: &mut Vec<String>) {
    dedup_pks_vec(&mut data.items, errs);
    dedup_pks_vec(&mut data.item_groups, errs);
    dedup_pks_vec(&mut data.attrs, errs);
    dedup_pks_vec(&mut data.item_attrs, errs);
    dedup_pks_vec(&mut data.effects, errs);
    dedup_pks_vec(&mut data.item_effects, errs);
    dedup_pks_vec(&mut data.fighter_abils, errs);
    dedup_pks_vec(&mut data.item_fighter_abils, errs);
    dedup_pks_vec(&mut data.buffs, errs);
    dedup_pks_vec(&mut data.item_skill_reqs, errs);
    dedup_pks_vec(&mut data.muta_item_convs, errs);
    dedup_pks_vec(&mut data.muta_attr_mods, errs);
}
