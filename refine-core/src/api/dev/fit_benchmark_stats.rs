use crate::{
    api::FitMut,
    misc::DpsProfile,
    num::{PValue, UnitInterval},
    svc::{
        cycle::CseqMap,
        vast::{
            StatCapBlcNosfsInt, StatCapBlcNosfsOptionsInt, StatCapBlcRegen, StatCapBlcRegenOptions,
            StatCapBlcSrcKindsInt, StatDmgItemKinds, StatJumpRange, StatMiningItemKinds, StatNeutItemKinds,
            StatOutRepItemKinds, StatTimeOptions, StatTimeOptionsBurst, StatTimeOptionsSim,
        },
    },
    ud::ItemId,
};

impl<'s> FitMut<'s> {
    pub fn benchmark_stats(&mut self, projectee_item_id: ItemId, iterations: usize) {
        let u_fit = self.sol.u_data.fits.get(self.uid);
        let ship_uid = u_fit.ship.unwrap();
        let char_uid = u_fit.character.unwrap();
        let projectee_item_uid = self.sol.u_data.items.int_id_by_ext_id(&projectee_item_id).unwrap();
        let time_burst = StatTimeOptions::Burst(StatTimeOptionsBurst { spool: None });
        let time_sim_inf = StatTimeOptions::Sim(StatTimeOptionsSim {
            time: None,
            optional_reloads: None,
            rearm_minions: None,
        });
        let time_sim_1200 = StatTimeOptions::Sim(StatTimeOptionsSim {
            time: Some(PValue::from_f64_clamped(1200.0)),
            optional_reloads: None,
            rearm_minions: None,
        });
        let dmg_pattern_uniform = DpsProfile::new(PValue::ONE, PValue::ONE, PValue::ONE, PValue::ONE, None);
        let dmg_pattern_laser = DpsProfile::new(PValue::ONE, PValue::ONE, PValue::ZERO, PValue::ZERO, None);
        let shield_perc_peak = UnitInterval::from_f64_clamped(0.25);
        let dmg_item_kinds = StatDmgItemKinds::all_enabled();
        let mining_item_kinds = StatMiningItemKinds::all_enabled();
        let neut_item_kinds = StatNeutItemKinds::all_enabled();
        let rr_item_kinds = StatOutRepItemKinds { default: true, .. };
        let cap_src_kinds_all = StatCapBlcSrcKindsInt {
            regen: StatCapBlcRegen::Enabled(StatCapBlcRegenOptions { .. }),
            cap_injectors: true,
            nosfs: StatCapBlcNosfsInt::Enabled(StatCapBlcNosfsOptionsInt { .. }),
            consumers: true,
            incoming_transfers: true,
            incoming_neuts: true,
        };
        let cap_src_kinds_positive = StatCapBlcSrcKindsInt {
            regen: StatCapBlcRegen::Enabled(StatCapBlcRegenOptions { .. }),
            cap_injectors: true,
            nosfs: StatCapBlcNosfsInt::Enabled(StatCapBlcNosfsOptionsInt { .. }),
            consumers: false,
            incoming_transfers: true,
            incoming_neuts: false,
        };
        let cap_src_kinds_negative = StatCapBlcSrcKindsInt {
            regen: StatCapBlcRegen::Disabled,
            cap_injectors: false,
            nosfs: StatCapBlcNosfsInt::Disabled,
            consumers: true,
            incoming_transfers: false,
            incoming_neuts: true,
        };
        let jump_range = StatJumpRange::Max;
        let jump_passengers = Vec::new();
        let mut reuse_cseq_map = CseqMap::new();
        for _ in 0..iterations {
            ////////////////////////////////////////////////////////////////////////////////////////////////////////////
            // Damage
            ////////////////////////////////////////////////////////////////////////////////////////////////////////////
            self.sol.svc.get_stat_fit_dmg_raw(
                &mut reuse_cseq_map,
                &self.sol.u_data,
                self.uid,
                dmg_item_kinds,
                time_burst,
            );
            self.sol.svc.get_stat_fit_dmg_raw(
                &mut reuse_cseq_map,
                &self.sol.u_data,
                self.uid,
                dmg_item_kinds,
                time_sim_inf,
            );
            self.sol.svc.get_stat_fit_dmg_raw(
                &mut reuse_cseq_map,
                &self.sol.u_data,
                self.uid,
                dmg_item_kinds,
                time_sim_1200,
            );
            self.sol.svc.get_stat_fit_dmg_applied(
                &mut reuse_cseq_map,
                &self.sol.u_data,
                self.uid,
                dmg_item_kinds,
                time_burst,
                projectee_item_uid,
            );
            self.sol.svc.get_stat_fit_dmg_applied(
                &mut reuse_cseq_map,
                &self.sol.u_data,
                self.uid,
                dmg_item_kinds,
                time_sim_inf,
                projectee_item_uid,
            );
            self.sol.svc.get_stat_fit_dmg_applied(
                &mut reuse_cseq_map,
                &self.sol.u_data,
                self.uid,
                dmg_item_kinds,
                time_sim_1200,
                projectee_item_uid,
            );
            ////////////////////////////////////////////////////////////////////////////////////////////////////////////
            // Mining
            ////////////////////////////////////////////////////////////////////////////////////////////////////////////
            self.sol.svc.get_stat_fit_mps(
                &mut reuse_cseq_map,
                &self.sol.u_data,
                self.uid,
                mining_item_kinds,
                time_burst,
                false,
            );
            self.sol.svc.get_stat_fit_mps(
                &mut reuse_cseq_map,
                &self.sol.u_data,
                self.uid,
                mining_item_kinds,
                time_sim_inf,
                false,
            );
            self.sol.svc.get_stat_fit_mps(
                &mut reuse_cseq_map,
                &self.sol.u_data,
                self.uid,
                mining_item_kinds,
                time_sim_1200,
                false,
            );
            ////////////////////////////////////////////////////////////////////////////////////////////////////////////
            // Neuts
            ////////////////////////////////////////////////////////////////////////////////////////////////////////////
            self.sol.svc.get_stat_fit_outgoing_nps(
                &mut reuse_cseq_map,
                &self.sol.u_data,
                self.uid,
                neut_item_kinds,
                time_burst,
                None,
            );
            self.sol.svc.get_stat_fit_outgoing_nps(
                &mut reuse_cseq_map,
                &self.sol.u_data,
                self.uid,
                neut_item_kinds,
                time_sim_inf,
                None,
            );
            self.sol.svc.get_stat_fit_outgoing_nps(
                &mut reuse_cseq_map,
                &self.sol.u_data,
                self.uid,
                neut_item_kinds,
                time_sim_1200,
                None,
            );
            self.sol.svc.get_stat_fit_outgoing_nps(
                &mut reuse_cseq_map,
                &self.sol.u_data,
                self.uid,
                neut_item_kinds,
                time_burst,
                Some(projectee_item_uid),
            );
            self.sol.svc.get_stat_fit_outgoing_nps(
                &mut reuse_cseq_map,
                &self.sol.u_data,
                self.uid,
                neut_item_kinds,
                time_sim_inf,
                Some(projectee_item_uid),
            );
            self.sol.svc.get_stat_fit_outgoing_nps(
                &mut reuse_cseq_map,
                &self.sol.u_data,
                self.uid,
                neut_item_kinds,
                time_sim_1200,
                Some(projectee_item_uid),
            );
            ////////////////////////////////////////////////////////////////////////////////////////////////////////////
            // RRs
            ////////////////////////////////////////////////////////////////////////////////////////////////////////////
            self.sol.svc.get_stat_fit_outgoing_rps(
                &mut reuse_cseq_map,
                &self.sol.u_data,
                self.uid,
                rr_item_kinds,
                time_burst,
                None,
            );
            self.sol.svc.get_stat_fit_outgoing_rps(
                &mut reuse_cseq_map,
                &self.sol.u_data,
                self.uid,
                rr_item_kinds,
                time_sim_inf,
                None,
            );
            self.sol.svc.get_stat_fit_outgoing_rps(
                &mut reuse_cseq_map,
                &self.sol.u_data,
                self.uid,
                rr_item_kinds,
                time_sim_1200,
                None,
            );
            self.sol.svc.get_stat_fit_outgoing_rps(
                &mut reuse_cseq_map,
                &self.sol.u_data,
                self.uid,
                rr_item_kinds,
                time_burst,
                Some(projectee_item_uid),
            );
            self.sol.svc.get_stat_fit_outgoing_rps(
                &mut reuse_cseq_map,
                &self.sol.u_data,
                self.uid,
                rr_item_kinds,
                time_sim_inf,
                Some(projectee_item_uid),
            );
            self.sol.svc.get_stat_fit_outgoing_rps(
                &mut reuse_cseq_map,
                &self.sol.u_data,
                self.uid,
                rr_item_kinds,
                time_sim_1200,
                Some(projectee_item_uid),
            );
            ////////////////////////////////////////////////////////////////////////////////////////////////////////////
            // Cap transfers
            ////////////////////////////////////////////////////////////////////////////////////////////////////////////
            self.sol
                .svc
                .get_stat_fit_outgoing_cps(&mut reuse_cseq_map, &self.sol.u_data, self.uid, time_burst, None);
            self.sol
                .svc
                .get_stat_fit_outgoing_cps(&mut reuse_cseq_map, &self.sol.u_data, self.uid, time_sim_inf, None);
            self.sol.svc.get_stat_fit_outgoing_cps(
                &mut reuse_cseq_map,
                &self.sol.u_data,
                self.uid,
                time_sim_1200,
                None,
            );
            self.sol.svc.get_stat_fit_outgoing_cps(
                &mut reuse_cseq_map,
                &self.sol.u_data,
                self.uid,
                time_burst,
                Some(projectee_item_uid),
            );
            self.sol.svc.get_stat_fit_outgoing_cps(
                &mut reuse_cseq_map,
                &self.sol.u_data,
                self.uid,
                time_sim_inf,
                Some(projectee_item_uid),
            );
            self.sol.svc.get_stat_fit_outgoing_cps(
                &mut reuse_cseq_map,
                &self.sol.u_data,
                self.uid,
                time_sim_1200,
                Some(projectee_item_uid),
            );
            ////////////////////////////////////////////////////////////////////////////////////////////////////////////
            // Resources
            ////////////////////////////////////////////////////////////////////////////////////////////////////////////
            self.sol.svc.get_stat_fit_cpu(&self.sol.u_data, self.uid, u_fit);
            self.sol.svc.get_stat_fit_powergrid(&self.sol.u_data, self.uid, u_fit);
            self.sol.svc.get_stat_fit_calibration(&self.sol.u_data, self.uid, u_fit);
            self.sol
                .svc
                .get_stat_fit_drone_bay_volume(&self.sol.u_data, self.uid, u_fit);
            self.sol
                .svc
                .get_stat_fit_drone_bandwidth(&self.sol.u_data, self.uid, u_fit);
            self.sol
                .svc
                .get_stat_fit_fighter_bay_volume(&self.sol.u_data, self.uid, u_fit);
            ////////////////////////////////////////////////////////////////////////////////////////////////////////////
            // Slots
            ////////////////////////////////////////////////////////////////////////////////////////////////////////////
            self.sol.svc.get_stat_fit_high_slots(&self.sol.u_data, self.uid, u_fit);
            self.sol.svc.get_stat_fit_mid_slots(&self.sol.u_data, self.uid, u_fit);
            self.sol.svc.get_stat_fit_low_slots(&self.sol.u_data, self.uid, u_fit);
            self.sol
                .svc
                .get_stat_fit_turret_slots(&self.sol.u_data, self.uid, u_fit);
            self.sol
                .svc
                .get_stat_fit_launcher_slots(&self.sol.u_data, self.uid, u_fit);
            self.sol.svc.get_stat_fit_rig_slots(&self.sol.u_data, self.uid, u_fit);
            self.sol
                .svc
                .get_stat_fit_service_slots(&self.sol.u_data, self.uid, u_fit);
            self.sol
                .svc
                .get_stat_fit_subsystem_slots(&self.sol.u_data, self.uid, u_fit);
            self.sol
                .svc
                .get_stat_fit_launched_drones(&self.sol.u_data, self.uid, u_fit);
            self.sol
                .svc
                .get_stat_fit_launched_fighters(&self.sol.u_data, self.uid, u_fit);
            self.sol
                .svc
                .get_stat_fit_launched_light_fighters(&self.sol.u_data, self.uid, u_fit);
            self.sol
                .svc
                .get_stat_fit_launched_heavy_fighters(&self.sol.u_data, self.uid, u_fit);
            self.sol
                .svc
                .get_stat_fit_launched_support_fighters(&self.sol.u_data, self.uid, u_fit);
            self.sol
                .svc
                .get_stat_fit_launched_st_light_fighters(&self.sol.u_data, self.uid, u_fit);
            self.sol
                .svc
                .get_stat_fit_launched_st_heavy_fighters(&self.sol.u_data, self.uid, u_fit);
            self.sol
                .svc
                .get_stat_fit_launched_st_support_fighters(&self.sol.u_data, self.uid, u_fit);
            ////////////////////////////////////////////////////////////////////////////////////////////////////////////
            // Tank
            ////////////////////////////////////////////////////////////////////////////////////////////////////////////
            self.sol.svc.get_stat_item_resists(&self.sol.u_data, ship_uid).unwrap();
            self.sol
                .svc
                .get_stat_item_hp(&mut reuse_cseq_map, &self.sol.u_data, ship_uid)
                .unwrap();
            self.sol
                .svc
                .get_stat_item_ehp(
                    &mut reuse_cseq_map,
                    &self.sol.u_data,
                    ship_uid,
                    Some(dmg_pattern_uniform),
                )
                .unwrap();
            self.sol
                .svc
                .get_stat_item_ehp(&mut reuse_cseq_map, &self.sol.u_data, ship_uid, Some(dmg_pattern_laser))
                .unwrap();
            self.sol
                .svc
                .get_stat_item_wc_ehp(&mut reuse_cseq_map, &self.sol.u_data, ship_uid)
                .unwrap();
            self.sol
                .svc
                .get_stat_item_rps(
                    &mut reuse_cseq_map,
                    &self.sol.u_data,
                    ship_uid,
                    time_burst,
                    shield_perc_peak,
                )
                .unwrap();
            self.sol
                .svc
                .get_stat_item_rps(
                    &mut reuse_cseq_map,
                    &self.sol.u_data,
                    ship_uid,
                    time_sim_inf,
                    shield_perc_peak,
                )
                .unwrap();
            self.sol
                .svc
                .get_stat_item_rps(
                    &mut reuse_cseq_map,
                    &self.sol.u_data,
                    ship_uid,
                    time_sim_1200,
                    shield_perc_peak,
                )
                .unwrap();
            self.sol
                .svc
                .get_stat_item_erps(
                    &mut reuse_cseq_map,
                    &self.sol.u_data,
                    ship_uid,
                    Some(dmg_pattern_uniform),
                    time_burst,
                    shield_perc_peak,
                )
                .unwrap();
            self.sol
                .svc
                .get_stat_item_erps(
                    &mut reuse_cseq_map,
                    &self.sol.u_data,
                    ship_uid,
                    Some(dmg_pattern_uniform),
                    time_sim_inf,
                    shield_perc_peak,
                )
                .unwrap();
            self.sol
                .svc
                .get_stat_item_erps(
                    &mut reuse_cseq_map,
                    &self.sol.u_data,
                    ship_uid,
                    Some(dmg_pattern_uniform),
                    time_sim_1200,
                    shield_perc_peak,
                )
                .unwrap();
            self.sol
                .svc
                .get_stat_item_erps(
                    &mut reuse_cseq_map,
                    &self.sol.u_data,
                    ship_uid,
                    Some(dmg_pattern_laser),
                    time_burst,
                    shield_perc_peak,
                )
                .unwrap();
            self.sol
                .svc
                .get_stat_item_erps(
                    &mut reuse_cseq_map,
                    &self.sol.u_data,
                    ship_uid,
                    Some(dmg_pattern_laser),
                    time_sim_inf,
                    shield_perc_peak,
                )
                .unwrap();
            self.sol
                .svc
                .get_stat_item_erps(
                    &mut reuse_cseq_map,
                    &self.sol.u_data,
                    ship_uid,
                    Some(dmg_pattern_laser),
                    time_sim_1200,
                    shield_perc_peak,
                )
                .unwrap();
            self.sol
                .svc
                .get_stat_item_breach_resist(&self.sol.u_data, ship_uid)
                .unwrap();
            ////////////////////////////////////////////////////////////////////////////////////////////////////////////
            // Cap
            ////////////////////////////////////////////////////////////////////////////////////////////////////////////
            self.sol
                .svc
                .get_stat_item_cap_amount(&self.sol.u_data, ship_uid)
                .unwrap();
            self.sol
                .svc
                .get_stat_item_cap_balance(
                    &mut reuse_cseq_map,
                    &self.sol.u_data,
                    ship_uid,
                    cap_src_kinds_all,
                    time_sim_inf,
                )
                .unwrap();
            self.sol
                .svc
                .get_stat_item_cap_balance(
                    &mut reuse_cseq_map,
                    &self.sol.u_data,
                    ship_uid,
                    cap_src_kinds_positive,
                    time_sim_inf,
                )
                .unwrap();
            self.sol
                .svc
                .get_stat_item_cap_balance(
                    &mut reuse_cseq_map,
                    &self.sol.u_data,
                    ship_uid,
                    cap_src_kinds_negative,
                    time_sim_inf,
                )
                .unwrap();
            self.sol
                .svc
                .get_stat_item_neut_resist(&self.sol.u_data, ship_uid)
                .unwrap();
            ////////////////////////////////////////////////////////////////////////////////////////////////////////////
            // Sensors
            ////////////////////////////////////////////////////////////////////////////////////////////////////////////
            self.sol.svc.get_stat_item_locks(&self.sol.u_data, ship_uid).unwrap();
            self.sol
                .svc
                .get_stat_item_lock_range(&self.sol.u_data, ship_uid)
                .unwrap();
            self.sol.svc.get_stat_item_scan_res(&self.sol.u_data, ship_uid).unwrap();
            self.sol.svc.get_stat_item_sensors(&self.sol.u_data, ship_uid).unwrap();
            self.sol.svc.get_stat_dscan_range(&self.sol.u_data, ship_uid).unwrap();
            self.sol
                .svc
                .get_stat_item_probing_size(&self.sol.u_data, ship_uid)
                .unwrap();
            self.sol
                .svc
                .get_stat_item_incoming_jam(&mut reuse_cseq_map, &self.sol.u_data, ship_uid, time_burst)
                .unwrap();
            self.sol
                .svc
                .get_stat_item_incoming_jam(&mut reuse_cseq_map, &self.sol.u_data, ship_uid, time_sim_inf)
                .unwrap();
            self.sol
                .svc
                .get_stat_item_incoming_jam(&mut reuse_cseq_map, &self.sol.u_data, ship_uid, time_sim_1200)
                .unwrap();
            ////////////////////////////////////////////////////////////////////////////////////////////////////////////
            // Mobility
            ////////////////////////////////////////////////////////////////////////////////////////////////////////////
            self.sol.svc.get_stat_item_speed(&self.sol.u_data, ship_uid).unwrap();
            self.sol.svc.get_stat_item_agility(&self.sol.u_data, ship_uid).unwrap();
            self.sol
                .svc
                .get_stat_item_align_time(&self.sol.u_data, ship_uid)
                .unwrap();
            self.sol
                .svc
                .get_stat_item_sig_radius(&self.sol.u_data, ship_uid)
                .unwrap();
            self.sol.svc.get_stat_item_mass(&self.sol.u_data, ship_uid).unwrap();
            self.sol
                .svc
                .get_stat_item_warp_speed(&self.sol.u_data, ship_uid)
                .unwrap();
            self.sol
                .svc
                .get_stat_item_max_warp_range(&self.sol.u_data, ship_uid)
                .unwrap();
            self.sol
                .svc
                .get_stat_item_jump(&self.sol.u_data, ship_uid, jump_range, &jump_passengers)
                .unwrap();
            ////////////////////////////////////////////////////////////////////////////////////////////////////////////
            // Misc
            ////////////////////////////////////////////////////////////////////////////////////////////////////////////
            self.sol
                .svc
                .get_stat_item_drone_control_range(&self.sol.u_data, char_uid)
                .unwrap();
            self.sol.svc.get_stat_item_can_warp(&self.sol.u_data, ship_uid).unwrap();
            self.sol
                .svc
                .get_stat_item_can_jump_gate(&self.sol.u_data, ship_uid)
                .unwrap();
            self.sol
                .svc
                .get_stat_item_can_jump_wormhole(&self.sol.u_data, ship_uid)
                .unwrap();
            self.sol
                .svc
                .get_stat_item_can_jump_drive(&self.sol.u_data, ship_uid)
                .unwrap();
            self.sol
                .svc
                .get_stat_item_can_dock_station(&self.sol.u_data, ship_uid)
                .unwrap();
            self.sol
                .svc
                .get_stat_item_can_dock_citadel(&self.sol.u_data, ship_uid)
                .unwrap();
            self.sol
                .svc
                .get_stat_item_can_tether(&self.sol.u_data, ship_uid)
                .unwrap();
        }
    }
}
