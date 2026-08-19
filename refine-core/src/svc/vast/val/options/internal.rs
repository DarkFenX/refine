use itertools::Itertools;

use crate::{
    OptionExt, SolarSystem,
    svc::vast::val::ValKind,
    ud::{UFitId, UItemId},
    util::RSet,
    val::{ValEnabled, ValOptions, ValOptionsSol},
};

// Internal variant, with fit/item UIDs instead of external IDs
pub(crate) struct ValOptionsSolInt {
    pub(crate) options: ValOptionsInt,
    pub(crate) fit_uids: Vec<UFitId>,
}
impl ValOptionsSolInt {
    pub(crate) fn from_pub(pub_sol_opts: &ValOptionsSol, sol: &SolarSystem) -> Self {
        Self {
            options: ValOptionsInt::from_pub(&pub_sol_opts.options, sol),
            fit_uids: pub_sol_opts
                .fit_ids
                .iter()
                .filter_map(|fit_id| sol.u_data.fits.int_id_by_ext_id(fit_id))
                .unique()
                .collect(),
        }
    }
}

pub(crate) struct ValOptionsInt([ValOptionInt; std::mem::variant_count::<ValKind>()]);
impl ValOptionsInt {
    pub(crate) fn from_pub(pub_opts: &ValOptions, sol: &SolarSystem) -> Self {
        let default = match pub_opts.default {
            true => ValOptionInt::Enabled(ValOptionEnabledInt { kfs: RSet::new() }),
            false => ValOptionInt::Disabled,
        };
        let mut int_opts = Self(std::array::from_fn(|_| default.clone()));
        for (opt_kind, opt_value) in pub_opts.overrides.iter() {
            int_opts.0[*opt_kind as usize] = ValOptionInt::from_pub(opt_value, sol);
        }
        int_opts
    }
    pub(in crate::svc::vast::val) fn get(&self, kind: ValKind) -> &ValOptionInt {
        &self.0[kind as usize]
    }
}

#[derive(Clone)]
pub(in crate::svc::vast::val) enum ValOptionInt {
    Enabled(ValOptionEnabledInt),
    Disabled,
}
impl ValOptionInt {
    fn from_pub(pub_opt: &OptionExt<ValEnabled>, sol: &SolarSystem) -> Self {
        match pub_opt {
            OptionExt::Disabled => Self::Disabled,
            OptionExt::Enabled => Self::Enabled(ValOptionEnabledInt { kfs: RSet::new() }),
            OptionExt::EnabledExtended(pub_opt_enabled) => Self::Enabled(ValOptionEnabledInt {
                kfs: pub_opt_enabled
                    .kfs
                    .iter()
                    .filter_map(|item_id| sol.u_data.items.int_id_by_ext_id(item_id))
                    .collect(),
            }),
        }
    }
}

#[derive(Clone)]
pub(in crate::svc::vast::val) struct ValOptionEnabledInt {
    pub(in crate::svc::vast::val) kfs: RSet<UItemId>,
}
