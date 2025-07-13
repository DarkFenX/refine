use rc::ItemMutCommon;

use crate::{
    cmd::{
        shared::get_primary_item,
        stats::options::{HStatOption, HStatOptionEhp, HStatOptionItemRr, HStatOptionReps, HStatResolvedOption},
    },
    info::{
        HItemStats,
        stats::{HStatLayerEhp, HStatLayerReps, HStatTank},
    },
    util::HExecError,
};

#[derive(educe::Educe, serde::Deserialize)]
#[educe(Default)]
pub(crate) struct HGetItemStatsCmd {
    #[serde(default)]
    #[educe(Default = true)]
    default: bool,
    agility: Option<bool>,
    align_time: Option<bool>,
    speed: Option<bool>,
    hp: Option<bool>,
    ehp: Option<HStatOption<HStatOptionEhp>>,
    wc_ehp: Option<bool>,
    resists: Option<bool>,
    rr_shield: Option<HStatOption<HStatOptionItemRr>>,
    rr_armor: Option<HStatOption<HStatOptionItemRr>>,
    rr_hull: Option<HStatOption<HStatOptionItemRr>>,
    rr_capacitor: Option<HStatOption<HStatOptionItemRr>>,
    reps: Option<HStatOption<HStatOptionReps>>,
}
impl HGetItemStatsCmd {
    pub(crate) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HItemStats, HExecError> {
        let mut core_item = get_primary_item(core_sol, item_id)?;
        let mut stats = HItemStats::new();
        if self.agility.unwrap_or(self.default) {
            stats.agility = core_item.get_stat_agility().into();
        }
        if self.align_time.unwrap_or(self.default) {
            stats.align_time = core_item.get_stat_align_time().into();
        }
        if self.speed.unwrap_or(self.default) {
            stats.speed = core_item.get_stat_speed().into();
        }
        if self.hp.unwrap_or(self.default) {
            stats.hp = core_item.get_stat_hp().into();
        }
        let ehp_opt = HStatResolvedOption::new(&self.ehp, self.default);
        if ehp_opt.enabled {
            stats.ehp = get_ehp_stats(&mut core_item, ehp_opt.options).into()
        }
        if self.wc_ehp.unwrap_or(self.default) {
            stats.wc_ehp = core_item.get_stat_wc_ehp().into();
        }
        let reps_opt = HStatResolvedOption::new(&self.reps, self.default);
        if ehp_opt.enabled {
            stats.reps = get_reps_stats(&mut core_item, reps_opt.options).into();
        }
        if self.resists.unwrap_or(self.default) {
            stats.resists = core_item.get_stat_resists().into();
        }
        let rr_shield_opt = HStatResolvedOption::new(&self.rr_shield, self.default);
        if rr_shield_opt.enabled {
            stats.rr_shield = get_shield_rr_stats(&mut core_item, rr_shield_opt.options).into();
        }
        let rr_armor_opt = HStatResolvedOption::new(&self.rr_armor, self.default);
        if rr_armor_opt.enabled {
            stats.rr_armor = get_armor_rr_stats(&mut core_item, rr_armor_opt.options).into();
        }
        let rr_hull_opt = HStatResolvedOption::new(&self.rr_hull, self.default);
        if rr_hull_opt.enabled {
            stats.rr_hull = get_hull_rr_stats(&mut core_item, rr_hull_opt.options).into();
        }
        let rr_cap_opt = HStatResolvedOption::new(&self.rr_capacitor, self.default);
        if rr_cap_opt.enabled {
            stats.rr_capacitor = get_cap_rr_stats(&mut core_item, rr_cap_opt.options).into();
        }
        Ok(stats)
    }
}

fn get_ehp_stats(
    core_item: &mut rc::ItemMut,
    options: Vec<HStatOptionEhp>,
) -> Option<Vec<Option<HStatTank<HStatLayerEhp>>>> {
    Some(
        options
            .into_iter()
            .map(|inner_opt| {
                let core_incoming_dps = inner_opt.incoming_dps.map(|h_incoming_dps| h_incoming_dps.into());
                core_item
                    .get_stat_ehp(core_incoming_dps.as_ref())
                    .map(|core_ehp| core_ehp.into())
            })
            .collect(),
    )
}

fn get_reps_stats(
    core_item: &mut rc::ItemMut,
    options: Vec<HStatOptionReps>,
) -> Option<Vec<Option<HStatTank<HStatLayerReps>>>> {
    Some(
        options
            .into_iter()
            .map(|_inner_opt| core_item.get_stat_reps().map(|core_reps| core_reps.into()))
            .collect(),
    )
}

fn get_shield_rr_stats(core_item: &mut rc::ItemMut, options: Vec<HStatOptionItemRr>) -> Option<Vec<rc::AttrVal>> {
    options
        .iter()
        .map(|inner_opt| {
            core_item.get_stat_rr_shield(inner_opt.spool.map(|spool| spool.into()), inner_opt.ignore_state)
        })
        .collect()
}

fn get_armor_rr_stats(core_item: &mut rc::ItemMut, options: Vec<HStatOptionItemRr>) -> Option<Vec<rc::AttrVal>> {
    options
        .iter()
        .map(|inner_opt| core_item.get_stat_rr_armor(inner_opt.spool.map(|spool| spool.into()), inner_opt.ignore_state))
        .collect()
}

fn get_hull_rr_stats(core_item: &mut rc::ItemMut, options: Vec<HStatOptionItemRr>) -> Option<Vec<rc::AttrVal>> {
    options
        .iter()
        .map(|inner_opt| core_item.get_stat_rr_hull(inner_opt.spool.map(|spool| spool.into()), inner_opt.ignore_state))
        .collect()
}

fn get_cap_rr_stats(core_item: &mut rc::ItemMut, options: Vec<HStatOptionItemRr>) -> Option<Vec<rc::AttrVal>> {
    options
        .iter()
        .map(|inner_opt| {
            core_item.get_stat_rr_capacitor(inner_opt.spool.map(|spool| spool.into()), inner_opt.ignore_state)
        })
        .collect()
}
