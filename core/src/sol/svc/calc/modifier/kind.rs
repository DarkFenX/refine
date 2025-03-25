#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::sol::svc::calc) enum ModifierKind {
    Local,
    Buff,
    FleetBuff,
    System,
    Targeted,
}
