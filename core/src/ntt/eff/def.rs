use crate::{
    ad, ed,
    sol::{
        AttrVal, ItemKey,
        svc::{
            calc::{Calc, RawModifier},
            eprojs::EProjs,
        },
        uad::Uad,
    },
};

struct NttEffect {
    // EVE data effect ID. Not all effects have it, since some are added via other means
    eid: Option<ed::EEffectId>,
    // Adapted data effect ID
    aid: ad::AEffectId,
    // Specifies if effect applies any buffs
    buff_info: Option<ad::AEffectBuffInfo>,
    // Specifies if effect uses charges
    charge_info: Option<ad::AEffectChargeInfo>,
    // Effect customization function ran during cache generation time
    custom_fn_adg: Option<fn(&mut ad::AEffect)>,
    // Effect modifier customization function ran during runtime in calculator service
    custom_fn_calc: Option<fn(&mut Vec<RawModifier>)>,
    // Specific effect getters
    get_local_armor_rep_amount: Option<fn(&Uad, &EProjs, &mut Calc, ItemKey) -> Option<AttrVal>>,
    get_local_shield_rep_amount: Option<fn(&Uad, &EProjs, &mut Calc, ItemKey) -> Option<AttrVal>>,
    get_local_structure_rep_amount: Option<fn(&Uad, &EProjs, &mut Calc, ItemKey) -> Option<AttrVal>>,
    get_remote_armor_rep_amount: Option<fn(&Uad, &EProjs, &mut Calc, ItemKey, ItemKey) -> Option<AttrVal>>,
    get_remote_shield_rep_amount: Option<fn(&Uad, &EProjs, &mut Calc, ItemKey, ItemKey) -> Option<AttrVal>>,
    get_remote_structure_rep_amount: Option<fn(&Uad, &EProjs, &mut Calc, ItemKey, ItemKey) -> Option<AttrVal>>,
}
