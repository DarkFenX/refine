use crate::{
    def::{AttrVal, OF},
    misc::DmgKinds,
    svc::output::OutputDmgBreacher,
};

pub struct StatDmg {
    pub em: AttrVal,
    pub thermal: AttrVal,
    pub kinetic: AttrVal,
    pub explosive: AttrVal,
    pub breacher: Option<StatDmgBreacher>,
}
impl StatDmg {
    pub(in crate::svc::vast) fn new() -> Self {
        Self {
            em: OF(0.0),
            thermal: OF(0.0),
            kinetic: OF(0.0),
            explosive: OF(0.0),
            breacher: None,
        }
    }
    pub(in crate::svc::vast) fn stack_instance_self(&mut self, other: Self) {
        self.em += other.em;
        self.thermal += other.thermal;
        self.kinetic += other.kinetic;
        self.explosive += other.explosive;
        if let Some(other_breacher) = other.breacher {
            self.stack_instance_breacher_stat(other_breacher);
        }
    }
    pub(in crate::svc::vast) fn stack_instance_normal(&mut self, other: DmgKinds<AttrVal>) {
        self.em += other.em;
        self.thermal += other.thermal;
        self.kinetic += other.kinetic;
        self.explosive += other.explosive;
    }
    pub(in crate::svc::vast) fn stack_instance_breacher_stat(&mut self, other: StatDmgBreacher) {
        match &mut self.breacher {
            Some(breacher) => breacher.stack_instance_self(other),
            None => self.breacher = Some(other),
        }
    }
    pub(in crate::svc::vast) fn stack_instance_breacher_output(&mut self, other: OutputDmgBreacher) {
        match &mut self.breacher {
            Some(breacher) => breacher.stack_instance_output(other),
            None => self.breacher = Some(other.into()),
        }
    }
}
impl From<DmgKinds<AttrVal>> for StatDmg {
    fn from(dmg_kinds: DmgKinds<AttrVal>) -> Self {
        Self {
            em: dmg_kinds.em,
            thermal: dmg_kinds.thermal,
            kinetic: dmg_kinds.kinetic,
            explosive: dmg_kinds.explosive,
            breacher: None,
        }
    }
}

pub struct StatDmgBreacher {
    pub absolute_max: AttrVal,
    pub relative_max: AttrVal,
}
impl StatDmgBreacher {
    pub(in crate::svc::vast) fn stack_instance_self(&mut self, other: Self) {
        self.absolute_max = self.absolute_max.max(other.absolute_max);
        self.relative_max = self.relative_max.max(other.relative_max);
    }
    pub(in crate::svc::vast) fn stack_instance_output(&mut self, other: OutputDmgBreacher) {
        self.absolute_max = self.absolute_max.max(other.absolute_max);
        self.relative_max = self.relative_max.max(other.relative_max);
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
