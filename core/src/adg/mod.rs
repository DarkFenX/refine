//! Adapted data generator
use std::collections::HashMap;

use crate::{ad, defs::ReeInt, ed, util::IntResult};

mod clean;
mod conv;
mod fetch;
mod norm;
mod pk;
mod rels;
mod valid;

/// Fetch EVE data and generate adapted data out of it
pub(crate) fn generate_adapted_data(e_handler: &dyn ed::EveDataHandler) -> IntResult<ad::AData> {
    let mut g_data = GData::new();
    let mut g_supp = GSupport::new();
    let mut a_data = ad::AData::new();
    fetch::fetch_data(e_handler, &mut g_data)?;
    pk::dedup_pks(&mut g_data);
    norm::normalize(&mut g_data);
    g_supp.fill(&g_data);
    clean::clean_unused(&mut g_data, &g_supp)?;
    valid::validate(&mut g_data, &g_supp);
    conv::convert(&g_data, &g_supp, &mut a_data);
    Ok(a_data)
}

/// Container for primary data, used internally by the generator.
pub(in crate::adg) struct GData {
    pub(in crate::adg) items: Vec<ed::EItem>,
    pub(in crate::adg) groups: Vec<ed::EItemGroup>,
    pub(in crate::adg) attrs: Vec<ed::EAttr>,
    pub(in crate::adg) item_attrs: Vec<ed::EItemAttr>,
    pub(in crate::adg) effects: Vec<ed::EEffect>,
    pub(in crate::adg) item_effects: Vec<ed::EItemEffect>,
    pub(in crate::adg) abils: Vec<ed::EFighterAbil>,
    pub(in crate::adg) item_abils: Vec<ed::EItemFighterAbil>,
    pub(in crate::adg) buffs: Vec<ed::EBuff>,
    pub(in crate::adg) item_srqs: Vec<ed::EItemSkillReq>,
    pub(in crate::adg) muta_items: Vec<ed::EMutaItemConv>,
    pub(in crate::adg) muta_attrs: Vec<ed::EMutaAttrMod>,
}
impl GData {
    pub(in crate::adg) fn new() -> Self {
        Self {
            items: Vec::new(),
            groups: Vec::new(),
            attrs: Vec::new(),
            item_attrs: Vec::new(),
            effects: Vec::new(),
            item_effects: Vec::new(),
            abils: Vec::new(),
            item_abils: Vec::new(),
            buffs: Vec::new(),
            item_srqs: Vec::new(),
            muta_items: Vec::new(),
            muta_attrs: Vec::new(),
        }
    }
}

/// Container for auxiliary data.
pub(in crate::adg) struct GSupport {
    pub(in crate::adg) attr_unit_map: HashMap<ReeInt, ReeInt>,
    pub(in crate::adg) grp_cat_map: HashMap<ReeInt, ReeInt>,
}
impl GSupport {
    pub(in crate::adg) fn new() -> Self {
        Self {
            attr_unit_map: HashMap::new(),
            grp_cat_map: HashMap::new(),
        }
    }
    pub(in crate::adg) fn fill(&mut self, g_data: &GData) {
        self.fill_attr_unit_map(&g_data);
        self.fill_grp_cat_map(&g_data);
    }
    fn fill_attr_unit_map(&mut self, g_data: &GData) {
        for attr in g_data.attrs.iter() {
            if let Some(unit) = attr.unit_id {
                self.attr_unit_map.insert(attr.id, unit);
            }
        }
    }
    fn fill_grp_cat_map(&mut self, g_data: &GData) {
        for grp in g_data.groups.iter() {
            self.grp_cat_map.insert(grp.id, grp.category_id);
        }
    }
}
