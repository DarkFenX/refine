use crate::{
    Sol,
    val::{SolValCmd, SolValResult},
};

impl Sol<'_> {
    #[tracing::instrument(name = "sol-val", level = "error", skip_all)]
    pub async fn validate(&mut self, val_cmd: SolValCmd) -> SolValResult {
        self.exec_standard_infallible(|core| val_cmd.execute_owned(core)).await
    }
}
