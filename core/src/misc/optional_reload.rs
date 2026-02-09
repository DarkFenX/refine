/// Controls what happens when modules with optional reloads (AAR, ASB) do when they run out of
/// charges.
#[derive(Copy, Clone)]
pub enum OptionalReload {
    /// No reload - keep running after running out of charges.
    Disabled,
    /// Go into reload when charges completely run out.
    OnEmpty,
}
