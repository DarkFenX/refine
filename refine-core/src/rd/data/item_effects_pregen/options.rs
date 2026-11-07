/// Effect resolution options, which depend on item kind.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct RItemEffectOptions(u8);
impl RItemEffectOptions {
    pub(crate) fn contains(self, options: Self) -> bool {
        self.0 & options.0 != 0
    }
    /// Position in arrays with an entry per used options value.
    pub(crate) fn index(self) -> usize {
        // Used options values are 0, 1 and 2, the order of ALL
        self.0 as usize
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Constants
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RItemEffectOptions {
    pub(crate) const NONE: Self = Self(0);
    /// Effects with disabled state run only if they are default effect of the item.
    pub(crate) const REQUIRE_DISABLED_DEFEFF: Self = Self(1);
    /// All effects with active state run, not just default effect of the item.
    pub(crate) const FORCE_ACTIVE_NONDEFEFF: Self = Self(2);
    /// Options items of some kind use.
    pub(crate) const ALL: [Self; 3] = [Self::NONE, Self::REQUIRE_DISABLED_DEFEFF, Self::FORCE_ACTIVE_NONDEFEFF];
}
