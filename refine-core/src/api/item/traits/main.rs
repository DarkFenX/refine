use crate::{
    AttrId, EffectId, EffectMode, ItemAttrValues, ItemEffectInfo, ItemId, ItemTypeId, Modification,
    api::ItemSealed,
    err::{
        ItemAttrGetError, ItemAttrsIterError, ItemEffectsIterError, ItemModifiersIterError,
        basic::{AttrFoundError, ItemLoadedError},
    },
    stats::ItemStats,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Methods shared by every item kind.
#[expect(private_bounds)]
pub trait ItemCommon: ItemSealed {
    fn get_item_id(&self) -> ItemId {
        self.get_sol().u_data.items.ext_id_by_int_id(self.get_uid())
    }
    fn get_type_id(&self) -> ItemTypeId {
        let type_aid = self.get_sol().u_data.items.get(self.get_uid()).get_rendered_type_aid();
        ItemTypeId::from_aid(type_aid)
    }
    fn iter_effects(&self) -> Result<impl ExactSizeIterator<Item = (EffectId, ItemEffectInfo)>, ItemEffectsIterError> {
        let sol = self.get_sol();
        let item_uid = self.get_uid();
        let item = sol.u_data.items.get(item_uid);
        let (effect_rids, reffs) = match (item.get_r_item_base(), item.get_reffs()) {
            (Some(item_rib), Some(reffs)) => (item_rib.effect_rids.iter(), reffs),
            _ => {
                return Err(ItemLoadedError {
                    item_id: sol.u_data.items.ext_id_by_int_id(item_uid),
                }
                .into());
            }
        };
        let effect_infos = effect_rids.map(move |&effect_rid| {
            let effect_aid = sol.u_data.r_data.get_effect_by_rid(effect_rid).aid;
            let running = reffs.contains(effect_rid);
            let mode = item.get_effect_mode_by_rid(effect_rid);
            (EffectId::from_aid(effect_aid), ItemEffectInfo { running, mode })
        });
        Ok(effect_infos)
    }
    fn iter_effect_mode_overrides(&self) -> impl ExactSizeIterator<Item = (EffectId, EffectMode)> {
        let sol = self.get_sol();
        let item_uid = self.get_uid();
        let item = sol.u_data.items.get(item_uid);
        item.iter_effect_mode_overrides()
            .map(|(effect_aid, effect_mode)| (EffectId::from_aid(effect_aid), effect_mode))
    }
    fn get_attr(&mut self, attr_id: &AttrId) -> Result<ItemAttrValues, ItemAttrGetError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        let attr_aid = attr_id.into_aid();
        let Some(attr_rid) = sol.u_data.r_data.get_attr_rid_by_aid(&attr_aid) else {
            return Err(AttrFoundError { attr_id: *attr_id }.into());
        };
        match sol.internal_get_item_attr(item_uid, attr_rid) {
            Ok(calc_vals) => Ok(ItemAttrValues::from_calc_attr_vals(calc_vals)),
            Err(error) => Err(ItemLoadedError {
                item_id: self.get_sol_mut().u_data.items.ext_id_by_int_id(error.item_uid),
            }
            .into()),
        }
    }
    fn iter_attrs(&mut self) -> Result<impl ExactSizeIterator<Item = (AttrId, ItemAttrValues)>, ItemAttrsIterError> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        match sol.svc.iter_item_attr_vals(&sol.u_data, item_uid) {
            Ok(attr_iter) => Ok(attr_iter.map(|(attr_rid, calc_vals)| {
                (
                    AttrId::from_aid(sol.u_data.r_data.get_attr_by_rid(attr_rid).aid),
                    ItemAttrValues::from_calc_attr_vals(calc_vals),
                )
            })),
            Err(error) => Err(ItemLoadedError {
                item_id: sol.u_data.items.ext_id_by_int_id(error.item_uid),
            }
            .into()),
        }
    }
    fn iter_modifiers(
        &mut self,
    ) -> Result<
        impl ExactSizeIterator<Item = (AttrId, impl ExactSizeIterator<Item = Modification>)>,
        ItemModifiersIterError,
    > {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        match sol.svc.iter_item_mods(&sol.u_data, item_uid) {
            Ok(mods_iter) => Ok(mods_iter.into_iter().map(|(attr_rid, modifications)| {
                (
                    AttrId::from_aid(sol.u_data.r_data.get_attr_by_rid(attr_rid).aid),
                    modifications
                        .into_iter()
                        .map(|calc_mod_info| Modification::from_calc(calc_mod_info, &sol.u_data)),
                )
            })),
            Err(err) => Err(ItemLoadedError {
                item_id: sol.u_data.items.ext_id_by_int_id(err.item_uid),
            }
            .into()),
        }
    }
    fn set_effect_mode(&mut self, effect_id: &EffectId, effect_mode: EffectMode) {
        let item_uid = self.get_uid();
        self.get_sol_mut()
            .internal_set_effect_id_mode(item_uid, effect_id.into_aid(), effect_mode);
    }
    fn set_effect_modes(&mut self, effect_modes: impl IntoIterator<Item = (EffectId, EffectMode)>) {
        let item_uid = self.get_uid();
        self.get_sol_mut().internal_set_effect_id_modes(
            item_uid,
            effect_modes
                .into_iter()
                .map(|(effect_id, effect_mode)| (effect_id.into_aid(), effect_mode)),
        );
    }
    fn stats(&mut self) -> ItemStats<'_> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        sol.reuse.cseq_cache.clear();
        ItemStats::new(sol, item_uid)
    }
}
