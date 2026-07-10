/// Controls what happens when modules with optional reloads (AAR, ASB) do when they run out of
/// charges.
#[derive(Copy, Clone)]
pub enum OptionalReload {
    /// No reload - keep running after running out of charges.
    Disabled,
    /// Go into reload when charges completely run out.
    OnEmpty,
}

#[derive(Copy, Clone)]
pub struct ItemOptionalReloadInfo {
    /// Effective value of item's "optional reload" setting.
    pub value: OptionalReload,
    /// True if setting is defined directly on item, false if inherited from sol.
    pub overridden: bool,
}
