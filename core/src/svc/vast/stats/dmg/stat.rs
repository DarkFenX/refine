use crate::{misc::DmgKinds, num::PValue, svc::output::OutputDmgBreacher};

pub struct StatDmg {
    pub em: PValue,
    pub thermal: PValue,
    pub kinetic: PValue,
    pub explosive: PValue,
    pub breacher: Option<StatDmgBreacher>,
}

pub struct StatDmgApplied {
    pub em: PValue,
    pub thermal: PValue,
    pub kinetic: PValue,
    pub explosive: PValue,
    pub breacher: Option<PValue>,
}

pub struct StatDmgBreacher {
    pub absolute_max: PValue,
    // This field is not unit interval since it is supposed to store breacher DPS as well, and DPS
    // can exceed value of 1 if server has more than 1 ticks per second
    pub relative_max: PValue,
}
impl StatDmgBreacher {
    pub(in crate::svc::vast) fn new() -> Self {
        Self {
            absolute_max: PValue::ZERO,
            relative_max: PValue::ZERO,
        }
    }
    pub(in crate::svc::vast) fn stack_instance_output(&mut self, other: OutputDmgBreacher) {
        self.absolute_max = self.absolute_max.max(other.absolute_max);
        self.relative_max = self.relative_max.max(other.relative_max.into_pvalue());
    }
    pub(in crate::svc::vast) fn nullified(self) -> Option<Self> {
        match self.absolute_max > PValue::FLOAT_TOLERANCE && self.relative_max > PValue::FLOAT_TOLERANCE {
            true => Some(self),
            false => None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatDmg {
    pub(in crate::svc::vast) fn from_dmgs(normal: DmgKinds<PValue>, breacher: Option<StatDmgBreacher>) -> Self {
        Self {
            em: normal.em,
            thermal: normal.thermal,
            kinetic: normal.kinetic,
            explosive: normal.explosive,
            breacher: match breacher {
                Some(breacher) => breacher.nullified(),
                _ => None,
            },
        }
    }
}

impl StatDmgApplied {
    pub(in crate::svc::vast) fn from_dmgs(normal: DmgKinds<PValue>, breacher: Option<PValue>) -> Self {
        Self {
            em: normal.em,
            thermal: normal.thermal,
            kinetic: normal.kinetic,
            explosive: normal.explosive,
            breacher,
        }
    }
}

impl StatDmgBreacher {
    pub(in crate::svc::vast) fn from_output(output: OutputDmgBreacher) -> Self {
        Self {
            absolute_max: output.absolute_max,
            relative_max: output.relative_max.into_pvalue(),
        }
    }
}
