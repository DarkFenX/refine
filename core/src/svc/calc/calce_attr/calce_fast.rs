use std::collections::hash_map::Entry;

use super::calce_shared::get_base_attr_value;
use crate::{
    misc::SecZone,
    num::Value,
    rd::{RAttr, RAttrId},
    svc::{
        SvcCtx,
        calc::{Calc, CalcAttrVals, CalcModification, CalcModificationKey, ModAccumFast},
        err::UItemLoadedError,
    },
    ud::{UItem, UItemId},
    util::RMap,
};

impl Calc {
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Thin wrappers around core query methods
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // - Extra value as an option
    pub(crate) fn get_item_attr_oextra(&mut self, ctx: SvcCtx, item_uid: UItemId, attr_rid: RAttrId) -> Option<Value> {
        self.get_item_attr_rfull(ctx, item_uid, attr_rid).ok().map(|v| v.extra)
    }
    // - Optional attribute
    // - Dogma value as an option
    pub(crate) fn get_item_oattr_odogma(
        &mut self,
        ctx: SvcCtx,
        item_uid: UItemId,
        attr_rid: Option<RAttrId>,
    ) -> Option<Value> {
        self.get_item_attr_rfull(ctx, item_uid, attr_rid?).ok().map(|v| v.dogma)
    }
    // - Optional attribute
    // - Extra value as an option
    pub(crate) fn get_item_oattr_oextra(
        &mut self,
        ctx: SvcCtx,
        item_uid: UItemId,
        attr_rid: Option<RAttrId>,
    ) -> Option<Value> {
        self.get_item_attr_rfull(ctx, item_uid, attr_rid?).ok().map(|v| v.extra)
    }
    // - Optional attribute
    // - Fallback in case of missing attribute argument
    // - Dogma value as an option
    pub(crate) fn get_item_oattr_afb_odogma(
        &mut self,
        ctx: SvcCtx,
        item_uid: UItemId,
        attr_rid: Option<RAttrId>,
        fallback: Value,
    ) -> Option<Value> {
        match self.get_item_oattr_rfull(ctx, item_uid, attr_rid) {
            Ok(full) => Some(full.dogma),
            Err(error) => match error {
                GetOAttrError::ItemNotLoaded(_) => None,
                GetOAttrError::NoAttr(_) => Some(fallback),
            },
        }
    }
    // - Optional attribute
    // - Fallback in case of missing attribute argument
    // - Extra value as an option
    pub(crate) fn get_item_oattr_afb_oextra(
        &mut self,
        ctx: SvcCtx,
        item_uid: UItemId,
        attr_rid: Option<RAttrId>,
        fallback: Value,
    ) -> Option<Value> {
        match self.get_item_oattr_rfull(ctx, item_uid, attr_rid) {
            Ok(full) => Some(full.extra),
            Err(error) => match error {
                GetOAttrError::ItemNotLoaded(_) => None,
                GetOAttrError::NoAttr(_) => Some(fallback),
            },
        }
    }
    // - Optional attribute
    // - Fallback for all cases
    // - Extra value
    pub(crate) fn get_item_oattr_ffb_extra(
        &mut self,
        ctx: SvcCtx,
        item_uid: UItemId,
        attr_rid: Option<RAttrId>,
        fallback: Value,
    ) -> Value {
        match self.get_item_oattr_rfull(ctx, item_uid, attr_rid) {
            Ok(full) => full.extra,
            Err(_) => fallback,
        }
    }
    // - Optional item
    // - Optional attribute
    // - Fallback for missing attribute argument
    // - Extra value as an option
    pub(crate) fn get_oitem_oattr_afb_oextra(
        &mut self,
        ctx: SvcCtx,
        item_uid: Option<UItemId>,
        attr_rid: Option<RAttrId>,
        fallback: Value,
    ) -> Option<Value> {
        match self.get_item_oattr_rfull(ctx, item_uid?, attr_rid) {
            Ok(full) => Some(full.extra),
            Err(error) => match error {
                GetOAttrError::ItemNotLoaded(_) => None,
                GetOAttrError::NoAttr(_) => Some(fallback),
            },
        }
    }
    // - Optional item
    // - Optional attribute
    // - Fallback for all cases
    // - Extra value
    pub(crate) fn get_oitem_oattr_ffb_extra(
        &mut self,
        ctx: SvcCtx,
        item_uid: Option<UItemId>,
        attr_rid: Option<RAttrId>,
        fallback: Value,
    ) -> Value {
        match (item_uid, attr_rid) {
            (Some(item_uid), Some(attr_rid)) => match self.get_item_attr_rfull(ctx, item_uid, attr_rid) {
                Ok(full) => full.extra,
                Err(_) => fallback,
            },
            _ => fallback,
        }
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Core query methods
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // TODO: make code below less duplicated
    pub(crate) fn get_item_attr_rfull(
        &mut self,
        ctx: SvcCtx,
        item_uid: UItemId,
        attr_rid: RAttrId,
    ) -> Result<CalcAttrVals, UItemLoadedError> {
        // Try accessing cached value
        let item_attr_data = self.get_item_data_with_err(item_uid)?;
        if let Some(attr_entry) = item_attr_data.get(&attr_rid)
            && let Some(cval) = attr_entry.value
        {
            let cval = match &attr_entry.postprocs {
                Some(postprocs) => {
                    let pp_fn = postprocs.fast;
                    pp_fn(self, ctx, item_uid, cval)
                }
                None => cval,
            };
            return Ok(cval);
        }
        // If it is not cached, calculate and cache it
        let mut cval = self.calc_item_attr_val(ctx, item_uid, attr_rid);
        let item_attr_data = self.attrs.get_item_attr_data_mut(&item_uid).unwrap();
        if let Some(postprocs) = item_attr_data.set_value_and_get_pp(attr_rid, cval) {
            let pp_fn = postprocs.fast;
            cval = pp_fn(self, ctx, item_uid, cval);
        }
        Ok(cval)
    }
    fn get_item_oattr_rfull(
        &mut self,
        ctx: SvcCtx,
        item_uid: UItemId,
        attr_rid: Option<RAttrId>,
    ) -> Result<CalcAttrVals, GetOAttrError> {
        // Try accessing cached value
        let item_attr_data = self.get_item_data_with_err(item_uid)?;
        let attr_rid = match attr_rid {
            Some(attr_rid) => attr_rid,
            None => return Err(NoAttrError {}.into()),
        };
        if let Some(attr_entry) = item_attr_data.get(&attr_rid)
            && let Some(cval) = attr_entry.value
        {
            let cval = match &attr_entry.postprocs {
                Some(postprocs) => {
                    let pp_fn = postprocs.fast;
                    pp_fn(self, ctx, item_uid, cval)
                }
                None => cval,
            };
            return Ok(cval);
        }
        // If it is not cached, calculate and cache it
        let mut cval = self.calc_item_attr_val(ctx, item_uid, attr_rid);
        let item_attr_data = self.attrs.get_item_attr_data_mut(&item_uid).unwrap();
        if let Some(postprocs) = item_attr_data.set_value_and_get_pp(attr_rid, cval) {
            let pp_fn = postprocs.fast;
            cval = pp_fn(self, ctx, item_uid, cval);
        }
        Ok(cval)
    }
    pub(in crate::svc::calc) fn get_item_oattr_ofull_nopp(
        &mut self,
        ctx: SvcCtx,
        item_uid: UItemId,
        attr_rid: Option<RAttrId>,
    ) -> Option<CalcAttrVals> {
        let attr_rid = attr_rid?;
        let item_attr_data = self.attrs.get_item_attr_data(&item_uid)?;
        if let Some(attr_entry) = item_attr_data.get(&attr_rid)
            && let Some(cval) = attr_entry.value
        {
            return Some(cval);
        }
        let cval = self.calc_item_attr_val(ctx, item_uid, attr_rid);
        self.attrs
            .get_item_attr_data_mut(&item_uid)
            .unwrap()
            .set_value_and_get_pp(attr_rid, cval);
        Some(cval)
    }
    pub(in crate::svc) fn iter_item_attrs_rfull(
        &mut self,
        ctx: SvcCtx,
        item_uid: UItemId,
    ) -> Result<impl ExactSizeIterator<Item = (RAttrId, CalcAttrVals)> + use<>, UItemLoadedError> {
        // Items can have attributes which are not defined on the original EVE item. This happens
        // when something requested an attr value, and it was calculated using base attribute value.
        // Here, we get already calculated attributes, which includes attributes absent on the EVE
        // item
        let item_attr_data = self.get_item_data_with_err(item_uid)?;
        let base_attrs = ctx.u_data.items.get(item_uid).get_attrs().unwrap();
        let mut cval_map = RMap::with_capacity(item_attr_data.len().max(base_attrs.len()));
        let mut attrs_with_pps = Vec::new();
        for (&attr_rid, attr_entry) in item_attr_data.iter() {
            if let Some(cval) = attr_entry.value {
                cval_map.insert(attr_rid, cval);
                if let Some(postprocs) = &attr_entry.postprocs {
                    attrs_with_pps.push((attr_rid, postprocs.fast));
                }
            }
        }
        for (attr_rid, pp_fn) in attrs_with_pps {
            if let Entry::Occupied(mut entry) = cval_map.entry(attr_rid) {
                let cval = *entry.get();
                let cval = pp_fn(self, ctx, item_uid, cval);
                entry.insert(cval);
            }
        }
        // Calculate & store attributes which are not calculated yet, but are defined on the EVE
        // item. Attribute fetcher handles postprocessing on its own, so no need to do it here
        for &attr_rid in base_attrs.keys() {
            if let Entry::Vacant(entry) = cval_map.entry(attr_rid) {
                match self.get_item_attr_rfull(ctx, item_uid, attr_rid) {
                    Ok(v) => entry.insert(v),
                    _ => continue,
                };
            }
        }
        Ok(cval_map.into_iter())
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Private methods
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn iter_modifications(
        &mut self,
        ctx: SvcCtx,
        item_uid: &UItemId,
        item: &UItem,
        attr_rid: RAttrId,
    ) -> impl Iterator<Item = CalcModification> {
        let mut mods = RMap::new();
        for cmod in self
            .std
            .get_mods_for_affectee(item_uid, item, attr_rid, &ctx.u_data.fits)
            .iter()
        {
            let val = match cmod.raw.get_mod_val(self, ctx) {
                Some(val) => val,
                None => continue,
            };
            let affector_item = ctx.u_data.items.get(cmod.raw.affector_espec.item_uid);
            let affector_item_cat_id = affector_item.get_category_id().unwrap();
            let mod_key = CalcModificationKey::from_cmod(cmod);
            let modification = CalcModification {
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
    fn calc_item_attr_val(&mut self, ctx: SvcCtx, item_uid: UItemId, attr_rid: RAttrId) -> CalcAttrVals {
        let item = ctx.u_data.items.get(item_uid);
        let attr = ctx.u_data.src.get_attr_by_rid(attr_rid);
        let base_val = self.calc_item_base_attr_value(ctx, item_uid, item, attr);
        let mut accumulator = ModAccumFast::new();
        for modification in self.iter_modifications(ctx, &item_uid, item, attr_rid) {
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
        if let Some(limiter_attr_rid) = attr.min_attr_rid
            && let Ok(limiter_cval) = self.get_item_attr_rfull(ctx, item_uid, limiter_attr_rid)
        {
            self.deps.add_anonymous(item_uid, limiter_attr_rid, attr_rid);
            dogma_val = Value::max(dogma_val, limiter_cval.dogma);
        }
        // Upper value limit
        if let Some(limiter_attr_rid) = attr.max_attr_rid
            && let Ok(limiter_cval) = self.get_item_attr_rfull(ctx, item_uid, limiter_attr_rid)
        {
            self.deps.add_anonymous(item_uid, limiter_attr_rid, attr_rid);
            dogma_val = Value::min(dogma_val, limiter_cval.dogma);
        }
        if ctx.ac().limited_precision.contains(&attr_rid) {
            dogma_val.round_to_digits(2);
        }
        // Post-dogma calculations
        let extra_val = accumulator.apply_extra_mods(dogma_val, attr.hig);
        CalcAttrVals {
            base: base_val,
            dogma: dogma_val,
            extra: extra_val,
        }
    }
    fn calc_item_base_attr_value(&mut self, ctx: SvcCtx, item_uid: UItemId, item: &UItem, attr: &RAttr) -> Value {
        let attr_consts = ctx.ac();
        // Security modifier is a special case - it takes modified value of another attribute as its
        // own base
        if let Some(sec_zone_attr_rid) = attr_consts.security_modifier
            && attr.rid == sec_zone_attr_rid
        {
            let security_attr_rid = match ctx.u_data.sec_zone {
                SecZone::HiSec(_) => attr_consts.hisec_modifier,
                SecZone::LowSec(_) => attr_consts.lowsec_modifier,
                SecZone::NullSec | SecZone::WSpace | SecZone::Hazard => attr_consts.nullsec_modifier,
            };
            if let Some(security_attr_rid) = security_attr_rid
                && let Ok(security_full_val) = self.get_item_attr_rfull(ctx, item_uid, security_attr_rid)
            {
                // Ensure that change in any a security-specific attribute value triggers
                // recalculation of generic security attribute value
                self.deps.add_anonymous(item_uid, security_attr_rid, attr.rid);
                return security_full_val.dogma;
            }
        }
        get_base_attr_value(item, attr)
    }
}

#[derive(thiserror::Error, Debug)]
#[error("no attribute in request")]
struct NoAttrError {}

#[derive(thiserror::Error, Debug)]
enum GetOAttrError {
    #[error("{0}")]
    ItemNotLoaded(#[from] UItemLoadedError),
    #[error("{0}")]
    NoAttr(#[from] NoAttrError),
}
