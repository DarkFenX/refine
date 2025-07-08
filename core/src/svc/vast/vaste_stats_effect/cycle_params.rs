use crate::{
    ac,
    def::{Count, ItemKey},
    misc::EffectSpec,
    nd::NEffectCharge,
    svc::SvcCtx,
    uad::UadItem,
};

pub(in crate::svc::vast) enum EffectCharge {
    NoCharge,
    Charge(EffectChargeInfo),
}
impl EffectCharge {
    pub(in crate::svc::vast) fn get_cycle_count(&self) -> Option<Count> {
        if let Self::Charge(charge_info) = self
            && let EffectChargeCountKind::Count(count_info) = charge_info.count_info
        {
            return Some(count_info.cycle_count);
        }
        None
    }
}

pub(in crate::svc::vast) struct EffectChargeInfo {
    pub(in crate::svc::vast) item_key: ItemKey,
    pub(in crate::svc::vast) count_info: EffectChargeCountKind,
}

pub(in crate::svc::vast) enum EffectChargeCountKind {
    Count(EffectChargeCountInfo),
    Infinite,
}

#[derive(Copy, Clone)]
pub(in crate::svc::vast) struct EffectChargeCountInfo {
    pub(in crate::svc::vast) charge_count: Count,
    pub(in crate::svc::vast) cycle_count: Count,
}

pub(in crate::svc::vast) fn get_effect_charge(ctx: SvcCtx, espec: &EffectSpec) -> EffectCharge {
    match ctx.uad.src.get_a_effect(&espec.a_effect_id).unwrap().hc.charge {
        Some(NEffectCharge::Loaded(_)) => {
            let parent_item = ctx.uad.items.get(espec.item_key);
            let charge_key = match parent_item.get_charge_key() {
                Some(charge_key) => charge_key,
                // No charge - return, well, no charge
                None => return EffectCharge::NoCharge,
            };
            let charge_count = match parent_item.get_charge_count(ctx.uad) {
                Some(charge_count) => charge_count,
                // No info about charge count is handled as if there was no charge. It can happen
                // when either module or charge is not loaded, or when some attributes are missing,
                // or in other fringe cases
                None => return EffectCharge::NoCharge,
            };
            EffectCharge::Charge(EffectChargeInfo {
                item_key: charge_key,
                count_info: get_count_info_for_loaded_charge(parent_item, charge_count),
            })
        }
        Some(NEffectCharge::Attr(_)) => {
            let parent_item = ctx.uad.items.get(espec.item_key);
            let charge_key = match parent_item.get_autocharges() {
                Some(autocharges) => match autocharges.get(&espec.a_effect_id) {
                    Some(charge_key) => *charge_key,
                    None => return EffectCharge::NoCharge,
                },
                None => return EffectCharge::NoCharge,
            };
            // For now, assume all items are loaded
            let count_info = match parent_item.get_a_effect_datas().unwrap().get(&espec.a_effect_id) {
                Some(a_item_effect_data) => match a_item_effect_data.charge_count {
                    Some(charge_count) => EffectChargeCountKind::Count(EffectChargeCountInfo {
                        charge_count,
                        cycle_count: charge_count,
                    }),
                    None => EffectChargeCountKind::Infinite,
                },
                None => EffectChargeCountKind::Infinite,
            };
            EffectCharge::Charge(EffectChargeInfo {
                item_key: charge_key,
                count_info,
            })
        }
        None => EffectCharge::NoCharge,
    }
}

fn get_count_info_for_loaded_charge(parent_item: &UadItem, charge_count: Count) -> EffectChargeCountKind {
    let charges_per_cycle = match parent_item.get_a_attr(&ac::attrs::CHARGE_RATE) {
        Some(charge_rate) => charge_rate.round() as Count,
        None => 1,
    };
    // Here it's assumed that an effect can cycle only when it has enough charges into it. This is
    // not true for items like AAR, which can cycle for partial rep efficiency, but since w/o manual
    // adjustments all AARs have enough paste to run w/o partial efficiency cycles, we ignore this
    // for simplicity's & performance's sake
    let cycle_count = charge_count / charges_per_cycle;
    EffectChargeCountKind::Count(EffectChargeCountInfo {
        charge_count,
        cycle_count,
    })
}
