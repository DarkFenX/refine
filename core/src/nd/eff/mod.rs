use std::{collections::HashMap, sync::LazyLock};

pub(crate) use eff::{
    NBreacherDmgGetter, NCalcCustomizer, NDmgKindGetter, NEcmGetter, NEffect, NEffectHc, NLocalRepGetter,
    NNormalDmgGetter, NProjMultGetter, NRemoteRepGetter, NSpoolResolver,
};
pub(crate) use shared::{NEffectCharge, NEffectChargeDepl, NEffectChargeLoc, NEffectDmgKind};

use crate::ad;

mod eff;
mod eff_c1_char_missile_dmg;
mod eff_c2_aar_paste_boost;
mod eff_c3_stasis_web_probe;
mod eff_c4_missile_flight_time;
mod eff_c5_stability_generator_electric;
mod eff_c6_stability_generator_plasma;
mod eff_c7_stability_generator_exotic;
mod eff_c8_stability_generator_gamma;
mod eff_d101_use_missiles;
mod eff_d103_defender_missile_launching;
mod eff_d104_fof_missile_launching;
mod eff_d10_tgt_attack;
mod eff_d11691_debuff_lance;
mod eff_d12126_micro_jump_portal_drive_capital;
mod eff_d12174_dot_missile_launching;
mod eff_d1730_drone_dmg_bonus;
mod eff_d1851_self_rof;
mod eff_d26_structure_repair;
mod eff_d27_armor_repair;
mod eff_d2971_bomb_launching;
mod eff_d3380_warp_disrupt_sphere;
mod eff_d34_projectile_fired;
mod eff_d3773_hardpoint_modifier_effect;
mod eff_d3774_slot_modifier;
mod eff_d38_emp_wave;
mod eff_d4921_micro_jump_drive;
mod eff_d4928_adaptive_armor_hardener;
mod eff_d4936_fueled_shield_boosting;
mod eff_d4_shield_boosting;
mod eff_d5275_fueled_armor_repair;
mod eff_d6184_ship_mod_remote_capacitor_transmitter;
mod eff_d6185_ship_mod_remote_hull_repairer;
mod eff_d6186_ship_mod_remote_shield_booster;
mod eff_d6188_ship_mod_remote_armor_repairer;
mod eff_d6208_micro_jump_portal_drive;
mod eff_d6222_struct_warp_scram_block_mwd_with_npc;
mod eff_d6422_remote_sensor_damp_falloff;
mod eff_d6423_ship_module_guidance_disruptor;
mod eff_d6424_ship_mod_tracking_disruptor;
mod eff_d6426_remote_webifier_falloff;
mod eff_d6427_remote_sensor_boost_falloff;
mod eff_d6443_point_defense;
mod eff_d6470_remote_ecm_falloff;
mod eff_d6476_doomsday_aoe_web;
mod eff_d6479_doomsday_aoe_track;
mod eff_d6481_doomsday_aoe_damp;
mod eff_d6485_ftr_abil_bomb;
mod eff_d6513_doomsday_aoe_ecm;
mod eff_d660_missile_em_dmg_bonus;
mod eff_d661_missile_expl_dmg_bonus;
mod eff_d662_missile_therm_dmg_bonus;
mod eff_d6651_ship_mod_ancillary_remote_armor_repairer;
mod eff_d6652_ship_mod_ancillary_remote_shield_booster;
mod eff_d6682_struct_mod_effect_stasis_webifier;
mod eff_d6684_struct_mod_effect_remote_sensor_dampener;
mod eff_d6685_struct_mod_effect_ecm;
mod eff_d6686_struct_mod_effect_weapon_disruption;
mod eff_d6687_npc_entity_remote_armor_repairer;
mod eff_d6688_npc_entity_remote_shield_booster;
mod eff_d6689_npc_entity_remote_hull_repairer;
mod eff_d668_missile_kin_dmg_bonus;
mod eff_d6690_remote_webifier_entity;
mod eff_d6693_remote_sensor_damp_entity;
mod eff_d6694_npc_entity_weapon_disruptor;
mod eff_d6695_entity_ecm_falloff;
mod eff_d6714_ecm_burst_jammer;
mod eff_d6730_mod_bonus_microwarpdrive;
mod eff_d6731_mod_bonus_afterburner;
mod eff_d6732_warfare_link_armor;
mod eff_d6733_warfare_link_shield;
mod eff_d6734_warfare_link_skirmish;
mod eff_d6735_warfare_link_info;
mod eff_d6736_warfare_link_mining;
mod eff_d6753_mod_titan_effect_generator;
mod eff_d67_mining_laser;
mod eff_d6848_ship_mod_focused_warp_scram_script;
mod eff_d6849_ship_mod_focused_warp_disrupt_script;
mod eff_d6995_tgt_disintegrator_attack;
mod eff_d7050_aoe_beacon_bioluminescence_cloud;
mod eff_d7051_aoe_beacon_caustic_cloud;
mod eff_d7053_aoe_beacon_pulse_01;
mod eff_d7058_aoe_beacon_filament_cloud;
mod eff_d7059_weather_caustic_toxin;
mod eff_d7060_weather_darkness;
mod eff_d7061_weather_electric_storm;
mod eff_d7062_weather_infernal;
mod eff_d7063_weather_xenon_gas;
mod eff_d7166_ship_mod_remote_armor_mutadaptive_repairer;
mod eff_d8037_chain_lightning;
mod eff_d848_cloaking_targeting_delay_bonus;
mod eff_d9_missile_launching;
mod shared;
mod test;

