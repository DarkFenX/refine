use crate::{
    ad::{
        ADataGenerator, ADataGeneratorError, AdgWarnings,
        generator::rels::{KeyDb, KeyPart},
    },
    ed::{EBuffId, EData, EDataCont, EEffectId, EItemCatId, EItemGrpId, EItemId, EItemListId},
    util::{LibNamed, RSet},
};

const MAX_CYCLES: u32 = 100;

impl ADataGenerator {
    pub(in crate::ad::generator) fn clean_unused(&mut self) -> Result<(), ADataGeneratorError> {
        let mut trash = EData::new();
        self.trash_all(&mut trash);
        self.restore_core_items(&mut trash);
        self.restore_attrs(&mut trash);
        self.restore_hardcoded_buffs(&mut trash);
        self.restore_hardcoded_item_lists(&mut trash);

        let mut counter = 0;
        let mut changes = true;
        while changes {
            counter += 1;
            if counter > MAX_CYCLES {
                return Err(ADataGeneratorError::CleanupFailed(format!(
                    "reached limit of {MAX_CYCLES} cycles"
                )));
            }
            changes = self.restore_item_data(&mut trash) || self.restore_fk_tgts(&mut trash);
        }
        self.record_stats(&trash);
        Ok(())
    }
}

fn move_data<T, F>(src_cont: &mut EDataCont<T>, tgt_cont: &mut EDataCont<T>, filter: F) -> bool
where
    F: FnMut(&mut T) -> bool,
{
    let mut drained = src_cont.data.extract_if(.., filter).peekable();
    let changes = drained.peek().is_some();
    tgt_cont.data.extend(drained);
    changes
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Initial preparation
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ADataGenerator {
    fn trash_all(&mut self, trash: &mut EData) {
        move_data(&mut self.e_data.items, &mut trash.items, |_| true);
        move_data(&mut self.e_data.groups, &mut trash.groups, |_| true);
        move_data(&mut self.e_data.item_lists, &mut trash.item_lists, |_| true);
        move_data(&mut self.e_data.attrs, &mut trash.attrs, |_| true);
        move_data(&mut self.e_data.item_attrs, &mut trash.item_attrs, |_| true);
        move_data(&mut self.e_data.effects, &mut trash.effects, |_| true);
        move_data(&mut self.e_data.item_effects, &mut trash.item_effects, |_| true);
        move_data(&mut self.e_data.abils, &mut trash.abils, |_| true);
        move_data(&mut self.e_data.item_abils, &mut trash.item_abils, |_| true);
        move_data(&mut self.e_data.buffs, &mut trash.buffs, |_| true);
        move_data(&mut self.e_data.space_comps, &mut trash.space_comps, |_| true);
        move_data(&mut self.e_data.item_srqs, &mut trash.item_srqs, |_| true);
        move_data(&mut self.e_data.muta_items, &mut trash.muta_items, |_| true);
        move_data(&mut self.e_data.muta_attrs, &mut trash.muta_attrs, |_| true);
    }
    fn restore_core_items(&mut self, trash: &mut EData) {
        let cats = [
            EItemCatId::CHARGE,
            EItemCatId::DRONE,
            EItemCatId::FIGHTER,
            EItemCatId::IMPLANT,
            EItemCatId::MODULE,
            EItemCatId::SHIP,
            EItemCatId::SKILL,
            EItemCatId::STRUCTURE,
            EItemCatId::STRUCTURE_MODULE,
            EItemCatId::SUBSYSTEM,
        ];
        let mut grps = vec![
            EItemGrpId::CHARACTER,
            EItemGrpId::EFFECT_BEACON,
            EItemGrpId::DESTRUCTIBLE_EFFECT_BEACON,
            EItemGrpId::TEMPORARY_COLLIDABLE_STRUCTURES,
            EItemGrpId::ABYSSAL_HAZARDS,
            EItemGrpId::SOV_HUB_SYSTEM_EFFECT_GENERATOR_UPGRADES,
        ];
        // Some useful items are hard to pick apart from others; for example, abyssal weathers
        // belong to 2 separate groups (non-interactable objects and massive environments), with
        // both groups including lots of items useless for the lib. Just rely on effects to restore
        // those.
        let effs = [
            EEffectId::WEATHER_ELECTRIC_STORM,
            EEffectId::WEATHER_INFERNAL,
            EEffectId::WEATHER_CAUSTIC_TOXIN,
            EEffectId::WEATHER_XENON_GAS,
            EEffectId::WEATHER_DARKNESS,
            EEffectId::AOE_BEACON_PULSE_01,
        ];
        // Items included directly, for cases when there is no easy other way to include them
        let mut items = vec![EItemId::WEAPON_OVERCHARGE_PYLON];
        for (&grp, cat) in self.support.grp_cat_map.iter() {
            if cats.contains(cat) {
                grps.push(grp);
            }
        }
        for eff in effs {
            items.extend(self.support.eff_item_map.get(&eff).copied())
        }
        move_data(&mut trash.items, &mut self.e_data.items, |v| {
            items.contains(&v.id) || grps.contains(&v.group_id)
        });
    }
    fn restore_attrs(&mut self, trash: &mut EData) {
        // Some attributes are known to be used by EVE, despite them not referred from anywhere.
        // Oftentimes, those are needed, due to how attribute calculation works: a user can request
        // calculation of any attribute on any item, and this would use on-attribute info for
        // calculation (for example, default value). Just restore all the attributes here.
        move_data(&mut trash.attrs, &mut self.e_data.attrs, |_| true);
    }
    fn restore_hardcoded_buffs(&mut self, trash: &mut EData) {
        // Used in custom wubble effect
        move_data(&mut trash.buffs, &mut self.e_data.buffs, |v| {
            v.id == EBuffId::STASIS_WEBIFICATION_BURST
        });
    }
    fn restore_hardcoded_item_lists(&mut self, trash: &mut EData) {
        // Used in sec zone validation
        move_data(&mut trash.item_lists, &mut self.e_data.item_lists, |v| {
            v.id == EItemListId::WORMHOLE_JUMP_BLACK_LIST
        });
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Cyclic restoration
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ADataGenerator {
    fn restore_item_data(&mut self, trash: &mut EData) -> bool {
        let item_ids: RSet<_> = self.e_data.items.data.iter().map(|v| v.id).collect();
        // We need the data which describes our items directly, so some FKs are avoided
        // deliberately. For instance, having an item-attribute mapping entry restored just because
        // its value refers some item which is already "alive" is undesired.
        //
        // Extra notes on specific entities:
        // - Space components are restored if they contain any buff data
        // - Mutator item conversions are restored for input/output items which are alive
        // - Mutator attribute modifications are restored for alive mutators
        move_data(&mut trash.item_attrs, &mut self.e_data.item_attrs, |v| {
            item_ids.contains(&v.item_id)
        }) || move_data(&mut trash.item_effects, &mut self.e_data.item_effects, |v| {
            item_ids.contains(&v.item_id)
        }) || move_data(&mut trash.item_abils, &mut self.e_data.item_abils, |v| {
            item_ids.contains(&v.item_id)
        }) || move_data(&mut trash.space_comps, &mut self.e_data.space_comps, |v| v.has_buffs())
            || move_data(&mut trash.item_srqs, &mut self.e_data.item_srqs, |v| {
                item_ids.contains(&v.item_id)
            })
            || move_data(&mut trash.muta_items, &mut self.e_data.muta_items, |v| {
                item_ids.contains(&v.in_item_id) || item_ids.contains(&v.out_item_id)
            })
            || move_data(&mut trash.muta_attrs, &mut self.e_data.muta_attrs, |v| {
                item_ids.contains(&v.muta_id)
            })
    }
    fn restore_fk_tgts(&mut self, trash: &mut EData) -> bool {
        let fkdb = KeyDb::new_fkdb(&self.e_data, &self.support);
        move_data(&mut trash.items, &mut self.e_data.items, |v| {
            fkdb.items.contains(&KeyPart::from_item_eid(v.id))
        }) || move_data(&mut trash.groups, &mut self.e_data.groups, |v| {
            fkdb.groups.contains(&KeyPart::from_item_grp_eid(v.id))
        }) || move_data(&mut trash.item_lists, &mut self.e_data.item_lists, |v| {
            fkdb.item_lists.contains(&KeyPart::from_item_list_eid(v.id))
        }) || move_data(&mut trash.attrs, &mut self.e_data.attrs, |v| {
            fkdb.attrs.contains(&KeyPart::from_attr_eid(v.id))
        }) || move_data(&mut trash.effects, &mut self.e_data.effects, |v| {
            fkdb.effects.contains(&KeyPart::from_effect_eid(v.id))
        }) || move_data(&mut trash.abils, &mut self.e_data.abils, |v| {
            fkdb.abils.contains(&KeyPart::from_abil_eid(v.id))
        }) || move_data(&mut trash.buffs, &mut self.e_data.buffs, |v| {
            fkdb.buffs.contains(&KeyPart::from_buff_eid(v.id))
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Recording stats
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ADataGenerator {
    fn record_stats(&mut self, trash: &EData) {
        record_cont_stats(&self.e_data.items, &trash.items, &mut self.a_data.warnings);
        record_cont_stats(&self.e_data.groups, &trash.groups, &mut self.a_data.warnings);
        record_cont_stats(&self.e_data.item_lists, &trash.item_lists, &mut self.a_data.warnings);
        record_cont_stats(&self.e_data.attrs, &trash.attrs, &mut self.a_data.warnings);
        record_cont_stats(&self.e_data.item_attrs, &trash.item_attrs, &mut self.a_data.warnings);
        record_cont_stats(&self.e_data.effects, &trash.effects, &mut self.a_data.warnings);
        record_cont_stats(
            &self.e_data.item_effects,
            &trash.item_effects,
            &mut self.a_data.warnings,
        );
        record_cont_stats(&self.e_data.abils, &trash.abils, &mut self.a_data.warnings);
        record_cont_stats(&self.e_data.item_abils, &trash.item_abils, &mut self.a_data.warnings);
        record_cont_stats(&self.e_data.buffs, &trash.buffs, &mut self.a_data.warnings);
        record_cont_stats(&self.e_data.space_comps, &trash.space_comps, &mut self.a_data.warnings);
        record_cont_stats(&self.e_data.item_srqs, &trash.item_srqs, &mut self.a_data.warnings);
        record_cont_stats(&self.e_data.muta_items, &trash.muta_items, &mut self.a_data.warnings);
        record_cont_stats(&self.e_data.muta_attrs, &trash.muta_attrs, &mut self.a_data.warnings);
    }
}

fn record_cont_stats<T>(e_cont_alive: &EDataCont<T>, e_cont_trash: &EDataCont<T>, a_warnings: &mut AdgWarnings)
where
    T: LibNamed,
{
    let removed = e_cont_trash.data.len();
    if removed == 0 {
        return;
    }
    let total = e_cont_alive.data.len() + removed;
    let ratio = removed as f64 / total as f64;
    let warning = format!("cleaned {:.1}% of {}", ratio * 100.0, T::lib_get_name());
    a_warnings.cleanup.push(warning);
}
