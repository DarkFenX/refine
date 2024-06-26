#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::sol::svc::svce_calc) enum SolModifierKind {
    Local,
    Buff,
    FleetBuff,
    System,
    Targeted,
}
impl SolModifierKind {
    pub(in crate::sol::svc::svce_calc) fn is_projectable(&self) -> bool {
        match self {
            Self::Local => false,
            Self::Buff => true,
            Self::FleetBuff => false,
            Self::System => false,
            Self::Targeted => true,
        }
    }
}