pub(crate) static N_EFFECTS: LazyLock<Vec<NEffect>> = LazyLock::new(get_n_effects);
pub(crate) static N_EFFECT_MAP: LazyLock<HashMap<ad::AEffectId, NEffect>> = LazyLock::new(get_n_effect_map);

fn get_n_effects() -> Vec<NEffect> {
    vec![
        eff_c1_char_missile_dmg::mk_n_effect(),
        eff_c2_aar_paste_boost::mk_n_effect(),
        eff_c3_stasis_web_probe::mk_n_effect(),
        eff_c4_missile_flight_time::mk_n_effect(),
        eff_c5_stability_generator_electric::mk_n_effect(),
        eff_c6_stability_generator_plasma::mk_n_effect(),
        eff_c7_stability_generator_exotic::mk_n_effect(),
        eff_c8_stability_generator_gamma::mk_n_effect(),
        eff_d4_shield_boosting::mk_n_effect(),
        eff_d9_missile_launching::mk_n_effect(),
        eff_d10_tgt_attack::mk_n_effect(),
        eff_d26_structure_repair::mk_n_effect(),
        eff_d27_armor_repair::mk_n_effect(),
        eff_d34_projectile_fired::mk_n_effect(),
        eff_d38_emp_wave::mk_n_effect(),
        eff_d67_mining_laser::mk_n_effect(),
        eff_d101_use_missiles::mk_n_effect(),
        eff_d103_defender_missile_launching::mk_n_effect(),
        eff_d104_fof_missile_launching::mk_n_effect(),
        eff_d660_missile_em_dmg_bonus::mk_n_effect(),
        eff_d661_missile_expl_dmg_bonus::mk_n_effect(),
        eff_d662_missile_therm_dmg_bonus::mk_n_effect(),
        eff_d668_missile_kin_dmg_bonus::mk_n_effect(),
        eff_d848_cloaking_targeting_delay_bonus::mk_n_effect(),
        eff_d1730_drone_dmg_bonus::mk_n_effect(),
        eff_d1851_self_rof::mk_n_effect(),
        eff_d2971_bomb_launching::mk_n_effect(),
        eff_d3380_warp_disrupt_sphere::mk_n_effect(),
        eff_d3773_hardpoint_modifier_effect::mk_n_effect(),
        eff_d3774_slot_modifier::mk_n_effect(),
        eff_d4921_micro_jump_drive::mk_n_effect(),
        eff_d4928_adaptive_armor_hardener::mk_n_effect(),
        eff_d4936_fueled_shield_boosting::mk_n_effect(),
        eff_d5275_fueled_armor_repair::mk_n_effect(),
        eff_d6184_ship_mod_remote_capacitor_transmitter::mk_n_effect(),
        eff_d6185_ship_mod_remote_hull_repairer::mk_n_effect(),
        eff_d6186_ship_mod_remote_shield_booster::mk_n_effect(),
        eff_d6188_ship_mod_remote_armor_repairer::mk_n_effect(),
        eff_d6208_micro_jump_portal_drive::mk_n_effect(),
        eff_d6222_struct_warp_scram_block_mwd_with_npc::mk_n_effect(),
        eff_d6422_remote_sensor_damp_falloff::mk_n_effect(),
        eff_d6423_ship_module_guidance_disruptor::mk_n_effect(),
        eff_d6424_ship_mod_tracking_disruptor::mk_n_effect(),
        eff_d6426_remote_webifier_falloff::mk_n_effect(),
        eff_d6427_remote_sensor_boost_falloff::mk_n_effect(),
        eff_d6443_point_defense::mk_n_effect(),
        eff_d6470_remote_ecm_falloff::mk_n_effect(),
        eff_d6476_doomsday_aoe_web::mk_n_effect(),
        eff_d6479_doomsday_aoe_track::mk_n_effect(),
        eff_d6481_doomsday_aoe_damp::mk_n_effect(),
        eff_d6485_ftr_abil_bomb::mk_n_effect(),
        eff_d6513_doomsday_aoe_ecm::mk_n_effect(),
        eff_d6651_ship_mod_ancillary_remote_armor_repairer::mk_n_effect(),
        eff_d6652_ship_mod_ancillary_remote_shield_booster::mk_n_effect(),
        eff_d6682_struct_mod_effect_stasis_webifier::mk_n_effect(),
        eff_d6684_struct_mod_effect_remote_sensor_dampener::mk_n_effect(),
        eff_d6685_struct_mod_effect_ecm::mk_n_effect(),
        eff_d6686_struct_mod_effect_weapon_disruption::mk_n_effect(),
        eff_d6687_npc_entity_remote_armor_repairer::mk_n_effect(),
        eff_d6688_npc_entity_remote_shield_booster::mk_n_effect(),
        eff_d6689_npc_entity_remote_hull_repairer::mk_n_effect(),
        eff_d6690_remote_webifier_entity::mk_n_effect(),
        eff_d6693_remote_sensor_damp_entity::mk_n_effect(),
        eff_d6694_npc_entity_weapon_disruptor::mk_n_effect(),
        eff_d6695_entity_ecm_falloff::mk_n_effect(),
        eff_d6714_ecm_burst_jammer::mk_n_effect(),
        eff_d6730_mod_bonus_microwarpdrive::mk_n_effect(),
        eff_d6731_mod_bonus_afterburner::mk_n_effect(),
        eff_d6732_warfare_link_armor::mk_n_effect(),
        eff_d6733_warfare_link_shield::mk_n_effect(),
        eff_d6734_warfare_link_skirmish::mk_n_effect(),
        eff_d6735_warfare_link_info::mk_n_effect(),
        eff_d6736_warfare_link_mining::mk_n_effect(),
        eff_d6753_mod_titan_effect_generator::mk_n_effect(),
        eff_d6848_ship_mod_focused_warp_scram_script::mk_n_effect(),
        eff_d6849_ship_mod_focused_warp_disrupt_script::mk_n_effect(),
        eff_d6995_tgt_disintegrator_attack::mk_n_effect(),
        eff_d7050_aoe_beacon_bioluminescence_cloud::mk_n_effect(),
        eff_d7051_aoe_beacon_caustic_cloud::mk_n_effect(),
        eff_d7053_aoe_beacon_pulse_01::mk_n_effect(),
        eff_d7058_aoe_beacon_filament_cloud::mk_n_effect(),
        eff_d7059_weather_caustic_toxin::mk_n_effect(),
        eff_d7060_weather_darkness::mk_n_effect(),
        eff_d7061_weather_electric_storm::mk_n_effect(),
        eff_d7062_weather_infernal::mk_n_effect(),
        eff_d7063_weather_xenon_gas::mk_n_effect(),
        eff_d7166_ship_mod_remote_armor_mutadaptive_repairer::mk_n_effect(),
        eff_d8037_chain_lightning::mk_n_effect(),
        eff_d11691_debuff_lance::mk_n_effect(),
        eff_d12126_micro_jump_portal_drive_capital::mk_n_effect(),
        eff_d12174_dot_missile_launching::mk_n_effect(),
        test::eff_d10000000_mod_proj_simple::mk_n_effect(),
        test::eff_d10000001_mod_proj_normal1::mk_n_effect(),
        test::eff_d10000002_mod_proj_normal2::mk_n_effect(),
    ]
}

fn get_n_effect_map() -> HashMap<ad::AEffectId, NEffect> {
    get_n_effects().into_iter().map(|v| (v.aid, v)).collect()
}
