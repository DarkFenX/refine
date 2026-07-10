use crate::{
    ad::ABuffId,
    misc::EffectSpec,
    num::Value,
    rd::{RAttrId, RBuff, REffect, REffectBuffScope, REffectId, REffectModStrength},
    svc::{
        SvcCtx,
        calc::{Calc, RawModifier},
    },
    ud::{UData, UItem, UItemId},
};

impl Calc {
    pub(super) fn generate_mods_for_effect(
        &mut self,
        reuse_rmods: &mut Vec<RawModifier>,
        ctx: SvcCtx,
        item_uid: UItemId,
        item: &UItem,
        effect: &REffect,
    ) {
        reuse_rmods.clear();
        // Regular modifiers
        for effect_mod in effect.modifiers.iter() {
            match RawModifier::try_from_effect_mod(ctx.u_data, item_uid, item, effect, effect_mod) {
                Some(raw_mod) => reuse_rmods.push(raw_mod),
                None => continue,
            };
        }
        // Buffs
        if let Some(effect_buff) = &effect.buff {
            // Buffs which are partially defined and rely on on-item attributes to complete
            // definition
            if let Some(buff_attr_merge) = &effect_buff.attr_merge {
                for (buff_type_attr_rid, buff_str_attr_rid) in ctx.ac().buff_merge_ids_strs.iter() {
                    if let Ok(buff_id_cval) = self.get_item_attr_rfull(ctx, item_uid, *buff_type_attr_rid) {
                        let Some(buff_aid) = ABuffId::try_eve_from_f64_rounded(buff_id_cval.extra.into_f64()) else {
                            continue;
                        };
                        let Some(buff) = ctx.u_data.r_data.get_buff_by_aid(&buff_aid) else {
                            continue;
                        };
                        add_buff_mods_with_attr(
                            ctx.u_data,
                            reuse_rmods,
                            item_uid,
                            item,
                            effect,
                            buff,
                            &buff_attr_merge.scope,
                            Some(*buff_type_attr_rid),
                            *buff_str_attr_rid,
                        );
                    }
                }
            }
            // Fully defined buffs
            for buff_full in effect_buff.full.iter() {
                match buff_full.strength {
                    REffectModStrength::Attr(buff_str_attr_rid) => {
                        let buff = ctx.u_data.r_data.get_buff_by_rid(buff_full.buff_rid);
                        add_buff_mods_with_attr(
                            ctx.u_data,
                            reuse_rmods,
                            item_uid,
                            item,
                            effect,
                            buff,
                            &buff_full.scope,
                            None,
                            buff_str_attr_rid,
                        )
                    }
                    REffectModStrength::Hardcoded(buff_str) => {
                        let buff = ctx.u_data.r_data.get_buff_by_rid(buff_full.buff_rid);
                        add_buff_mods_with_hardcoded(
                            ctx.u_data,
                            reuse_rmods,
                            item_uid,
                            item,
                            effect,
                            buff,
                            &buff_full.scope,
                            buff_str,
                        )
                    }
                }
            }
        }
        // Custom modifiers
        if let Some(custom_mod) = effect.calc_custom_mod
            && let Some(rmod) = custom_mod.make_rmod(
                ctx.u_data.r_data.get_attr_consts(),
                EffectSpec::new(item_uid, effect.rid),
                effect.state,
            )
        {
            reuse_rmods.push(rmod);
        }
    }
    pub(super) fn generate_dependent_buff_mods<'a>(
        &mut self,
        ctx: SvcCtx,
        item_uid: UItemId,
        item: &UItem,
        effect_rids: impl Iterator<Item = &'a REffectId>,
        buff_type_attr_rid: RAttrId,
    ) -> Vec<RawModifier> {
        let mut rmods = Vec::new();
        let Some(buff_str_attr_rid) = ctx.u_data.r_data.get_attr_by_rid(buff_type_attr_rid).buff_str_attr_rid else {
            return rmods;
        };
        for &effect_rid in effect_rids {
            let effect = ctx.u_data.r_data.get_effect_by_rid(effect_rid);
            if let Some(effect_buff) = &effect.buff
                && let Some(buff_attr_merge) = &effect_buff.attr_merge
                && let Ok(buff_id_cval) = self.get_item_attr_rfull(ctx, item_uid, buff_type_attr_rid)
            {
                let Some(buff_aid) = ABuffId::try_eve_from_f64_rounded(buff_id_cval.extra.into_f64()) else {
                    continue;
                };
                let Some(buff) = ctx.u_data.r_data.get_buff_by_aid(&buff_aid) else {
                    continue;
                };
                add_buff_mods_with_attr(
                    ctx.u_data,
                    &mut rmods,
                    item_uid,
                    item,
                    effect,
                    buff,
                    &buff_attr_merge.scope,
                    Some(buff_type_attr_rid),
                    buff_str_attr_rid,
                );
            }
        }
        rmods
    }
}

fn add_buff_mods_with_attr(
    u_data: &UData,
    rmods: &mut Vec<RawModifier>,
    item_uid: UItemId,
    item: &UItem,
    effect: &REffect,
    buff: &RBuff,
    buff_scope: &REffectBuffScope,
    buff_type_attr_rid: Option<RAttrId>,
    buff_str_attr_rid: RAttrId,
) {
    for buff_mod in buff.mods.iter() {
        let Some(rmod) = RawModifier::try_from_buff_with_attr(
            u_data,
            item_uid,
            item,
            effect,
            buff,
            buff_scope,
            buff_mod,
            buff_type_attr_rid,
            buff_str_attr_rid,
        ) else {
            continue;
        };
        rmods.push(rmod);
    }
}

fn add_buff_mods_with_hardcoded(
    u_data: &UData,
    rmods: &mut Vec<RawModifier>,
    item_uid: UItemId,
    item: &UItem,
    effect: &REffect,
    buff: &RBuff,
    buff_scope: &REffectBuffScope,
    buff_str: Value,
) {
    for buff_mod in buff.mods.iter() {
        if let Some(rmod) = RawModifier::try_from_buff_with_hardcoded(
            u_data, item_uid, item, effect, buff, buff_scope, buff_mod, buff_str,
        ) {
            rmods.push(rmod);
        }
    }
}
