use itertools::Itertools;

use crate::{
    ad::{
        ADataGenerator, ADataWarnings,
        generator::{
            AdgSupport,
            rels::{Fk, ItemFk, ItemFkConds, KeyDb, KeyPart},
        },
    },
    ed::{EBuffId, EData, EDataCont, EEffectId, EItem, EItemCatId, EItemGrpId, EItemId, EItemListId},
    util::{LibNamed, RMap, RSet},
};

const MAX_CYCLES: u32 = 100;

impl ADataGenerator {
    pub(in crate::ad::generator) fn clean_unused(&mut self) -> Result<(), ADataGeneratorCleanupError> {
        let mut trash = EData::new();
        self.trash_all(&mut trash);
        // Keys of all items, with conditions they meet
        let item_pks = KeyDb::new_pkdb(&trash).items;
        self.restore_core_items(&mut trash);
        self.restore_attrs(&mut trash);
        self.restore_hardcoded_buffs(&mut trash);
        self.restore_hardcoded_item_lists(&mut trash);

        let mut counter = 0;
        let mut changes = true;
        while changes {
            counter += 1;
            if counter > MAX_CYCLES {
                return Err(ADataGeneratorCleanupError { max_cycles: MAX_CYCLES });
            }
            changes = self.restore_item_data(&mut trash, &item_pks) || self.restore_fk_tgts(&mut trash);
        }
        self.record_stats(&trash);
        self.record_missing_required(&trash, &item_pks);
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
#[error("reached limit of {max_cycles} cycles")]
pub struct ADataGeneratorCleanupError {
    pub max_cycles: u32,
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
        let EData {
            items,
            groups,
            item_lists,
            attrs,
            item_attrs,
            effects,
            item_effects,
            abils,
            item_abils,
            buffs,
            item_buffs,
            muta_items,
            muta_attrs,
        } = &mut self.e_data;
        move_data(items, &mut trash.items, |_| true);
        move_data(groups, &mut trash.groups, |_| true);
        move_data(item_lists, &mut trash.item_lists, |_| true);
        move_data(attrs, &mut trash.attrs, |_| true);
        move_data(item_attrs, &mut trash.item_attrs, |_| true);
        move_data(effects, &mut trash.effects, |_| true);
        move_data(item_effects, &mut trash.item_effects, |_| true);
        move_data(abils, &mut trash.abils, |_| true);
        move_data(item_abils, &mut trash.item_abils, |_| true);
        move_data(buffs, &mut trash.buffs, |_| true);
        move_data(item_buffs, &mut trash.item_buffs, |_| true);
        move_data(muta_items, &mut trash.muta_items, |_| true);
        move_data(muta_attrs, &mut trash.muta_attrs, |_| true);
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
        // Restore only non-dynamic items here, dynamic ones will be restored later as needed
        move_data(&mut trash.items, &mut self.e_data.items, |v| {
            !v.is_dynamic && (items.contains(&v.id) || grps.contains(&v.group_id))
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
    fn restore_item_data(&mut self, trash: &mut EData, item_pks: &RMap<KeyPart, ItemFkConds>) -> bool {
        let restorer = RowRestorer::new(&self.e_data.items, item_pks, &self.support);
        // We need the data which describes our items directly, so rows are restored only via their
        // restoring FKs. For instance, having an item-attribute mapping entry restored just because
        // its value refers some item which is already "alive" is undesired. Item buffs have no
        // restoring FK: they are restored if they contain any buff data, and keep their items
        // alive; buffs of dynamic items wait until a mutator item conversion restores the item.
        restorer.restore(&mut trash.item_attrs, &mut self.e_data.item_attrs)
            || restorer.restore(&mut trash.item_effects, &mut self.e_data.item_effects)
            || restorer.restore(&mut trash.item_abils, &mut self.e_data.item_abils)
            || move_data(&mut trash.item_buffs, &mut self.e_data.item_buffs, |v| {
                v.has_buffs() && restorer.is_alive_or_met(ItemFk::non_dynamic(KeyPart::from_item_eid(v.item_id)))
            })
            || restorer.restore(&mut trash.muta_items, &mut self.e_data.muta_items)
            || restorer.restore(&mut trash.muta_attrs, &mut self.e_data.muta_attrs)
    }
    fn restore_fk_tgts(&mut self, trash: &mut EData) -> bool {
        let fkdb = KeyDb::new_fkdb(&self.e_data, &self.support);
        move_data(&mut trash.items, &mut self.e_data.items, |v| {
            fkdb.items
                .get(&KeyPart::from_item_eid(v.id))
                .is_some_and(|fk_conds| fk_conds.intersects(ItemFkConds::met_by(v)))
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

// Restores rows via their restoring FKs, if none of their required FKs is missing
struct RowRestorer<'a> {
    item_pks_alive: RSet<KeyPart>,
    item_pks_all: &'a RMap<KeyPart, ItemFkConds>,
    adg_supp: &'a AdgSupport,
}
impl<'a> RowRestorer<'a> {
    fn new(alive_items: &EDataCont<EItem>, item_pks_all: &'a RMap<KeyPart, ItemFkConds>, adg_supp: &'a AdgSupport) -> Self {
        Self {
            item_pks_alive: alive_items.data.iter().map(|v| KeyPart::from_item_eid(v.id)).collect(),
            item_pks_all,
            adg_supp,
        }
    }
    fn restore<T>(&self, src_cont: &mut EDataCont<T>, tgt_cont: &mut EDataCont<T>) -> bool
    where
        T: Fk,
    {
        move_data(src_cont, tgt_cont, |v| {
            self.restoring_fk_met(&*v) && self.iter_missing_required(&*v).next().is_none()
        })
    }
    fn restoring_fk_met<T>(&self, row: &T) -> bool
    where
        T: Fk,
    {
        row.get_item_fk_self_restoring()
            .is_some_and(|fk| self.item_pks_alive.contains(&fk.key) && self.is_met(fk))
    }
    fn iter_missing_required<T>(&self, row: &T) -> impl Iterator<Item = KeyPart>
    where
        T: Fk,
    {
        row.get_item_fks_regular(self.adg_supp)
            .into_iter()
            .filter(|fk| fk.required && !self.is_met(*fk))
            .map(|fk| fk.key)
    }
    // Target exists and meets the condition
    fn is_met(&self, fk: ItemFk) -> bool {
        self.item_pks_all.get(&fk.key).is_some_and(|conds| conds.contains(fk.cond))
    }
    fn is_alive_or_met(&self, fk: ItemFk) -> bool {
        self.item_pks_alive.contains(&fk.key) || self.is_met(fk)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Reporting - stats and errors
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ADataGenerator {
    fn record_stats(&mut self, trash: &EData) {
        let EData {
            items,
            groups,
            item_lists,
            attrs,
            item_attrs,
            effects,
            item_effects,
            abils,
            item_abils,
            buffs,
            item_buffs,
            muta_items,
            muta_attrs,
        } = &self.e_data;
        record_cont_stats(items, &trash.items, &mut self.a_data.warnings);
        record_cont_stats(groups, &trash.groups, &mut self.a_data.warnings);
        record_cont_stats(item_lists, &trash.item_lists, &mut self.a_data.warnings);
        record_cont_stats(attrs, &trash.attrs, &mut self.a_data.warnings);
        record_cont_stats(item_attrs, &trash.item_attrs, &mut self.a_data.warnings);
        record_cont_stats(effects, &trash.effects, &mut self.a_data.warnings);
        record_cont_stats(item_effects, &trash.item_effects, &mut self.a_data.warnings);
        record_cont_stats(abils, &trash.abils, &mut self.a_data.warnings);
        record_cont_stats(item_abils, &trash.item_abils, &mut self.a_data.warnings);
        record_cont_stats(buffs, &trash.buffs, &mut self.a_data.warnings);
        record_cont_stats(item_buffs, &trash.item_buffs, &mut self.a_data.warnings);
        record_cont_stats(muta_items, &trash.muta_items, &mut self.a_data.warnings);
        record_cont_stats(muta_attrs, &trash.muta_attrs, &mut self.a_data.warnings);
    }
    fn record_missing_required(&mut self, trash: &EData, item_pks: &RMap<KeyPart, ItemFkConds>) {
        let restorer = RowRestorer::new(&self.e_data.items, item_pks, &self.support);
        let a_warnings = &mut self.a_data.warnings;
        record_cont_missing_required(&restorer, &trash.item_attrs, a_warnings);
        record_cont_missing_required(&restorer, &trash.item_effects, a_warnings);
        record_cont_missing_required(&restorer, &trash.item_abils, a_warnings);
        record_cont_missing_required(&restorer, &trash.muta_items, a_warnings);
        record_cont_missing_required(&restorer, &trash.muta_attrs, a_warnings);
    }
}

fn record_cont_stats<T>(e_cont_alive: &EDataCont<T>, e_cont_trash: &EDataCont<T>, a_warnings: &mut ADataWarnings)
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

// Rows whose restoring FK is met stay in trash only due to missing required FK targets
fn record_cont_missing_required<T>(restorer: &RowRestorer, e_cont_trash: &EDataCont<T>, a_warnings: &mut ADataWarnings)
where
    T: Fk + LibNamed,
{
    let mut rows = 0;
    let mut missing_keys = RSet::new();
    for row in e_cont_trash.data.iter().filter(|v| restorer.restoring_fk_met(*v)) {
        rows += 1;
        missing_keys.extend(restorer.iter_missing_required(row));
    }
    if rows == 0 {
        return;
    }
    let entry_limit = 5;
    let entry_count = missing_keys.len();
    let mut warning = format!(
        "skipped {} {} with missing required {}: {}",
        rows,
        T::lib_get_name(),
        EItem::lib_get_name(),
        missing_keys
            .into_iter()
            .map(|v| v.into_i32())
            .sorted_unstable()
            .take(entry_limit)
            .join(", ")
    );
    if entry_count > entry_limit {
        warning = format!("{}, <{} more entries hidden>", warning, entry_count - entry_limit);
    }
    a_warnings.cleanup.push(warning);
}
