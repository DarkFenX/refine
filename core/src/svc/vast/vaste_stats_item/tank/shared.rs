use crate::{
    def::{AttrVal, OF},
    misc::{DmgKinds, DpsProfile},
};

pub(super) fn get_tanking_efficiency(resists: &DmgKinds<AttrVal>, incoming_dps: DpsProfile) -> Option<AttrVal> {
    let dealt = incoming_dps.get_sum_regular();
    let absorbed = incoming_dps.get_em() * resists.em
        + incoming_dps.get_thermal() * resists.thermal
        + incoming_dps.get_kinetic() * resists.kinetic
        + incoming_dps.get_explosive() * resists.explosive;
    let received = dealt - absorbed;
    match received > OF(0.0) {
        true => Some(dealt / received),
        false => None,
    }
}
