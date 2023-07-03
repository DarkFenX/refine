use crate::ad;

mod drone_dmg_self_srq;
mod missile_dmg_self_srq;
mod missile_rof_self_srq;
mod online_eff_cat;

pub(in crate::adg) fn customize(a_data: &mut ad::AData) {
    online_eff_cat::fix_online_effect_cat(a_data);
    missile_rof_self_srq::mk_self_skillreq_modifiers_launcher_rof(a_data);
    missile_dmg_self_srq::mk_self_skillreq_modifier_missile_dmg(a_data);
    drone_dmg_self_srq::mk_self_skillreq_drone_dmg(a_data);
}
