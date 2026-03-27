#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc::calc) enum ModifierKind {
    Local,
    Buff,
    FleetBuff,
    System,
    Targeted,
}
