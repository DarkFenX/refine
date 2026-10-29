#[cfg_attr(feature = "dev", derive(PartialEq))]
#[derive(Copy, Clone)]
pub(in crate::svc::calc) enum ModifierKind {
    Local,
    Buff,
    FleetBuff,
    System,
    Targeted,
}
