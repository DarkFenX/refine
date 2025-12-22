use crate::{
    def::OF,
    nd::NEffectChargeDeplChargeRate,
    svc::{SvcCtx, cycle::effect_charge_info::EffectChargeInfo},
    ud::UModule,
    util::InfCount,
};

pub(in crate::svc::cycle) fn get_eci_charge_rate(
    ctx: SvcCtx,
    module: &UModule,
    n_charge_rate: NEffectChargeDeplChargeRate,
) -> EffectChargeInfo {
    let charge_count = match module.get_charge_count(ctx.u_data) {
        Some(charge_count) => charge_count,
        None => {
            return EffectChargeInfo {
                fully_charged: InfCount::Count(0),
                part_charged: None,
                can_run_uncharged: n_charge_rate.can_run_uncharged,
            };
        }
    };
    if charge_count == 0 {
        return EffectChargeInfo {
            fully_charged: InfCount::Count(0),
            part_charged: None,
            can_run_uncharged: n_charge_rate.can_run_uncharged,
        };
    }
    let charges_per_cycle = module.get_axt().unwrap().charge_rate;
    if charges_per_cycle == 0 {
        return EffectChargeInfo {
            fully_charged: InfCount::Infinite,
            part_charged: None,
            can_run_uncharged: n_charge_rate.can_run_uncharged,
        };
    }
    let fully_charged = charge_count / charges_per_cycle;
    // Modules which can run uncharged are assumed to be able to run off partial charges
    let part_charged = match n_charge_rate.can_run_uncharged {
        true => {
            let remaining = charge_count % charges_per_cycle;
            match remaining > 0 {
                true => Some(OF(remaining as f64 / charges_per_cycle as f64)),
                false => None,
            }
        }
        false => None,
    };
    EffectChargeInfo {
        fully_charged: InfCount::Count(fully_charged),
        part_charged,
        can_run_uncharged: n_charge_rate.can_run_uncharged,
    }
}
