use crate::{
    Value,
    rd::REffect,
    svc::{
        SvcCtx,
        calc::{Calc, CalcOp},
    },
    ud::UItemId,
    util::sig_eq,
};

impl Calc {
    /// Get operator and strength shared by all modifiers of an item's effect.
    ///
    /// The effect does not have to be running. Strengths have to be almost equal, since they can
    /// reach the same value in different ways.
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
        let (first, others) = rmods.split_first()?;
        let first_strength = first.get_mod_val(self, ctx)?;
        for other in others {
            if first.op != other.op {
                return None;
            }
            let other_strength = other.get_mod_val(self, ctx)?;
            if !sig_eq(other_strength.into_f64(), first_strength.into_f64(), 5) {
                return None;
            }
        }
        Some((first.op, first_strength))
    }
}
