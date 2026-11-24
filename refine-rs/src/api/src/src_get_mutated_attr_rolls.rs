use crate::{
    Src,
    src::{SrcMutatedAttrRolls, SrcMutatedAttrRollsCmd},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Src<'_> {
    #[tracing::instrument(name = "src-mut-atr-rol", level = "error", skip_all)]
    pub async fn get_mutated_attr_rolls(&self, info_cmd: SrcMutatedAttrRollsCmd) -> SrcMutatedAttrRolls {
        info_cmd.execute(self.inner.get_core())
    }
}
