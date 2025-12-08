use std::collections::HashMap;

use super::shared::is_attr_flag_set;
use crate::{
    def::{ItemId, OF},
    misc::EffectSpec,
    rd::RAttrKey,
    svc::{SvcCtx, calc::Calc, eff_funcs, vast::VastFitData},
    ud::UItemKey,
    util::{RMapRSet, RSet},
};

pub struct ValProjImmunityFail {
    /// Map between projecting item IDs and targets they can't be projected to.
    pub items: HashMap<ItemId, Vec<ItemId>>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_assist_immunity_fast(
        &self,
        kfs: &RSet<UItemKey>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> bool {
        validate_fast(kfs, ctx, calc, &self.blockable_assistance, ctx.ac().disallow_assistance)
    }
    pub(in crate::svc::vast) fn validate_offense_immunity_fast(
        &self,
        kfs: &RSet<UItemKey>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> bool {
        validate_fast(
            kfs,
            ctx,
            calc,
            &self.blockable_offense,
            ctx.ac().disallow_offensive_modifiers,
        )
    }
    pub(in crate::svc::vast) fn validate_resist_immunity_fast(
        &self,
        kfs: &RSet<UItemKey>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> bool {
        for (projectee_aspec, mut projector_especs) in self.resist_immunity.iter() {
            if eff_funcs::get_resist_mult_by_projectee_aspec(ctx, calc, projectee_aspec) == Some(OF(0.0)) {
                match kfs.is_empty() {
                    true => return false,
                    false => {
                        if !projector_especs.all(|v| kfs.contains(&v.item_key)) {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_assist_immunity_verbose(
        &self,
        kfs: &RSet<UItemKey>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> Option<ValProjImmunityFail> {
        validate_verbose(kfs, ctx, calc, &self.blockable_assistance, ctx.ac().disallow_assistance)
    }
    pub(in crate::svc::vast) fn validate_offense_immunity_verbose(
        &self,
        kfs: &RSet<UItemKey>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> Option<ValProjImmunityFail> {
        validate_verbose(
            kfs,
            ctx,
            calc,
            &self.blockable_offense,
            ctx.ac().disallow_offensive_modifiers,
        )
    }
    pub(in crate::svc::vast) fn validate_resist_immunity_verbose(
        &self,
        kfs: &RSet<UItemKey>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> Option<ValProjImmunityFail> {
        let mut items = HashMap::new();
        for (projectee_aspec, projector_especs) in self.resist_immunity.iter() {
            if eff_funcs::get_resist_mult_by_projectee_aspec(ctx, calc, projectee_aspec) == Some(OF(0.0))
                && projector_especs.len() > 0
            {
                let projectee_item_id = ctx.u_data.items.id_by_key(projectee_aspec.item_key);
                for projector_espec in projector_especs {
                    if kfs.contains(&projector_espec.item_key) {
                        continue;
                    }
                    let projector_item_id = ctx.u_data.items.id_by_key(projector_espec.item_key);
                    let projectee_item_ids = items.entry(projector_item_id).or_insert_with(Vec::new);
                    if !projectee_item_ids.contains(&projectee_item_id) {
                        projectee_item_ids.push(projectee_item_id)
                    }
                }
            }
        }
        match items.is_empty() {
            true => None,
            false => Some(ValProjImmunityFail { items }),
        }
    }
}

fn validate_fast(
    kfs: &RSet<UItemKey>,
    ctx: SvcCtx,
    calc: &mut Calc,
    blockable: &RMapRSet<UItemKey, EffectSpec>,
    attr_key: Option<RAttrKey>,
) -> bool {
    let attr_key = match attr_key {
        Some(attr_key) => attr_key,
        None => return true,
    };
    for (&projectee_key, mut projector_especs) in blockable.iter() {
        if is_attr_flag_set(ctx, calc, projectee_key, attr_key) {
            match kfs.is_empty() {
                true => return false,
                false => {
                    if !projector_especs.all(|v| kfs.contains(&v.item_key)) {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn validate_verbose(
    kfs: &RSet<UItemKey>,
    ctx: SvcCtx,
    calc: &mut Calc,
    blockable: &RMapRSet<UItemKey, EffectSpec>,
    attr_key: Option<RAttrKey>,
) -> Option<ValProjImmunityFail> {
    let attr_key = attr_key?;
    let mut items = HashMap::new();
    for (&projectee_key, projector_especs) in blockable.iter() {
        if is_attr_flag_set(ctx, calc, projectee_key, attr_key) && projector_especs.len() > 0 {
            let projectee_item_id = ctx.u_data.items.id_by_key(projectee_key);
            for projector_espec in projector_especs {
                if kfs.contains(&projector_espec.item_key) {
                    continue;
                }
                let projector_item_id = ctx.u_data.items.id_by_key(projector_espec.item_key);
                let projectee_item_ids = items.entry(projector_item_id).or_insert_with(Vec::new);
                if !projectee_item_ids.contains(&projectee_item_id) {
                    projectee_item_ids.push(projectee_item_id)
                }
            }
        }
    }
    match items.is_empty() {
        true => None,
        false => Some(ValProjImmunityFail { items }),
    }
}
