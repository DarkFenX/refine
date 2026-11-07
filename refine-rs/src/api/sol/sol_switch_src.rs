use crate::{Sol, SolInfo, SolInfoCmd, SrcAlias, src::err::SrcGetError};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol<'_> {
    #[tracing::instrument(name = "sol-swt-src", level = "error", skip_all)]
    pub async fn switch_src(&mut self, src_alias: Option<SrcAlias>) -> Result<(), SolSwitchSrcError> {
        let inner_src = self.refine.internal_get_src(src_alias)?;
        // Variables for move
        let core_src = inner_src.get_core().clone();
        let src_alias = inner_src.get_alias();
        let guarded_src_alias = self.inner().get_guarded_src_alias();
        self.exec_standard_infallible(move |core_sol| {
            core_sol.set_src(&core_src);
            // Set alias in worker after the switch has been completed, to cover case when job is
            // skipped before worker picked it up
            *guarded_src_alias.lock() = src_alias;
        })
        .await;
        Ok(())
    }
    #[tracing::instrument(name = "sol-swt-src-inf", level = "error", skip_all)]
    pub async fn switch_src_and_get_info(
        &mut self,
        src_alias: Option<SrcAlias>,
        info_cmd: SolInfoCmd,
    ) -> Result<SolInfo, SolSwitchSrcError> {
        let inner_src = self.refine.internal_get_src(src_alias)?;
        // Variables for move
        let core_src = inner_src.get_core().clone();
        let src_alias = inner_src.get_alias();
        let shared_src_alias = self.inner().get_guarded_src_alias();
        let sol_id = self.get_id();
        let sol_info = self
            .exec_standard_infallible(move |core_sol| {
                core_sol.set_src(&core_src);
                *shared_src_alias.lock() = src_alias;
                info_cmd.execute(sol_id, src_alias, core_sol)
            })
            .await;
        Ok(sol_info)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SolSwitchSrcError {
    #[error("failed to get source")]
    SrcGet(#[from] SrcGetError),
}
