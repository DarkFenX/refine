#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::sol::svc::svce_calc) enum SolModifierKind {
    Local,
    Buff,
    FleetBuff,
    System,
    Targeted,
}
