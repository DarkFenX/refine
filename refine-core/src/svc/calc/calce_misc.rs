use crate::{
    PValue, Value,
    rd::REffect,
    svc::{
        SvcCtx,
        calc::{Calc, CalcOp},
    },
    ud::UItemId,
};

impl Calc {
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
