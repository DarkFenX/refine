use crate::{
    cmd::{RemoveFitCmd, RemoveFitError},
    fit::Fit,
};

impl Fit<'_, '_> {
    #[tracing::instrument(name = "fit-rmv", level = "trace", skip_all)]
    pub async fn remove(self, cmd: RemoveFitCmd) {
        match self
            .sol
            .exec_standard_safe(move |core_sol| cmd.execute(core_sol, &self.id))
            .await
        {
            Ok(_) => (),
            // Holding mutex on sol - nothing can remove the fit before we do
            Err(RemoveFitError::FitGetFailed(_)) => unreachable!(),
        }
    }
}
