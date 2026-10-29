use crate::{
    misc::{DpsProfile, NpcProp, OptionalReload, RearmMinion, SecZone, Spool},
    num::{PValue, UnitInterval},
    rd::RcData,
    ud::{UFits, UFleets, UItemId, UItems},
    util::RSet,
};

/// UAD stands for User and Adapted Data. Per definition, contains user-defined data, as well as
/// container with EVE-derived-data, since which one is used is also chosen by user.
#[derive(Clone)]
pub(crate) struct UData {
    pub(crate) r_data: RcData,
    pub(crate) fleets: UFleets,
    pub(crate) fits: UFits,
    pub(crate) sw_effects: RSet<UItemId>,
    pub(crate) proj_effects: RSet<UItemId>,
    pub(crate) items: UItems,
    pub(crate) sec_zone: SecZone,
    // Default setting used in stats / RAH sim
    pub(crate) default_incoming_dps: DpsProfile,
    // Default settings related to item cycles
    pub(crate) default_spool: Spool,
    pub(crate) default_npc_prop: NpcProp,
    pub(crate) default_optional_reloads: OptionalReload,
    pub(crate) default_rearm_minions: RearmMinion,
}
impl UData {
    pub(crate) fn new(r_data: RcData) -> Self {
        Self {
            r_data,
            fleets: UFleets::new(5),
            fits: UFits::new(50),
            sw_effects: RSet::new(),
            proj_effects: RSet::new(),
            items: UItems::new(10000),
            sec_zone: SecZone::NullSec,
            default_incoming_dps: DpsProfile {
                em: PValue::ONE,
                thermal: PValue::ONE,
                kinetic: PValue::ONE,
                explosive: PValue::ONE,
                ..
            },
            default_spool: Spool::SpoolScale(UnitInterval::from_f64_clamped(1.0)),
            default_npc_prop: NpcProp::Chase,
            default_optional_reloads: OptionalReload::OnEmpty,
            default_rearm_minions: RearmMinion::Disabled,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "dev")]
mod debug {
    use super::*;
    use crate::{dbg::DebugResult, ud::item::UItem};

    impl UData {
        pub(crate) fn consistency_check(&self) -> DebugResult {
            let Self {
                fleets,
                fits,
                sw_effects,
                proj_effects,
                items,
                // Not checked
                r_data: _,
                sec_zone: _,
                default_incoming_dps: _,
                default_spool: _,
                default_npc_prop: _,
                default_optional_reloads: _,
                default_rearm_minions: _,
            } = self;
            let mut seen_items = Vec::new();
            // Fleets
            for fleet in fleets.values() {
                fleet.consistency_check(self)?;
            }
            // Fits
            for fit in fits.values() {
                fit.consistency_check(self, &mut seen_items)?;
            }
            // System-wide effects
            for &sw_effect_uid in sw_effects.iter() {
                seen_items.push(sw_effect_uid);
                let Some(item) = items.try_get(sw_effect_uid) else {
                    return Err(Default::default());
                };
                if !matches!(item, UItem::SwEffect(..)) {
                    return Err(Default::default());
                }
                item.consistency_check(self)?;
            }
            // Projected effects
            for &proj_effect_uid in proj_effects.iter() {
                seen_items.push(proj_effect_uid);
                let item = match items.try_get(proj_effect_uid) {
                    Some(item) => item,
                    None => return Err(Default::default()),
                };
                if !matches!(item, UItem::ProjEffect(..)) {
                    return Err(Default::default());
                }
                item.consistency_check(self)?;
            }
            // Check if we have any duplicate references to items
            if check_item_duplicates(&seen_items) {
                return Err(Default::default());
            }
            // Check if we have any unreferenced items
            if !items.keys().all(|item_uid| seen_items.contains(&item_uid)) {
                return Err(Default::default());
            }
            // Checks for internal container consistency
            items.consistency_check()?;
            fits.consistency_check()?;
            fleets.consistency_check()?;
            Ok(())
        }
    }

    fn check_item_duplicates(item_ids: &[UItemId]) -> bool {
        let mut uniq = RSet::new();
        !item_ids.iter().all(|x| uniq.insert(x))
    }
}
