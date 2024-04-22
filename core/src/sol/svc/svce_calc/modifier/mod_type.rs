#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::sol::svc::svce_calc) enum SolModType {
    Local,
    Targeted,
    FitWide,
    SystemWide,
    Projected,
    Fleet,
}
