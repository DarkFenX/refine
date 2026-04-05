#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AEffectLocation {
    Ship,
    Structure,
    Char,
    Item,
    Other,
    Target,
}
