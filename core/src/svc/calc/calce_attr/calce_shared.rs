use crate::{
    def::AttrVal,
    misc::AttrSpec,
    rd::RAttr,
    svc::{
        SvcCtx,
        calc::{Calc, CtxModifier},
        eff_funcs,
    },
    ud::UItem,
};

impl Calc {
    pub(super) fn calc_resist_mult(&mut self, ctx: SvcCtx, cmod: &CtxModifier) -> Option<AttrVal> {
        let resist_attr_key = cmod.raw.resist_attr_key?;
        let item_key = cmod.ctx.get_item_key()?;
        let resist =
            eff_funcs::get_resist_mult_by_projectee_aspec(ctx, self, &AttrSpec::new(item_key, resist_attr_key))?;
        Some(resist)
    }
    pub(super) fn calc_proj_mult(&mut self, ctx: SvcCtx, cmod: &CtxModifier) -> Option<AttrVal> {
        let item_key = cmod.ctx.get_item_key()?;
        let proj_mult_getter = cmod.raw.proj_mult_getter?;
        let effect = ctx.u_data.src.get_effect(cmod.raw.affector_espec.effect_key);
        let proj_data = ctx.eff_projs.get_proj_data(cmod.raw.affector_espec, item_key)?;
        Some(proj_mult_getter(
            ctx,
            self,
            cmod.raw.affector_espec.item_key,
            effect,
            item_key,
            proj_data,
        ))
    }
}

pub(super) fn get_base_attr_value(item: &UItem, attr: &RAttr) -> AttrVal {
    // Fetch unmodified on-item attribute value, or use base attribute value if it is not available
    match item.get_attrs().unwrap().get(&attr.key) {
        Some(orig_val) => *orig_val,
        None => attr.def_val,
    }
}
