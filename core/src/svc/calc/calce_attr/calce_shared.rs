use crate::{
    ac, ad,
    def::{AttrVal, OF},
    misc::AttrSpec,
    svc::{
        SvcCtx,
        calc::{Calc, Context, CtxModifier, ModifierKind},
        get_resist_mult_val_by_projectee_aspec,
    },
    uad::UadItem,
};

pub(super) const LIMITED_PRECISION_A_ATTR_IDS: [ad::AAttrId; 4] = [
    ac::attrs::CPU,
    ac::attrs::POWER,
    ac::attrs::CPU_OUTPUT,
    ac::attrs::POWER_OUTPUT,
];

impl Calc {
    pub(super) fn calc_resist_mult(&mut self, ctx: SvcCtx, cmod: &CtxModifier) -> Option<AttrVal> {
        // Only buffs and targeted modifiers can be resisted
        if !matches!(cmod.raw.kind, ModifierKind::Buff | ModifierKind::Targeted) {
            return None;
        }
        let resist_a_attr_id = cmod.raw.resist_a_attr_id?;
        let projectee_key = match cmod.ctx {
            Context::Item(projectee_key) => projectee_key,
            _ => return None,
        };
        let resist =
            get_resist_mult_val_by_projectee_aspec(ctx, self, &AttrSpec::new(projectee_key, resist_a_attr_id))?;
        Some(resist)
    }
    pub(super) fn calc_proj_mult(&mut self, ctx: SvcCtx, cmod: &CtxModifier) -> Option<AttrVal> {
        let projectee_key = match cmod.ctx {
            Context::Item(projectee_key) => projectee_key,
            _ => return None,
        };
        let proj_mult_getter = cmod.raw.proj_mult_getter?;
        let a_effect = ctx.uad.src.get_a_effect(&cmod.raw.affector_espec.a_effect_id)?;
        let prange = ctx.eprojs.get_range(cmod.raw.affector_espec, projectee_key)?;
        Some(proj_mult_getter(
            ctx,
            self,
            cmod.raw.affector_espec.item_key,
            &a_effect.ae,
            prange,
        ))
    }
}

pub(super) fn get_a_attr(a_attr_id: ad::AAttrId) -> ad::AAttr {
    ad::AAttr {
        id: a_attr_id,
        penalizable: false,
        hig: true,
        def_val: OF(0.0),
        min_attr_id: None,
        max_attr_id: None,
    }
}

pub(super) fn get_base_attr_value(item: &UadItem, a_attr: &ad::AAttr) -> AttrVal {
    // Fetch unmodified on-item attribute value, or use base attribute value if it is not available
    match item.get_a_attrs().unwrap().get(&a_attr.id) {
        Some(orig_val) => *orig_val as AttrVal,
        None => a_attr.def_val as AttrVal,
    }
}
