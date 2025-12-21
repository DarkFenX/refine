use crate::{
    def::{AttrVal, OF},
    misc::DmgKinds,
    svc::output::OutputDmgBreacher,
    util::FLOAT_TOLERANCE,
};

pub struct StatDmg {
    pub em: AttrVal,
    pub thermal: AttrVal,
    pub kinetic: AttrVal,
    pub explosive: AttrVal,
    pub breacher: Option<StatDmgBreacher>,
}
impl From<(DmgKinds<AttrVal>, Option<StatDmgBreacher>)> for StatDmg {
    fn from((dmg_kinds, breacher): (DmgKinds<AttrVal>, Option<StatDmgBreacher>)) -> Self {
        Self {
            em: dmg_kinds.em,
            thermal: dmg_kinds.thermal,
            kinetic: dmg_kinds.kinetic,
            explosive: dmg_kinds.explosive,
            breacher: match breacher {
                Some(breacher) => breacher.nullified(),
                _ => None,
            },
        }
    }
}

pub struct StatDmgApplied {
    pub em: AttrVal,
    pub thermal: AttrVal,
    pub kinetic: AttrVal,
    pub explosive: AttrVal,
    pub breacher: Option<AttrVal>,
}
impl From<(DmgKinds<AttrVal>, Option<AttrVal>)> for StatDmgApplied {
    fn from((dmg_kinds, breacher): (DmgKinds<AttrVal>, Option<AttrVal>)) -> Self {
        Self {
            em: dmg_kinds.em,
            thermal: dmg_kinds.thermal,
            kinetic: dmg_kinds.kinetic,
            explosive: dmg_kinds.explosive,
            breacher,
        }
    }
}

pub struct StatDmgBreacher {
    pub absolute_max: AttrVal,
    pub relative_max: AttrVal,
}
impl StatDmgBreacher {
    pub(in crate::svc::vast) fn new() -> Self {
        Self {
            absolute_max: OF(0.0),
            relative_max: OF(0.0),
        }
    }
    pub(in crate::svc::vast) fn stack_instance_output(&mut self, other: OutputDmgBreacher) {
        self.absolute_max = self.absolute_max.max(other.absolute_max);
        self.relative_max = self.relative_max.max(other.relative_max);
    }
    pub(in crate::svc::vast) fn nullified(self) -> Option<Self> {
        match self.absolute_max > FLOAT_TOLERANCE && self.relative_max > FLOAT_TOLERANCE {
            true => Some(self),
            false => None,
        }
    }
}
impl From<OutputDmgBreacher> for StatDmgBreacher {
    fn from(output: OutputDmgBreacher) -> Self {
        Self {
            absolute_max: output.absolute_max,
            relative_max: output.relative_max,
        }
    }
}
