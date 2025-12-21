use crate::{
    def::OF,
    nd::NEffectChargeDeplChargeRate,
    svc::{SvcCtx, cycle::charged_info::EffectChargedInfo},
    ud::UModule,
    util::InfCount,
};

pub(in crate::svc::cycle) fn get_charge_rate_charged_info(
    ctx: SvcCtx,
    module: &UModule,
    n_charge_rate: NEffectChargeDeplChargeRate,
) -> EffectChargedInfo {
    let charge_count = match module.get_charge_count(ctx.u_data) {
        Some(charge_count) => charge_count,
        None => {
            return EffectChargedInfo {
                fully_charged: InfCount::Count(0),
                part_charged: None,
                can_run_uncharged: n_charge_rate.can_run_uncharged,
            };
        }
    };
    if charge_count == 0 {
        return EffectChargedInfo {
            fully_charged: InfCount::Count(0),
            part_charged: None,
            can_run_uncharged: n_charge_rate.can_run_uncharged,
        };
    }
    let charges_per_cycle = module.get_axt().unwrap().charge_rate;
    if charges_per_cycle == 0 {
        return EffectChargedInfo {
            fully_charged: InfCount::Infinite,
            part_charged: None,
            can_run_uncharged: n_charge_rate.can_run_uncharged,
        };
    }
    let fully_charged = charge_count / charges_per_cycle;
    let part_charged = match n_charge_rate.can_run_part_charged {
        true => {
            let remaining = charge_count % charges_per_cycle;
            match remaining > 0 {
                true => Some(OF(remaining as f64 / charges_per_cycle as f64)),
                false => None,
            }
        }
        false => None,
    };
    EffectChargedInfo {
        fully_charged: InfCount::Count(fully_charged),
        part_charged,
        can_run_uncharged: n_charge_rate.can_run_uncharged,
    }
}
