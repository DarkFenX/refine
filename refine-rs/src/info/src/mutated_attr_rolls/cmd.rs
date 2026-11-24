use crate::{
    AttrId, Value,
    src::{SrcMutatedAttrRolls, SrcMutation},
};

/// Converts absolute attribute values of a mutated item into rolls.
#[cfg_attr(feature = "serde", cfg_eval, serde_with::serde_as, derive(serde::Deserialize))]
#[derive(Clone)]
pub struct SrcMutatedAttrRollsCmd {
    mutation: SrcMutation,
    #[cfg_attr(feature = "serde", serde_as(as = "serde_with::Map<_, _>"))]
    values: Vec<(AttrId, Value)> = Vec::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SrcMutatedAttrRollsCmd {
    pub const fn new(mutation: SrcMutation) -> Self {
        Self { mutation, .. }
    }
    pub fn with_values(mut self, values: impl IntoIterator<Item = (AttrId, Value)>) -> Self {
        self.values.extend(values);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SrcMutatedAttrRollsCmd {
    pub(crate) fn execute(self, core_src: &rc::Src) -> SrcMutatedAttrRolls {
        SrcMutatedAttrRolls::from_core(core_src, self.mutation, &self.values)
    }
}
