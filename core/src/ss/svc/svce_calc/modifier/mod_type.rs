#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::ss::svc::svce_calc) enum SsModType {
    Local,
    Targeted,
    FitWide,
    SystemWide,
    Projected,
    Fleet,
}
