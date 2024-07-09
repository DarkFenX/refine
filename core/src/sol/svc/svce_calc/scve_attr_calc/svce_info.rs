//! Methods here reimplement attribute calculation counterparts to some extent, to provide extended
//! info while not bloating calculation part (since calculation is supposed to be used much more
//! often than modification info fetching).

use crate::{
    defs::{AttrVal, EAttrId, SolItemId},
    ec,
    sol::{
        item::SolItem,
        svc::{
            svce_calc::{
                SolAffectorInfo, SolAffectorValueInfo, SolAttrValInfo, SolModAccumInfo, SolModification,
                SolModificationInfo, SolModificationKey, SolOpInfo,
            },
            SolSvcs,
        },
        SolView,
    },
    util::{Error, ErrorKind, Result, StMap, StMapVecL1, StSet},
};

const LIMITED_PRECISION_ATTR_IDS: [EAttrId; 4] = [
    ec::attrs::CPU,
    ec::attrs::POWER,
    ec::attrs::CPU_OUTPUT,
    ec::attrs::POWER_OUTPUT,
];

struct SolAffection {
    modification: SolModification,
    affectors: Vec<(SolItemId, EAttrId)>,
}
impl SolAffection {
    fn new(modification: SolModification, affectors: Vec<(SolItemId, EAttrId)>) -> Self {
        Self {
            modification,
            affectors,
        }
    }
}

impl SolSvcs {
    // Query methods
    pub(in crate::sol) fn calc_iter_item_mods(
        &mut self,
        sol_view: &SolView,
        item_id: &SolItemId,
    ) -> Result<impl ExactSizeIterator<Item = (EAttrId, Vec<SolModificationInfo>)>> {
        let mut info_map = StMapVecL1::new();
        for attr_id in self.calc_iter_item_attr_ids(sol_view, item_id)? {
            let mut attr_info = match self.calc_calc_item_attr_info(sol_view, item_id, &attr_id) {
                Ok(attr_info) => attr_info,
                _ => continue,
            };
            let mut info_vec = Vec::new();
            info_vec.extend(attr_info.effective_infos.extract_if(|_| true));
            // info_vec.extend(attr_info.filtered_infos.extract_if(|_| true));
            if !info_vec.is_empty() {
                info_map.extend_entries(attr_id, info_vec.into_iter());
            }
        }
        Ok(info_map.into_iter())
    }
    // Private methods
    fn calc_iter_item_attr_ids(
        &self,
        sol_view: &SolView,
        item_id: &SolItemId,
    ) -> Result<impl ExactSizeIterator<Item = EAttrId>> {
        let mut attr_ids = StSet::new();
        for attr_id in sol_view.items.get_item(item_id)?.get_orig_attrs()?.keys() {
            attr_ids.insert(*attr_id);
        }
        for attr_id in self.calc_data.attrs.get_item_attrs(item_id)?.keys() {
            attr_ids.insert(*attr_id);
        }
        Ok(attr_ids.into_iter())
    }
    fn calc_iter_affections(
        &mut self,
        sol_view: &SolView,
        item: &SolItem,
        attr_id: &EAttrId,
    ) -> impl Iterator<Item = SolAffection> {
        let mut mods = StMap::new();
        for modifier in self
            .calc_data
            .std
            .get_mods_for_affectee(item, attr_id, sol_view.fits)
            .iter()
        {
            let val = match modifier.raw.get_mod_val(self, sol_view) {
                Ok(v) => v,
                _ => continue,
            };
            let affector_item = match sol_view.items.get_item(&modifier.raw.affector_item_id) {
                Ok(i) => i,
                _ => continue,
            };
            let affector_item_cat_id = match affector_item.get_category_id() {
                Ok(affector_item_cat_id) => affector_item_cat_id,
                _ => continue,
            };
            let mod_key = SolModificationKey::from(modifier);
            let modification = SolModification::new(
                modifier.raw.op,
                val,
                self.calc_resist_mult(sol_view, modifier),
                self.calc_proj_mult(sol_view, modifier),
                modifier.raw.aggr_mode,
                affector_item_cat_id,
            );
            let affection = SolAffection::new(modification, modifier.raw.get_affectors(sol_view));
            mods.insert(mod_key, affection);
        }
        mods.into_values()
    }
    fn calc_calc_item_attr_info(
        &mut self,
        sol_view: &SolView,
        item_id: &SolItemId,
        attr_id: &EAttrId,
    ) -> Result<SolAttrValInfo> {
        let item = sol_view.items.get_item(item_id)?;
        let attr = match sol_view.src.get_a_attr(attr_id) {
            Some(attr) => attr,
            None => return Err(Error::new(ErrorKind::AAttrNotFound(*attr_id))),
        };
        // Get base value; use on-item original attributes, or, if not specified, default attribute value.
        // If both can't be fetched, consider it a failure
        let base_val = match item.get_orig_attrs()?.get(attr_id) {
            Some(orig_val) => *orig_val,
            None => attr.def_val,
        };
        match (attr_id, item) {
            (&ec::attrs::SKILL_LEVEL, SolItem::Skill(s)) => return Ok(SolAttrValInfo::new(s.level as AttrVal)),
            _ => (),
        }
        let mut accumulator = SolModAccumInfo::new();
        for affection in self.calc_iter_affections(sol_view, item, attr_id) {
            accumulator.add_val(
                affection.modification.val,
                affection.modification.res_mult,
                affection.modification.proj_mult,
                &affection.modification.op,
                attr.penalizable,
                &affection.modification.affector_item_cat_id,
                &affection.modification.aggr_mode,
                affection
                    .affectors
                    .into_iter()
                    .map(|(item_id, attr_id)| SolAffectorInfo::new(item_id, SolAffectorValueInfo::AttrId(attr_id)))
                    .collect(),
            );
        }
        let mut dogma_attr_info = accumulator.apply_dogma_mods(base_val, attr.hig);
        // Upper cap for the attribute value being calculated
        match attr.max_attr_id {
            Some(capping_attr_id) => match self.calc_get_item_attr_val(sol_view, item_id, &capping_attr_id) {
                Ok(capping_vals) => {
                    self.calc_data
                        .deps
                        .add_direct_local(*item_id, capping_attr_id, *attr_id);
                    if capping_vals.dogma < dogma_attr_info.value {
                        dogma_attr_info.value = capping_vals.dogma;
                        dogma_attr_info.effective_infos.push(SolModificationInfo::new(
                            capping_vals.dogma,
                            None,
                            None,
                            None,
                            capping_vals.dogma,
                            SolOpInfo::MaxLimit,
                            vec![SolAffectorInfo::new(
                                *item_id,
                                SolAffectorValueInfo::AttrId(capping_attr_id),
                            )],
                        ))
                    }
                }
                Err(_) => (),
            },
            None => (),
        };
        if LIMITED_PRECISION_ATTR_IDS.contains(attr_id) {
            dogma_attr_info.value = (dogma_attr_info.value * 100.0).round() / 100.0
        }
        // Post-dogma calculations
        let extra_attr_info = accumulator.apply_extra_mods(dogma_attr_info, attr.hig);
        Ok(extra_attr_info)
    }
}
