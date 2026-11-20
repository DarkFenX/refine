use crate::{
    PValue, Value,
    misc::EffectSpec,
    rd::{RAttrId, REffect},
    svc::{
        SvcCtx,
        calc::{Calc, CalcOp},
        err::UItemLoadedError,
    },
    ud::UItemId,
};

impl Calc {
    /// Get affector effect, operator and strength of every modifier which affects an item's
    /// attribute.
    ///
    /// Modifiers are returned as they are registered, without evaluating how they combine.
    pub(in crate::svc) fn iter_item_attr_mods_op_str(
        &mut self,
        ctx: &SvcCtx,
        item_uid: UItemId,
        attr_rid: RAttrId,
    ) -> Result<impl ExactSizeIterator<Item = (EffectSpec, CalcOp, Value)> + use<>, UItemLoadedError> {
        let item = ctx.u_data.items.get(item_uid);
        if !item.is_loaded() {
            return Err(UItemLoadedError { item_uid });
        }
        let mut reuse_ckeys = self.reuse.ckeys.take();
        self.std
            .get_mods_for_affectee(&mut reuse_ckeys, item_uid, item, attr_rid, &ctx.u_data.fits);
        let mut mods = Vec::with_capacity(reuse_ckeys.len());
        for ckey in reuse_ckeys.iter() {
            let rmod = self.rmods.get_cmod(ctx.rmt(), ckey).rmod;
            if let Some(strength) = rmod.get_mod_val(self, ctx) {
                mods.push((rmod.affector_espec, rmod.op, strength));
            }
        }
        self.reuse.ckeys.put(reuse_ckeys);
        Ok(mods.into_iter())
    }
    /// Get operator and strength shared by all modifiers of an item's effect.
    ///
    /// The effect does not have to be running. Strengths have to be almost equal, since they can
    /// reach the same value in different ways; the one closest to 0 is returned.
    pub(in crate::svc) fn get_effect_mods_op_str(
        &mut self,
        ctx: &SvcCtx,
        item_uid: UItemId,
        effect: &REffect,
    ) -> Option<(CalcOp, Value)> {
        let mut rmods = Vec::new();
        if effect.needs_rmod_gen {
            let item = ctx.u_data.items.get(item_uid);
            self.generate_mods_for_effect(&mut rmods, ctx.u_data, item_uid, item, effect);
        }
        rmods.extend(ctx.rmt().iter_effect(effect, item_uid).map(|(_, rmod)| rmod));
        let op = rmods.first()?.op;
        let mut strengths = Vec::with_capacity(rmods.len());
        for rmod in rmods.iter() {
            if rmod.op != op {
                return None;
            }
            strengths.push(rmod.get_mod_val(self, ctx)?);
        }
        // Strength closest to 0 has the smallest tolerance, so comparing against it makes the
        // result independent of modifier order
        let base_strength = strengths.iter().copied().min_by_key(|strength| strength.abs())?;
        if !strengths.iter().all(|strength| {
            strength.approx_eq(base_strength, PValue::from_f64_unchecked(1e-5), PValue::FLOAT_TOLERANCE)
        }) {
            return None;
        }
        Some((op, base_strength))
    }
}
