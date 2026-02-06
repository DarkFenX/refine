use super::shared::{SeqAccum, SeqInstanceAccum};
use crate::{
    misc::Ecm,
    num::{Count, PValue, UnitInterval},
    svc::vast::{StatSensors, StatSensorsKind},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Top-level accumulator interface
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SeqAccum<SeqInstanceAccumEcm> {
    pub(in crate::svc::vast) fn new_jam_chance(sensors: StatSensors) -> Self {
        SeqAccum {
            instances: SeqInstanceAccumEcm::new(sensors),
            time: PValue::ZERO,
        }
    }
    pub(in crate::svc::vast) fn get_unjam_chance(&self) -> UnitInterval {
        UnitInterval::from_pvalue_clamped(self.instances.unjam_chance)
    }
    pub(in crate::svc::vast) fn get_unjam_uptime(&self) -> UnitInterval {
        UnitInterval::from_value_clamped(PValue::ONE - self.instances.jam_time / self.time)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sequence accumulator implementation
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc::vast) struct SeqInstanceAccumEcm {
    sensors: StatSensors,
    unjam_chance: PValue,
    jam_time: PValue,
}
impl SeqInstanceAccumEcm {
    pub(in crate::svc::vast) fn new(sensors: StatSensors) -> Self {
        Self {
            sensors,
            unjam_chance: PValue::ONE,
            jam_time: PValue::ZERO,
        }
    }
}
impl SeqInstanceAccum<Ecm> for SeqInstanceAccumEcm {
    fn add_instance(&mut self, mut instance: Ecm, chance_mult: Option<PValue>, count: Count) {
        if count == Count::ZERO {
            return;
        }
        let jam_str = match self.sensors.kind {
            StatSensorsKind::Radar => instance.radar,
            StatSensorsKind::Magnetometric => instance.magnetometric,
            StatSensorsKind::Gravimetric => instance.gravimetric,
            StatSensorsKind::Ladar => instance.ladar,
        };
        if jam_str <= PValue::FLOAT_TOLERANCE {
            return;
        }
        let mut jam_chance = UnitInterval::from_pvalue_clamped(jam_str / self.sensors.strength);
        if let Some(chance_mult) = chance_mult {
            jam_chance = UnitInterval::from_pvalue_clamped(jam_chance.into_pvalue() * chance_mult);
        }
        self.jam_time += instance.duration * jam_chance.into_pvalue() * count.into_pvalue();
        self.unjam_chance *= PValue::from_value_unchecked(PValue::ONE - jam_chance.into_pvalue()).pow_count(count)
    }
    fn copy_blank(&self) -> Self {
        Self::new(self.sensors)
    }
    fn merge(&mut self, other: &Self, count: Count) {
        self.unjam_chance *= other.unjam_chance.pow_count(count);
        self.jam_time += other.jam_time * count.into_pvalue();
    }
}
