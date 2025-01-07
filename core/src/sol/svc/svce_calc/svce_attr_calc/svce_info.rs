//! Methods here reimplement attribute calculation counterparts to some extent, to provide extended
//! info while not bloating calculation part (since calculation is supposed to be used much more
//! often than modification info fetching).

use crate::{
    defs::{AttrVal, EAttrId, SolItemId},
    ec,
    err::basic::AttrMetaFoundError,
    sol::{
        svc::{
            err::{AttrCalcError, LoadedItemFoundError},
            svce_calc::{
                SolAffectorInfo, SolAttrValInfo, SolModAccumInfo, SolModification, SolModificationInfo,
                SolModificationKey, SolOpInfo,
            },
            SolSvc,
        },
        uad::{item::SolItem, SolUad},
    },
    util::{round, StMap, StMapVecL1, StSet},
};

const LIMITED_PRECISION_ATTR_IDS: [EAttrId; 4] = [
    ec::attrs::CPU,
    ec::attrs::POWER,
    ec::attrs::CPU_OUTPUT,
    ec::attrs::POWER_OUTPUT,
];

struct SolAffection {
    modification: SolModification,
    affectors: Vec<SolAffectorInfo>,
}
impl SolAffection {
    fn new(modification: SolModification, affectors: Vec<SolAffectorInfo>) -> Self {
        Self {
            modification,
            affectors,
        }
    }
}

impl SolSvc {
    // Query methods
    pub(in crate::sol) fn calc_iter_item_mods(
        &mut self,
        uad: &SolUad,
        item_id: &SolItemId,
    ) -> Result<impl ExactSizeIterator<Item = (EAttrId, Vec<SolModificationInfo>)>, LoadedItemFoundError> {
        let mut info_map = StMapVecL1::new();
        for attr_id in self.calc_iter_item_attr_ids(uad, item_id)? {
            let mut attr_info = match self.calc_calc_item_attr_info(uad, item_id, &attr_id) {
                Ok(attr_info) => attr_info,
                _ => continue,
            };
            let mut info_vec = Vec::new();
            info_vec.extend(attr_info.effective_infos.extract_if(.., |_| true));
            // info_vec.extend(attr_info.filtered_infos.extract_if(.., |_| true));
            if !info_vec.is_empty() {
                info_map.extend_entries(attr_id, info_vec.into_iter());
            }
        }
        Ok(info_map.into_iter())
    }
    // Private methods
    fn calc_iter_item_attr_ids(
        &self,
        uad: &SolUad,
        item_id: &SolItemId,
    ) -> Result<impl ExactSizeIterator<Item = EAttrId>, LoadedItemFoundError> {
        let mut attr_ids = StSet::new();
        for attr_id in uad.items.get_item(item_id)?.get_attrs()?.keys() {
            attr_ids.insert(*attr_id);
        }
        for attr_id in self.calc.attrs.get_item_attr_data(item_id).unwrap().values.keys() {
            attr_ids.insert(*attr_id);
        }
        Ok(attr_ids.into_iter())
    }
    fn calc_iter_affections(
        &mut self,
        uad: &SolUad,
        item: &SolItem,
        attr_id: &EAttrId,
    ) -> impl Iterator<Item = SolAffection> {
        let mut mods = StMap::new();
        for modifier in self.calc.std.get_mods_for_affectee(item, attr_id, &uad.fits).iter() {
            let val = match modifier.raw.get_mod_val(self, uad) {
                Some(v) => v,
                None => continue,
            };
            let affector_item = uad.items.get_item(&modifier.raw.affector_item_id).unwrap();
            let affector_item_cat_id = affector_item.get_category_id().unwrap();
            let mod_key = SolModificationKey::from(modifier);
            let modification = SolModification::new(
                modifier.raw.op,
                val,
                self.calc_resist_mult(uad, modifier),
                self.calc_proj_mult(uad, modifier),
                modifier.raw.aggr_mode,
                affector_item_cat_id,
            );
            let affection = SolAffection::new(modification, modifier.raw.get_affector_info(uad));
            mods.insert(mod_key, affection);
        }
        mods.into_values()
    }
    fn calc_calc_item_attr_info(
        &mut self,
        uad: &SolUad,
        item_id: &SolItemId,
        attr_id: &EAttrId,
    ) -> Result<SolAttrValInfo, AttrCalcError> {
        let item = uad.items.get_item(item_id)?;
        let attr = match uad.src.get_a_attr(attr_id) {
            Some(attr) => attr,
            None => return Err(AttrMetaFoundError::new(*attr_id).into()),
        };
        // Get base value; use on-item original attributes, or, if not specified, default attribute value.
        // If both can't be fetched, consider it a failure
        let base_val = match item.get_attrs()?.get(attr_id) {
            Some(orig_val) => *orig_val,
            None => attr.def_val,
        };
        match (attr_id, item) {
            (&ec::attrs::SKILL_LEVEL, SolItem::Skill(s)) => {
                return Ok(SolAttrValInfo::new(AttrVal::from(s.get_level())))
            }
            _ => (),
        }
        let mut accumulator = SolModAccumInfo::new();
        for affection in self.calc_iter_affections(uad, item, attr_id) {
            accumulator.add_val(
                affection.modification.val,
                affection.modification.proj_mult,
                affection.modification.res_mult,
                &affection.modification.op,
                attr.penalizable,
                &affection.modification.affector_item_cat_id,
                &affection.modification.aggr_mode,
                affection.affectors,
            );
        }
        let mut dogma_attr_info = accumulator.apply_dogma_mods(base_val, attr.hig);
        // Lower value limit
        if let Some(limiter_attr_id) = attr.min_attr_id {
            if let Ok(limiter_val) = self.calc_get_item_attr_val(uad, item_id, &limiter_attr_id) {
                self.calc.deps.add_direct_local(*item_id, limiter_attr_id, *attr_id);
                if limiter_val.dogma > dogma_attr_info.value {
                    dogma_attr_info.value = limiter_val.dogma;
                    dogma_attr_info.effective_infos.push(SolModificationInfo::new(
                        SolOpInfo::MinLimit,
                        limiter_val.dogma,
                        None,
                        None,
                        None,
                        limiter_val.dogma,
                        vec![SolAffectorInfo::new(*item_id, Some(limiter_attr_id))],
                    ))
                }
            }
        }
        // Upper value limit
        if let Some(limiter_attr_id) = attr.max_attr_id {
            if let Ok(limiter_val) = self.calc_get_item_attr_val(uad, item_id, &limiter_attr_id) {
                self.calc.deps.add_direct_local(*item_id, limiter_attr_id, *attr_id);
                if limiter_val.dogma < dogma_attr_info.value {
                    dogma_attr_info.value = limiter_val.dogma;
                    dogma_attr_info.effective_infos.push(SolModificationInfo::new(
                        SolOpInfo::MaxLimit,
                        limiter_val.dogma,
                        None,
                        None,
                        None,
                        limiter_val.dogma,
                        vec![SolAffectorInfo::new(*item_id, Some(limiter_attr_id))],
                    ))
                }
            }
        }
        if LIMITED_PRECISION_ATTR_IDS.contains(attr_id) {
            dogma_attr_info.value = round(dogma_attr_info.value, 2);
        }
        // Post-dogma calculations
        let extra_attr_info = accumulator.apply_extra_mods(dogma_attr_info, attr.hig);
        Ok(extra_attr_info)
    }
}
