use crate::{
    SolarSystem,
    val::{SolValInfo, ValSolInfoArgs, ValidateSolCmd},
};

impl SolarSystem<'_> {
    #[tracing::instrument(name = "sol-val", level = "trace", skip_all)]
    pub async fn validate(&mut self, cmd: ValidateSolCmd, info_args: ValSolInfoArgs) -> SolValInfo {
        self.exec_standard_safe(move |core_sol| cmd.execute(core_sol, info_args.validation))
            .await
    }
}
