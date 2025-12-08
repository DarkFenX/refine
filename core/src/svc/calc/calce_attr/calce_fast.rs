use std::collections::hash_map::Entry;

use itertools::Itertools;

use super::calce_shared::get_base_attr_value;
use crate::{
    def::AttrVal,
    misc::SecZone,
    rd::{RAttr, RAttrKey},
    svc::{
        SvcCtx,
        calc::{Calc, CalcAttrVal, ModAccumFast, Modification, ModificationKey},
        err::KeyedItemLoadedError,
    },
    ud::{UItem, UItemKey},
    util::{RMap, round},
};

impl Calc {
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Thin wrappers around core query methods
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // - Extra value as an option
    pub(crate) fn get_item_attr_odogma(
        &mut self,
        ctx: SvcCtx,
        item_key: UItemKey,
        attr_key: RAttrKey,
    ) -> Option<AttrVal> {
        match self.get_item_attr_rfull(ctx, item_key, attr_key) {
            Ok(full) => Some(full.dogma),
            Err(_) => None,
        }
    }
    // - Extra value as a result
    pub(crate) fn get_item_attr_rextra(
        &mut self,
        ctx: SvcCtx,
        item_key: UItemKey,
        attr_key: RAttrKey,
    ) -> Result<AttrVal, KeyedItemLoadedError> {
        match self.get_item_attr_rfull(ctx, item_key, attr_key) {
            Ok(full) => Ok(full.extra),
            Err(err) => Err(err),
        }
    }
    // - Extra value as an option
    pub(crate) fn get_item_attr_oextra(
        &mut self,
        ctx: SvcCtx,
        item_key: UItemKey,
        attr_key: RAttrKey,
    ) -> Option<AttrVal> {
        match self.get_item_attr_rfull(ctx, item_key, attr_key) {
            Ok(full) => Some(full.extra),
            Err(_) => None,
        }
    }
    // - Optional attribute
    // - Dogma value as an option
    pub(crate) fn get_item_oattr_odogma(
        &mut self,
        ctx: SvcCtx,
        item_key: UItemKey,
        attr_key: Option<RAttrKey>,
    ) -> Option<AttrVal> {
        match attr_key {
            Some(attr_key) => match self.get_item_attr_rfull(ctx, item_key, attr_key) {
                Ok(full) => Some(full.dogma),
                Err(_) => None,
            },
            None => None,
        }
    }
    // - Optional attribute
    // - Extra value as an option
    pub(crate) fn get_item_oattr_oextra(
        &mut self,
        ctx: SvcCtx,
        item_key: UItemKey,
        attr_key: Option<RAttrKey>,
    ) -> Option<AttrVal> {
        match attr_key {
            Some(attr_key) => match self.get_item_attr_rfull(ctx, item_key, attr_key) {
                Ok(full) => Some(full.extra),
                Err(_) => None,
            },
            None => None,
        }
    }
    // - Optional attribute
    // - Fallback in case of missing attribute argument
    // - Dogma value as an option
    pub(in crate::svc::calc) fn get_item_oattr_afb_odogma(
        &mut self,
        ctx: SvcCtx,
        item_key: UItemKey,
        attr_key: Option<RAttrKey>,
        fallback: AttrVal,
    ) -> Option<AttrVal> {
        match attr_key {
            Some(attr_key) => match self.get_item_attr_rfull(ctx, item_key, attr_key) {
                Ok(full) => Some(full.dogma),
                Err(_) => None,
            },
            None => Some(fallback),
        }
    }
    // - Optional attribute
    // - Fallback in case of missing attribute argument
    // - Extra value as an option
    pub(crate) fn get_item_oattr_afb_oextra(
        &mut self,
        ctx: SvcCtx,
        item_key: UItemKey,
        attr_key: Option<RAttrKey>,
        fallback: AttrVal,
    ) -> Option<AttrVal> {
        match attr_key {
            Some(attr_key) => match self.get_item_attr_rfull(ctx, item_key, attr_key) {
                Ok(full) => Some(full.extra),
                Err(_) => None,
            },
            None => Some(fallback),
        }
    }
    // - Optional attribute
    // - Fallback for all cases
    // - Extra value
    pub(crate) fn get_item_oattr_ffb_extra(
        &mut self,
        ctx: SvcCtx,
        item_key: UItemKey,
        attr_key: Option<RAttrKey>,
        fallback: AttrVal,
    ) -> AttrVal {
        match attr_key {
            Some(attr_key) => match self.get_item_attr_rfull(ctx, item_key, attr_key) {
                Ok(full) => full.extra,
                Err(_) => fallback,
            },
            None => fallback,
        }
    }
    // - Optional item
    // - Extra value as an option
    pub(crate) fn get_oitem_attr_oextra(
        &mut self,
        ctx: SvcCtx,
        item_key: Option<UItemKey>,
        attr_key: RAttrKey,
    ) -> Option<AttrVal> {
        match item_key {
            Some(item_key) => match self.get_item_attr_rfull(ctx, item_key, attr_key) {
                Ok(full) => Some(full.extra),
                Err(_) => None,
            },
            None => None,
        }
    }
    // - Optional item
    // - Optional attribute
    // - Fallback for missing attribute argument
    // - Extra value as an option
    pub(crate) fn get_oitem_oattr_afb_oextra(
        &mut self,
        ctx: SvcCtx,
        item_key: Option<UItemKey>,
        attr_key: Option<UItemKey>,
        fallback: AttrVal,
    ) -> Option<AttrVal> {
        match item_key {
            Some(item_key) => match attr_key {
                Some(attr_key) => match self.get_item_attr_rfull(ctx, item_key, attr_key) {
                    Ok(full) => Some(full.extra),
                    Err(_) => None,
                },
                None => Some(fallback),
            },
            None => None,
        }
    }
    // - Optional item
    // - Optional attribute
    // - Fallback for all cases
    // - Extra value
    pub(crate) fn get_oitem_oattr_ffb_extra(
        &mut self,
        ctx: SvcCtx,
        item_key: Option<UItemKey>,
        attr_key: Option<UItemKey>,
        fallback: AttrVal,
    ) -> AttrVal {
        match (item_key, attr_key) {
            (Some(item_key), Some(attr_key)) => match self.get_item_attr_rfull(ctx, item_key, attr_key) {
                Ok(full) => full.extra,
                Err(_) => fallback,
            },
            _ => fallback,
        }
    }
    // - Optional attribute
    // - Fallback in case of missing attribute argument
    // - Full value as an option
    // - No postprocessing
    pub(in crate::svc::calc) fn get_item_oattr_afb_ofull_nopp(
        &mut self,
        ctx: SvcCtx,
        item_key: UItemKey,
        attr_key: Option<RAttrKey>,
        fallback: CalcAttrVal,
    ) -> Option<CalcAttrVal> {
        match attr_key {
            Some(attr_key) => match self.get_item_attr_rfull_nopp(ctx, item_key, attr_key) {
                Ok(full) => Some(full),
                Err(_) => None,
            },
            None => Some(fallback),
        }
    }
    // - Optional attribute
    // - Fallback for all cases
    // - Full value
    // - No postprocessing
    pub(in crate::svc::calc) fn get_item_oattr_ffb_full_nopp(
        &mut self,
        ctx: SvcCtx,
        item_key: UItemKey,
        attr_key: Option<RAttrKey>,
        fallback: CalcAttrVal,
    ) -> CalcAttrVal {
        match attr_key {
            Some(attr_key) => self
                .get_item_attr_rfull_nopp(ctx, item_key, attr_key)
                .unwrap_or(fallback),
            None => fallback,
        }
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Core query methods
    ////////////////////////////////////////////////////////////////////////////////////////////////
    pub(crate) fn get_item_attr_rfull(
        &mut self,
        ctx: SvcCtx,
        item_key: UItemKey,
        attr_key: RAttrKey,
    ) -> Result<CalcAttrVal, KeyedItemLoadedError> {
        // Try accessing cached value
        let item_attr_data = match self.attrs.get_item_attr_data(&item_key) {
            Some(item_attr_data) => item_attr_data,
            None => {
                return Err(KeyedItemLoadedError { item_key });
            }
        };
        if let Some(cval) = item_attr_data.values.get(&attr_key) {
            return Ok(match item_attr_data.postprocs.get(&attr_key) {
                Some(postprocs) => {
                    let pp_fn = postprocs.fast;
                    pp_fn(self, ctx, item_key, *cval)
                }
                None => *cval,
            });
        }
        // If it is not cached, calculate and cache it
        let mut cval = self.calc_item_attr_val(ctx, item_key, attr_key);
        let item_attr_data = self.attrs.get_item_attr_data_mut(&item_key).unwrap();
        item_attr_data.values.insert(attr_key, cval);
        if let Some(postprocs) = item_attr_data.postprocs.get(&attr_key) {
            let pp_fn = postprocs.fast;
            cval = pp_fn(self, ctx, item_key, cval);
        }
        Ok(cval)
    }
    fn get_item_attr_rfull_nopp(
        &mut self,
        ctx: SvcCtx,
        item_key: UItemKey,
        attr_key: RAttrKey,
    ) -> Result<CalcAttrVal, KeyedItemLoadedError> {
        let item_attr_data = match self.attrs.get_item_attr_data(&item_key) {
            Some(item_attr_data) => item_attr_data,
            None => {
                return Err(KeyedItemLoadedError { item_key });
            }
        };
        if let Some(cval) = item_attr_data.values.get(&attr_key) {
            return Ok(*cval);
        };
        let cval = self.calc_item_attr_val(ctx, item_key, attr_key);
        self.attrs
            .get_item_attr_data_mut(&item_key)
            .unwrap()
            .values
            .insert(attr_key, cval);
        Ok(cval)
    }
    pub(in crate::svc) fn iter_item_attr_rfull(
        &mut self,
        ctx: SvcCtx,
        item_key: UItemKey,
    ) -> Result<impl ExactSizeIterator<Item = (RAttrKey, CalcAttrVal)> + use<>, KeyedItemLoadedError> {
        let item = ctx.u_data.items.get(item_key);
        // SolItem can have attributes which are not defined on the original EVE item. This happens
        // when something requested an attr value, and it was calculated using base attribute value.
        // Here, we get already calculated attributes, which includes attributes absent on the EVE
        // item
        let item_attr_data = match self.attrs.get_item_attr_data(&item_key) {
            Some(item_attr_data) => item_attr_data,
            None => return Err(KeyedItemLoadedError { item_key }),
        };
        let pp_attr_keys = item_attr_data.postprocs.keys().copied().collect_vec();
        let mut cvals = item_attr_data.values.clone();
        // Calculate & store attributes which are not calculated yet, but are defined on the EVE
        // item
        for &attr_key in item.get_attrs().unwrap().keys() {
            if let Entry::Vacant(entry) = cvals.entry(attr_key) {
                match self.get_item_attr_rfull(ctx, item_key, attr_key) {
                    Ok(v) => entry.insert(v),
                    _ => continue,
                };
            }
        }
        for pp_attr_key in pp_attr_keys {
            if let Some(cval) = cvals.get(&pp_attr_key) {
                let pp_fn = self
                    .attrs
                    .get_item_attr_data(&item_key)
                    .unwrap()
                    .postprocs
                    .get(&pp_attr_key)
                    .unwrap()
                    .fast;
                let cval = pp_fn(self, ctx, item_key, *cval);
                cvals.insert(pp_attr_key, cval);
            }
        }
        Ok(cvals.into_iter())
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Private methods
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn iter_modifications(
        &mut self,
        ctx: SvcCtx,
        item_key: &UItemKey,
        item: &UItem,
        attr_key: RAttrKey,
    ) -> impl Iterator<Item = Modification> {
        let mut mods = RMap::new();
        for cmod in self
            .std
            .get_mods_for_affectee(item_key, item, attr_key, &ctx.u_data.fits)
            .iter()
        {
            let val = match cmod.raw.get_mod_val(self, ctx) {
                Some(val) => val,
                None => continue,
            };
            let affector_item = ctx.u_data.items.get(cmod.raw.affector_espec.item_key);
            let affector_item_cat_id = affector_item.get_category_id().unwrap();
            let mod_key = ModificationKey::from(cmod);
            let modification = Modification {
                op: cmod.raw.op,
                val,
                proj_mult: self.calc_proj_mult(ctx, cmod),
                res_mult: self.calc_resist_mult(ctx, cmod),
                aggr_mode: cmod.raw.aggr_mode,
                affector_item_cat_id,
            };
            mods.insert(mod_key, modification);
        }
        mods.into_values()
    }
    fn calc_item_attr_val(&mut self, ctx: SvcCtx, item_key: UItemKey, attr_key: RAttrKey) -> CalcAttrVal {
        let item = ctx.u_data.items.get(item_key);
        let attr = ctx.u_data.src.get_attr(attr_key);
        let base_val = self.calc_item_base_attr_value(ctx, item_key, item, attr);
        let mut accumulator = ModAccumFast::new();
        for modification in self.iter_modifications(ctx, &item_key, item, attr_key) {
            accumulator.add_val(
                modification.val,
                modification.proj_mult,
                modification.res_mult,
                &modification.op,
                attr.penalizable,
                &modification.affector_item_cat_id,
                &modification.aggr_mode,
            );
        }
        let mut dogma_val = accumulator.apply_dogma_mods(base_val, attr.hig);
        // Lower value limit
        if let Some(limiter_attr_key) = attr.min_attr_key
            && let Ok(limiter_cval) = self.get_item_attr_rfull(ctx, item_key, limiter_attr_key)
        {
            self.deps.add_anonymous(item_key, limiter_attr_key, attr_key);
            dogma_val = AttrVal::max(dogma_val, limiter_cval.dogma);
        }
        // Upper value limit
        if let Some(limiter_attr_key) = attr.max_attr_key
            && let Ok(limiter_cval) = self.get_item_attr_rfull(ctx, item_key, limiter_attr_key)
        {
            self.deps.add_anonymous(item_key, limiter_attr_key, attr_key);
            dogma_val = AttrVal::min(dogma_val, limiter_cval.dogma);
        }
        if ctx.ac().limited_precision.contains(&attr_key) {
            dogma_val = round(dogma_val, 2);
        }
        // Post-dogma calculations
        let extra_val = accumulator.apply_extra_mods(dogma_val, attr.hig);
        CalcAttrVal {
            base: base_val,
            dogma: dogma_val,
            extra: extra_val,
        }
    }
    fn calc_item_base_attr_value(&mut self, ctx: SvcCtx, item_key: UItemKey, item: &UItem, attr: &RAttr) -> AttrVal {
        let attr_consts = ctx.ac();
        // Security modifier is a special case - it takes modified value of another attribute as its
        // own base
        if let Some(sec_zone_attr_key) = attr_consts.security_modifier
            && attr.key == sec_zone_attr_key
        {
            let security_attr_key = match ctx.u_data.sec_zone {
                SecZone::HiSec(_) => attr_consts.hisec_modifier,
                SecZone::LowSec(_) => attr_consts.lowsec_modifier,
                SecZone::NullSec | SecZone::WSpace | SecZone::Hazard => attr_consts.nullsec_modifier,
            };
            if let Some(security_attr_key) = security_attr_key
                && let Ok(security_full_val) = self.get_item_attr_rfull(ctx, item_key, security_attr_key)
            {
                // Ensure that change in any a security-specific attribute value triggers
                // recalculation of generic security attribute value
                self.deps.add_anonymous(item_key, security_attr_key, attr.key);
                return security_full_val.dogma;
            }
        }
        get_base_attr_value(item, attr)
    }
}
