use crate::{
    misc::Ecm,
    num::{Count, PValue, UnitInterval},
    svc::vast::{
        StatSensors, StatSensorsKind,
        aggr::{SeqAccum, SeqInstanceAccum},
    },
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
    // This is total chance target won't be jammed a single time over sequence
    unjam_chance: PValue,
    // Jam time is jam chance multiplied by jam duration for a single instance. Different instances
    // within the same sequence stack additively, which is considered a good enough approximation.
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
    fn add_instance(&mut self, instance: Ecm, chance_mult: Option<PValue>, count: Count) {
        if count == Count::ZERO {
            return;
        }
        let jam_str = match self.sensors.kind {
            StatSensorsKind::Radar => instance.radar,
            StatSensorsKind::Magnetometric => instance.magnetometric,
            StatSensorsKind::Gravimetric => instance.gravimetric,
            StatSensorsKind::Ladar => instance.ladar,
        };
        // For case when jam strength is 0 and sensor strength is 0
        if jam_str < PValue::FLOAT_TOLERANCE {
            return;
        }
        // First, clamp jam strength to 100%, then apply chance-based multipliers. This is needed
        // for cases like: bomb has 10+ jam strength, target has 4 sensor strength, but bomb has 50%
        // chance to hit target (due to varying flight time), thus it can't jam target in more than
        // 50% cases in this case
        let mut jam_chance = UnitInterval::from_pvalue_clamped(jam_str / self.sensors.strength);
        if let Some(chance_mult) = chance_mult {
            jam_chance = UnitInterval::from_pvalue_clamped(jam_chance.into_pvalue() * chance_mult);
        }
        // Record changes based on calculated jam chance
        self.jam_time += instance.duration * jam_chance.into_pvalue() * count.into_pvalue();
        self.unjam_chance *= PValue::from_value_unchecked(PValue::ONE - jam_chance.into_pvalue()).pow_count(count)
    }
    fn copy_blank(&self) -> Self {
        Self::new(self.sensors)
    }
    fn merge(&mut self, other: &Self, count: Count) {
        // Consider accum being merged as part of the same sequence for stacking considerations
        self.unjam_chance *= other.unjam_chance.pow_count(count);
        self.jam_time += other.jam_time * count.into_pvalue();
    }
}
