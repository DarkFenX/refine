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

pub(crate) struct NttEffect {
    // EVE data effect ID. Not all effects have it, since some are added via other means
    pub(crate) eid: Option<ed::EEffectId>,
    // Adapted data effect ID
    pub(crate) aid: ad::AEffectId,
    // Specifies if effect applies any buffs
    pub(crate) buff_info: Option<ad::AEffectBuffInfo> = None,
    // Specifies if effect uses charges
    pub(crate) charge_info: Option<ad::AEffectChargeInfo> = None,
    // Effect customization function ran during cache generation time
    pub(crate) custom_fn_adg: Option<fn(&mut ad::AEffect)> = None,
    // Effect modifier customization function ran during runtime in calculator service
    pub(crate) custom_fn_calc: Option<fn(&mut Vec<RawModifier>)> = None,
    // Specific effect getters
    pub(crate) get_local_armor_rep_amount: Option<fn(&Uad, &EProjs, &mut Calc, ItemKey) -> Option<AttrVal>> = None,
    pub(crate) get_local_shield_rep_amount: Option<fn(&Uad, &EProjs, &mut Calc, ItemKey) -> Option<AttrVal>> = None,
    pub(crate) get_local_structure_rep_amount: Option<fn(&Uad, &EProjs, &mut Calc, ItemKey) -> Option<AttrVal>> = None,
    pub(crate) get_remote_armor_rep_amount:
        Option<fn(&Uad, &EProjs, &mut Calc, ItemKey, ItemKey) -> Option<AttrVal>> = None,
    pub(crate) get_remote_shield_rep_amount:
        Option<fn(&Uad, &EProjs, &mut Calc, ItemKey, ItemKey) -> Option<AttrVal>> = None,
    pub(crate) get_remote_structure_rep_amount:
        Option<fn(&Uad, &EProjs, &mut Calc, ItemKey, ItemKey) -> Option<AttrVal>> = None,
}
