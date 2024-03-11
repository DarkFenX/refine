use std::collections::{HashMap, HashSet};

use crate::{
    defs::{EAttrId, SsItemId},
    ss::{svc::SsSvcs, SsView},
    util::Result,
};
use crate::ss::svc::svce_calc::mod_info::{ModSrcInfo, ModSrcValInfo};

use super::{mod_info::ModInfo, svce_attr::is_penalizable};

impl SsSvcs {
    // Query methods
    pub(in crate::ss) fn calc_get_item_mods(
        &mut self,
        ss_view: &SsView,
        item_id: &SsItemId,
    ) -> Result<HashMap<EAttrId, Vec<ModInfo>>> {
        let item = ss_view.items.get_item(item_id)?;
        let mut info_map = HashMap::new();
        for attr_id in self.calc_get_item_attr_ids(ss_view, item_id)? {
            let attr = match ss_view.src.get_a_attr(&attr_id) {
                Some(attr) => attr,
                None => continue,
            };
            let mut attr_infos = Vec::new();
            for (mod_key, modification) in self.calc_get_modifications(ss_view, item, &attr_id) {
                let src_attr_id = match mod_key.src_attr_id {
                    Some(src_attr_id) => src_attr_id,
                    // TODO: for now, we skip modification info if we can't get source modifier, change it so that we
                    // TODO: always get source of modification
                    None => continue,
                };
                let mod_src = ModSrcInfo::new(mod_key.src_item_id, ModSrcValInfo::AttrId(src_attr_id));
                let mod_info = ModInfo::new(
                    modification.val,
                    (&modification.op).into(),
                    is_penalizable(&modification, &attr),
                    modification.aggr_mode,
                    vec![mod_src],
                );
                attr_infos.push(mod_info);
            }
            if !attr_infos.is_empty() {
                info_map.insert(attr_id, attr_infos);
            }
        }
        Ok(info_map)
    }
    // Private methods
    fn calc_get_item_attr_ids(&self, ss_view: &SsView, item_id: &SsItemId) -> Result<HashSet<EAttrId>> {
        let mut attr_ids = HashSet::new();
        for attr_id in ss_view.items.get_item(item_id)?.get_orig_attrs()?.keys() {
            attr_ids.insert(*attr_id);
        }
        for attr_id in self.calc_data.attrs.get_item_attrs(item_id)?.keys() {
            attr_ids.insert(*attr_id);
        }
        Ok(attr_ids)
    }
}
