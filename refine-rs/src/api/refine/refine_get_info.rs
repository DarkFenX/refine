use crate::{Refine, RefineInfo, RefineInfoCmd};

impl Refine {
    #[tracing::instrument(name = "rfn-inf", level = "error", skip_all)]
    pub async fn get_info(&self, info_cmd: RefineInfoCmd) -> RefineInfo {
        info_cmd.execute(self).await
    }
}
