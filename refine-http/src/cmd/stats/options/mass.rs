use serde::Deserialize;

use super::shared_affectors::HStatAffectors;

#[derive(Copy, Clone, Default, Deserialize)]
pub(in crate::cmd) struct HStatOptionMass {
    #[serde(default)]
    pub(in crate::cmd) affectors: HStatAffectors,
}
