use crate::{
    DpsProfile, Fit, ItemId, PValue, UnitInterval,
    stats::{
        StatCapBlcRegen, StatCritOptions, StatDmgItemKinds, StatJumpRange, StatMiningItemKinds, StatMiningResourceKind,
        StatNeutItemKinds, StatOutRepItemKinds, StatTime, StatTimeOptions, StatTimeOptionsBurst, StatTimeOptionsSim,
    },
    svc::{
        cycle::CseqMap,
        err::IntStatItemError,
        vast::{StatCapBlcNosfsOptionsInt, StatCapBlcSrcKindsInt},
    },
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Fit<'s> {
    pub fn benchmark_stats(&mut self, projectee_item_id: ItemId, iterations: usize) {
        let u_fit = self.core.u_data.fits.get(self.uid);
        let ship_uid = u_fit.ship.unwrap();
        let char_uid = u_fit.character.unwrap();
        let projectee_item_uid = self.core.u_data.items.int_id_by_ext_id(&projectee_item_id).unwrap();
        let time_burst = StatTimeOptions::Burst(StatTimeOptionsBurst { spool: None });
        let time_sim_inf = StatTimeOptions::Sim(StatTimeOptionsSim {
            time: None,
            optional_reloads: None,
            rearm_minions: None,
        });
        let time_sim_1200 = StatTimeOptions::Sim(StatTimeOptionsSim {
            time: Some(StatTime::from_finite_f64_clamped(1200.0)),
            optional_reloads: None,
            rearm_minions: None,
        });
        let dmg_pattern_uniform = DpsProfile {
            em: PValue::ONE,
            thermal: PValue::ONE,
            kinetic: PValue::ONE,
            explosive: PValue::ONE,
            ..
        };
        let dmg_pattern_laser = DpsProfile {
            em: PValue::ONE,
            thermal: PValue::ONE,
            ..
        };
        let shield_perc_peak = UnitInterval::from_finite_f64_clamped(0.25);
        let dmg_item_kinds = StatDmgItemKinds::new(true);
        let mining_item_kinds = StatMiningItemKinds::new(true);
        let neut_item_kinds = StatNeutItemKinds::new(true);
        let rr_item_kinds = StatOutRepItemKinds::new(true);
        let cap_src_kinds_all = StatCapBlcSrcKindsInt {
            regen: Some(StatCapBlcRegen::default()),
            cap_injectors: true,
            nosfs: Some(StatCapBlcNosfsOptionsInt::default()),
            consumers: true,
            incoming_transfers: true,
            incoming_neuts: true,
        };
        let cap_src_kinds_positive = StatCapBlcSrcKindsInt {
            regen: Some(StatCapBlcRegen::default()),
            cap_injectors: true,
            nosfs: Some(StatCapBlcNosfsOptionsInt::default()),
            consumers: false,
            incoming_transfers: true,
            incoming_neuts: false,
        };
        let cap_src_kinds_negative = StatCapBlcSrcKindsInt {
            regen: None,
            cap_injectors: false,
            nosfs: None,
            consumers: true,
            incoming_transfers: false,
            incoming_neuts: true,
        };
        let jump_range = StatJumpRange::Max;
        let jump_passengers = Vec::new();
        let mut reuse_cseq_map = CseqMap::new();
        for _ in 0..iterations {
            ////////////////////////////////////////////////////////////////////////////////////////
            // Damage
            ////////////////////////////////////////////////////////////////////////////////////////
            self.core.svc.get_stat_fit_dmg_raw(
                &mut reuse_cseq_map,
                &self.core.u_data,
                self.uid,
                dmg_item_kinds,
                time_burst,
                StatCritOptions::default(),
            );
            self.core.svc.get_stat_fit_dmg_raw(
                &mut reuse_cseq_map,
                &self.core.u_data,
                self.uid,
                dmg_item_kinds,
                time_sim_inf,
                StatCritOptions::default(),
            );
            self.core.svc.get_stat_fit_dmg_raw(
                &mut reuse_cseq_map,
                &self.core.u_data,
                self.uid,
                dmg_item_kinds,
                time_sim_1200,
                StatCritOptions::default(),
            );
            self.core.svc.get_stat_fit_dmg_applied(
                &mut reuse_cseq_map,
                &self.core.u_data,
                self.uid,
                dmg_item_kinds,
                time_burst,
                StatCritOptions::default(),
                projectee_item_uid,
            );
            self.core.svc.get_stat_fit_dmg_applied(
                &mut reuse_cseq_map,
                &self.core.u_data,
                self.uid,
                dmg_item_kinds,
                time_sim_inf,
                StatCritOptions::default(),
                projectee_item_uid,
            );
            self.core.svc.get_stat_fit_dmg_applied(
                &mut reuse_cseq_map,
                &self.core.u_data,
                self.uid,
                dmg_item_kinds,
                time_sim_1200,
                StatCritOptions::default(),
                projectee_item_uid,
            );
            ////////////////////////////////////////////////////////////////////////////////////////
            // Mining
            ////////////////////////////////////////////////////////////////////////////////////////
            self.core.svc.get_stat_fit_mps(
                &mut reuse_cseq_map,
                &self.core.u_data,
                self.uid,
                mining_item_kinds,
                time_burst,
                StatMiningResourceKind::default(),
            );
            self.core.svc.get_stat_fit_mps(
                &mut reuse_cseq_map,
                &self.core.u_data,
                self.uid,
                mining_item_kinds,
                time_sim_inf,
                StatMiningResourceKind::default(),
            );
            self.core.svc.get_stat_fit_mps(
                &mut reuse_cseq_map,
                &self.core.u_data,
                self.uid,
                mining_item_kinds,
                time_sim_1200,
                StatMiningResourceKind::default(),
            );
            ////////////////////////////////////////////////////////////////////////////////////////
            // Neuts
            ////////////////////////////////////////////////////////////////////////////////////////
            self.core.svc.get_stat_fit_outgoing_nps(
                &mut reuse_cseq_map,
                &self.core.u_data,
                self.uid,
                neut_item_kinds,
                time_burst,
                None,
            );
            self.core.svc.get_stat_fit_outgoing_nps(
                &mut reuse_cseq_map,
                &self.core.u_data,
                self.uid,
                neut_item_kinds,
                time_sim_inf,
                None,
            );
            self.core.svc.get_stat_fit_outgoing_nps(
                &mut reuse_cseq_map,
                &self.core.u_data,
                self.uid,
                neut_item_kinds,
                time_sim_1200,
                None,
            );
            self.core.svc.get_stat_fit_outgoing_nps(
                &mut reuse_cseq_map,
                &self.core.u_data,
                self.uid,
                neut_item_kinds,
                time_burst,
                Some(projectee_item_uid),
            );
            self.core.svc.get_stat_fit_outgoing_nps(
                &mut reuse_cseq_map,
                &self.core.u_data,
                self.uid,
                neut_item_kinds,
                time_sim_inf,
                Some(projectee_item_uid),
            );
            self.core.svc.get_stat_fit_outgoing_nps(
                &mut reuse_cseq_map,
                &self.core.u_data,
                self.uid,
                neut_item_kinds,
                time_sim_1200,
                Some(projectee_item_uid),
            );
            ////////////////////////////////////////////////////////////////////////////////////////
            // RRs
            ////////////////////////////////////////////////////////////////////////////////////////
            self.core.svc.get_stat_fit_outgoing_rps(
                &mut reuse_cseq_map,
                &self.core.u_data,
                self.uid,
                rr_item_kinds,
                time_burst,
                None,
            );
            self.core.svc.get_stat_fit_outgoing_rps(
                &mut reuse_cseq_map,
                &self.core.u_data,
                self.uid,
                rr_item_kinds,
                time_sim_inf,
                None,
            );
            self.core.svc.get_stat_fit_outgoing_rps(
                &mut reuse_cseq_map,
                &self.core.u_data,
                self.uid,
                rr_item_kinds,
                time_sim_1200,
                None,
            );
            self.core.svc.get_stat_fit_outgoing_rps(
                &mut reuse_cseq_map,
                &self.core.u_data,
                self.uid,
                rr_item_kinds,
                time_burst,
                Some(projectee_item_uid),
            );
            self.core.svc.get_stat_fit_outgoing_rps(
                &mut reuse_cseq_map,
                &self.core.u_data,
                self.uid,
                rr_item_kinds,
                time_sim_inf,
                Some(projectee_item_uid),
            );
            self.core.svc.get_stat_fit_outgoing_rps(
                &mut reuse_cseq_map,
                &self.core.u_data,
                self.uid,
                rr_item_kinds,
                time_sim_1200,
                Some(projectee_item_uid),
            );
            ////////////////////////////////////////////////////////////////////////////////////////
            // Cap transfers
            ////////////////////////////////////////////////////////////////////////////////////////
            self.core
                .svc
                .get_stat_fit_outgoing_cps(&mut reuse_cseq_map, &self.core.u_data, self.uid, time_burst, None);
            self.core.svc.get_stat_fit_outgoing_cps(
                &mut reuse_cseq_map,
                &self.core.u_data,
                self.uid,
                time_sim_inf,
                None,
            );
            self.core.svc.get_stat_fit_outgoing_cps(
                &mut reuse_cseq_map,
                &self.core.u_data,
                self.uid,
                time_sim_1200,
                None,
            );
            self.core.svc.get_stat_fit_outgoing_cps(
                &mut reuse_cseq_map,
                &self.core.u_data,
                self.uid,
                time_burst,
                Some(projectee_item_uid),
            );
            self.core.svc.get_stat_fit_outgoing_cps(
                &mut reuse_cseq_map,
                &self.core.u_data,
                self.uid,
                time_sim_inf,
                Some(projectee_item_uid),
            );
            self.core.svc.get_stat_fit_outgoing_cps(
                &mut reuse_cseq_map,
                &self.core.u_data,
                self.uid,
                time_sim_1200,
                Some(projectee_item_uid),
            );
            ////////////////////////////////////////////////////////////////////////////////////////
            // Resources
            ////////////////////////////////////////////////////////////////////////////////////////
            self.core.svc.get_stat_fit_cpu(&self.core.u_data, self.uid, u_fit);
            self.core.svc.get_stat_fit_powergrid(&self.core.u_data, self.uid, u_fit);
            self.core
                .svc
                .get_stat_fit_calibration(&self.core.u_data, self.uid, u_fit);
            self.core
                .svc
                .get_stat_fit_drone_bay_volume(&self.core.u_data, self.uid, u_fit);
            self.core
                .svc
                .get_stat_fit_drone_bandwidth(&self.core.u_data, self.uid, u_fit);
            self.core
                .svc
                .get_stat_fit_fighter_bay_volume(&self.core.u_data, self.uid, u_fit);
            ////////////////////////////////////////////////////////////////////////////////////////
            // Slots
            ////////////////////////////////////////////////////////////////////////////////////////
            self.core
                .svc
                .get_stat_fit_high_slots(&self.core.u_data, self.uid, u_fit);
            self.core.svc.get_stat_fit_mid_slots(&self.core.u_data, self.uid, u_fit);
            self.core.svc.get_stat_fit_low_slots(&self.core.u_data, self.uid, u_fit);
            self.core
                .svc
                .get_stat_fit_turret_slots(&self.core.u_data, self.uid, u_fit);
            self.core
                .svc
                .get_stat_fit_launcher_slots(&self.core.u_data, self.uid, u_fit);
            self.core.svc.get_stat_fit_rig_slots(&self.core.u_data, self.uid, u_fit);
            self.core
                .svc
                .get_stat_fit_service_slots(&self.core.u_data, self.uid, u_fit);
            self.core
                .svc
                .get_stat_fit_subsystem_slots(&self.core.u_data, self.uid, u_fit);
            self.core
                .svc
                .get_stat_fit_launched_drones(&self.core.u_data, self.uid, u_fit);
            self.core
                .svc
                .get_stat_fit_launched_fighters(&self.core.u_data, self.uid, u_fit);
            self.core
                .svc
                .get_stat_fit_launched_light_fighters(&self.core.u_data, self.uid, u_fit);
            self.core
                .svc
                .get_stat_fit_launched_heavy_fighters(&self.core.u_data, self.uid, u_fit);
            self.core
                .svc
                .get_stat_fit_launched_support_fighters(&self.core.u_data, self.uid, u_fit);
            self.core
                .svc
                .get_stat_fit_launched_st_light_fighters(&self.core.u_data, self.uid, u_fit);
            self.core
                .svc
                .get_stat_fit_launched_st_heavy_fighters(&self.core.u_data, self.uid, u_fit);
            self.core
                .svc
                .get_stat_fit_launched_st_support_fighters(&self.core.u_data, self.uid, u_fit);
            ////////////////////////////////////////////////////////////////////////////////////////
            // Tank
            ////////////////////////////////////////////////////////////////////////////////////////
            self.core
                .svc
                .get_stat_item_resists(&self.core.u_data, ship_uid)
                .unwrap();
            self.core
                .svc
                .get_stat_item_hp(&mut reuse_cseq_map, &self.core.u_data, ship_uid)
                .unwrap();
            self.core
                .svc
                .get_stat_item_ehp(
                    &mut reuse_cseq_map,
                    &self.core.u_data,
                    ship_uid,
                    Some(dmg_pattern_uniform),
                )
                .unwrap();
            self.core
                .svc
                .get_stat_item_ehp(
                    &mut reuse_cseq_map,
                    &self.core.u_data,
                    ship_uid,
                    Some(dmg_pattern_laser),
                )
                .unwrap();
            self.core
                .svc
                .get_stat_item_wc_ehp(&mut reuse_cseq_map, &self.core.u_data, ship_uid)
                .unwrap();
            self.core
                .svc
                .get_stat_item_rps(
                    &mut reuse_cseq_map,
                    &self.core.u_data,
                    ship_uid,
                    time_burst,
                    shield_perc_peak,
                )
                .unwrap();
            self.core
                .svc
                .get_stat_item_rps(
                    &mut reuse_cseq_map,
                    &self.core.u_data,
                    ship_uid,
                    time_sim_inf,
                    shield_perc_peak,
                )
                .unwrap();
            self.core
                .svc
                .get_stat_item_rps(
                    &mut reuse_cseq_map,
                    &self.core.u_data,
                    ship_uid,
                    time_sim_1200,
                    shield_perc_peak,
                )
                .unwrap();
            self.core
                .svc
                .get_stat_item_erps(
                    &mut reuse_cseq_map,
                    &self.core.u_data,
                    ship_uid,
                    Some(dmg_pattern_uniform),
                    time_burst,
                    shield_perc_peak,
                )
                .unwrap();
            self.core
                .svc
                .get_stat_item_erps(
                    &mut reuse_cseq_map,
                    &self.core.u_data,
                    ship_uid,
                    Some(dmg_pattern_uniform),
                    time_sim_inf,
                    shield_perc_peak,
                )
                .unwrap();
            self.core
                .svc
                .get_stat_item_erps(
                    &mut reuse_cseq_map,
                    &self.core.u_data,
                    ship_uid,
                    Some(dmg_pattern_uniform),
                    time_sim_1200,
                    shield_perc_peak,
                )
                .unwrap();
            self.core
                .svc
                .get_stat_item_erps(
                    &mut reuse_cseq_map,
                    &self.core.u_data,
                    ship_uid,
                    Some(dmg_pattern_laser),
                    time_burst,
                    shield_perc_peak,
                )
                .unwrap();
            self.core
                .svc
                .get_stat_item_erps(
                    &mut reuse_cseq_map,
                    &self.core.u_data,
                    ship_uid,
                    Some(dmg_pattern_laser),
                    time_sim_inf,
                    shield_perc_peak,
                )
                .unwrap();
            self.core
                .svc
                .get_stat_item_erps(
                    &mut reuse_cseq_map,
                    &self.core.u_data,
                    ship_uid,
                    Some(dmg_pattern_laser),
                    time_sim_1200,
                    shield_perc_peak,
                )
                .unwrap();
            self.core
                .svc
                .get_stat_item_breach_resist(&self.core.u_data, ship_uid)
                .unwrap();
            ////////////////////////////////////////////////////////////////////////////////////////
            // Cap
            ////////////////////////////////////////////////////////////////////////////////////////
            self.core
                .svc
                .get_stat_item_cap_amount(&self.core.u_data, ship_uid)
                .unwrap();
            self.core
                .svc
                .get_stat_item_cap_balance(
                    &mut reuse_cseq_map,
                    &self.core.u_data,
                    ship_uid,
                    cap_src_kinds_all,
                    time_sim_inf,
                )
                .unwrap();
            self.core
                .svc
                .get_stat_item_cap_balance(
                    &mut reuse_cseq_map,
                    &self.core.u_data,
                    ship_uid,
                    cap_src_kinds_positive,
                    time_sim_inf,
                )
                .unwrap();
            self.core
                .svc
                .get_stat_item_cap_balance(
                    &mut reuse_cseq_map,
                    &self.core.u_data,
                    ship_uid,
                    cap_src_kinds_negative,
                    time_sim_inf,
                )
                .unwrap();
            self.core
                .svc
                .get_stat_item_neut_resist(&self.core.u_data, ship_uid)
                .unwrap();
            ////////////////////////////////////////////////////////////////////////////////////////
            // Sensors
            ////////////////////////////////////////////////////////////////////////////////////////
            self.core.svc.get_stat_item_locks(&self.core.u_data, ship_uid).unwrap();
            self.core
                .svc
                .get_stat_item_lock_range(&self.core.u_data, ship_uid)
                .unwrap();
            self.core
                .svc
                .get_stat_item_scan_res(&self.core.u_data, ship_uid)
                .unwrap();
            self.core
                .svc
                .get_stat_item_sensors(&self.core.u_data, ship_uid)
                .unwrap();
            self.core.svc.get_stat_dscan_range(&self.core.u_data, ship_uid).unwrap();
            match self.core.svc.get_stat_item_probing_size(&self.core.u_data, ship_uid) {
                Ok(..) | Err(IntStatItemError::StatSpecific(..)) => (),
                _ => panic!(),
            }
            self.core
                .svc
                .get_stat_item_incoming_jam(&mut reuse_cseq_map, &self.core.u_data, ship_uid, time_burst)
                .unwrap();
            self.core
                .svc
                .get_stat_item_incoming_jam(&mut reuse_cseq_map, &self.core.u_data, ship_uid, time_sim_inf)
                .unwrap();
            self.core
                .svc
                .get_stat_item_incoming_jam(&mut reuse_cseq_map, &self.core.u_data, ship_uid, time_sim_1200)
                .unwrap();
            ////////////////////////////////////////////////////////////////////////////////////////
            // Mobility
            ////////////////////////////////////////////////////////////////////////////////////////
            self.core.svc.get_stat_item_speed(&self.core.u_data, ship_uid).unwrap();
            match self.core.svc.get_stat_item_agility(&self.core.u_data, ship_uid) {
                Ok(..) | Err(IntStatItemError::StatSpecific(..)) => (),
                _ => panic!(),
            };
            match self.core.svc.get_stat_item_align_time(&self.core.u_data, ship_uid) {
                Ok(..) | Err(IntStatItemError::StatSpecific(..)) => (),
                _ => panic!(),
            };
            self.core
                .svc
                .get_stat_item_sig_radius(&self.core.u_data, ship_uid)
                .unwrap();
            self.core.svc.get_stat_item_mass(&self.core.u_data, ship_uid).unwrap();
            match self.core.svc.get_stat_item_warp_speed(&self.core.u_data, ship_uid) {
                Ok(..) | Err(IntStatItemError::StatSpecific(..)) => (),
                _ => panic!(),
            };
            match self.core.svc.get_stat_item_max_warp_range(&self.core.u_data, ship_uid) {
                Ok(..) | Err(IntStatItemError::StatSpecific(..)) => (),
                _ => panic!(),
            };
            match self
                .core
                .svc
                .get_stat_item_jump(&self.core.u_data, ship_uid, jump_range, &jump_passengers)
            {
                Ok(..) | Err(IntStatItemError::StatSpecific(..)) => (),
                _ => panic!(),
            };
            ////////////////////////////////////////////////////////////////////////////////////////
            // Misc
            ////////////////////////////////////////////////////////////////////////////////////////
            self.core
                .svc
                .get_stat_item_drone_control_range(&self.core.u_data, char_uid)
                .unwrap();
            self.core
                .svc
                .get_stat_item_can_warp(&self.core.u_data, ship_uid)
                .unwrap();
            self.core
                .svc
                .get_stat_item_can_jump_gate(&self.core.u_data, ship_uid)
                .unwrap();
            self.core
                .svc
                .get_stat_item_can_jump_wormhole(&self.core.u_data, ship_uid)
                .unwrap();
            self.core
                .svc
                .get_stat_item_can_jump_drive(&self.core.u_data, ship_uid)
                .unwrap();
            self.core
                .svc
                .get_stat_item_can_dock_station(&self.core.u_data, ship_uid)
                .unwrap();
            self.core
                .svc
                .get_stat_item_can_dock_citadel(&self.core.u_data, ship_uid)
                .unwrap();
            self.core
                .svc
                .get_stat_item_can_tether(&self.core.u_data, ship_uid)
                .unwrap();
        }
    }
}
