#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum ModifierKind {
    Local,
    Buff,
    FleetBuff,
    System,
    Targeted,
}
