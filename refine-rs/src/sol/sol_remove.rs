use crate::{
    refine::Refine,
    sol::{SolarSystem, SolarSystemId},
};

impl Refine {
    #[tracing::instrument(name = "sol-rm", level = "trace", skip_all)]
    async fn remove_sol(&mut self, id: SolarSystemId) {
        self.id_sol_map.write().await.remove(&id);
    }
}

impl SolarSystem<'_> {
    pub async fn remove(self) {
        self.refine.remove_sol(self.id).await;
    }
}
