// In EVE, cap sticks reduce cap use of ASBs via regular dogma effect. In the lib, it's handled
// separately (to support varying ASB cap use depending on presence of charges)

use crate::ad::{AData, AEffectId, AItemGrpId};

const CHARGE_GROUP: AItemGrpId = AItemGrpId::CAPACITOR_BOOSTER_CHARGE;
const CAP_EFFECT: AEffectId = AEffectId::AMMO_INFLUENCE_CAP_NEED;

pub(in crate::ad::generator::flow::s7_custom) fn remove_asb_cap_stick_effect(a_data: &mut AData) {
    let mut applied = false;
    for item in a_data.items.data.values_mut() {
        if item.grp_id != CHARGE_GROUP {
            continue;
        }
        if item.effects.remove(&CAP_EFFECT).is_some() {
            applied = true;
        }
    }
    if !applied {
        tracing::info!("effect {CAP_EFFECT}: was not removed from cap charges");
    }
}
