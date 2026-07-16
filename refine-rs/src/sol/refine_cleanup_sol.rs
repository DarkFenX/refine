use crate::refine::Refine;

impl Refine {
    #[tracing::instrument(name = "sol-cln", level = "trace", skip_all)]
    pub(crate) async fn cleanup_sols(&self, sol_inact_limit: chrono::TimeDelta) {
        tracing::debug!("starting cleanup");
        let now = chrono::Utc::now();
        let to_clean: Vec<_> = self
            .id_sol_map
            .read()
            .await
            .iter()
            .filter_map(|(sol_id, inner_sol)| match inner_sol.try_lock() {
                Ok(sol) if sol.get_last_accessed() + sol_inact_limit < now => Some(sol_id.clone()),
                // If it's locked - it means it's being worked on, we don't touch that
                _ => None,
            })
            .collect();
        if to_clean.is_empty() {
            tracing::debug!("nothing to clean");
            return;
        }
        self.id_sol_map.write().await.retain(|k, _| !to_clean.contains(k));
        tracing::info!("{} solar systems cleaned up", to_clean.len());
    }
}
