use crate::{
    ac, ad,
    def::OF,
    ec,
    ntt::{
        NttEffect, NttEffectRt,
        eff::shared::proj_mult::{get_proj_attrs_simple, get_proj_mult_simple_s2s},
    },
};

pub(super) fn mk_ntt_effect() -> NttEffect {
    // Dreadnought lance
    NttEffect {
        eid: Some(ec::effects::DEBUFF_LANCE),
        aid: ac::effects::DEBUFF_LANCE,
        adg_buff_info: Some(ad::AEffectBuffInfo {
            source: ad::AEffectBuffSrc::Customized(vec![
                ad::AEffectBuffSrcCustom::HardcodedVal(ac::buffs::REMOTE_REPAIR_IMPEDANCE, OF(-50.0)),
                ad::AEffectBuffSrcCustom::HardcodedVal(ac::buffs::WARP_PENALTY, OF(100.0)),
                ad::AEffectBuffSrcCustom::HardcodedVal(ac::buffs::DISALLOW_DOCK_JUMP, OF(1.0)),
                ad::AEffectBuffSrcCustom::HardcodedVal(ac::buffs::DISALLOW_TETHER, OF(1.0)),
            ]),
            scope: ad::AEffectBuffScope::Everything,
        }),
        // TODO: test if it uses surface-to-surface range (might use center-to-surface), and check
        // TODO: if damage radius is needed to be added to range or not
        xt_get_proj_attrs: Some(get_proj_attrs_simple),
        rt: NttEffectRt {
            get_proj_mult: Some(get_proj_mult_simple_s2s),
            ..
        },
        ..
    }
}
