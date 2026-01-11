use crate::{misc::DpsProfile, num::PValue, svc::vast::StatResistsLayer};

pub(super) fn get_tanking_efficiency(resists: StatResistsLayer, incoming_dps: DpsProfile) -> Option<PValue> {
    let dealt = incoming_dps.get_sum_regular();
    let absorbed = incoming_dps.get_em() * resists.em.into_pvalue()
        + incoming_dps.get_thermal() * resists.thermal.into_pvalue()
        + incoming_dps.get_kinetic() * resists.kinetic.into_pvalue()
        + incoming_dps.get_explosive() * resists.explosive.into_pvalue();
    let received = PValue::from_value_unchecked(dealt - absorbed);
    match received > PValue::ZERO {
        true => Some(dealt / received),
        false => None,
    }
}
