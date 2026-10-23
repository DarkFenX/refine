use crate::{
    dbg::DebugResult,
    ud::{UData, UItemId, item::UItem},
    util::RSet,
};

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
