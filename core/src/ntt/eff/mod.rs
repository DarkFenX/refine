use std::{collections::HashMap, sync::LazyLock};

pub(crate) use eff::NttEffect;
pub use eff::NttEffectRt;

use crate::ad;

mod eff;
mod eff_c1_char_missile_dmg;
mod eff_c2_aar_paste_boost;
mod eff_c3_stasis_web_probe;
mod eff_c4_missile_flight_time;
mod eff_d101_use_missiles;
mod eff_d11691_debuff_lance;
mod eff_d16_online;
mod eff_d1730_drone_dmg_bonus;
mod eff_d1851_self_rof;
mod eff_d3380_warp_disrupt_sphere;
mod eff_d3773_hardpoint_modifier_effect;
mod eff_d3774_slot_modifier;
mod eff_d4928_adaptive_armor_hardener;
mod eff_d4936_fueled_shield_boosting;
mod eff_d5275_fueled_armor_repair;
mod eff_d6222_struct_warp_scram_block_mwd_with_npc;
mod eff_d6426_remote_webifier_falloff;
mod eff_d6427_remote_sensor_boost_falloff;
mod eff_d6476_doomsday_aoe_web;
mod eff_d6485_ftr_abil_bomb;
mod eff_d660_missile_em_dmg_bonus;
mod eff_d661_missile_expl_dmg_bonus;
mod eff_d662_missile_therm_dmg_bonus;
mod eff_d6651_ship_module_raar;
mod eff_d6652_ship_module_rasb;
mod eff_d6682_struct_mod_effect_stasis_webifier;
mod eff_d668_missile_kin_dmg_bonus;
mod eff_d6690_remote_webifier_entity;
mod eff_d6730_mod_bonus_microwarpdrive;
mod eff_d6731_mod_bonus_afterburner;
mod eff_d6732_warfare_link_armor;
mod eff_d6733_warfare_link_shield;
mod eff_d6734_warfare_link_skirmish;
mod eff_d6735_warfare_link_info;
mod eff_d6736_warfare_link_mining;
mod eff_d6753_mod_titan_effect_generator;
mod eff_d6848_ship_mod_focused_warp_scram_script;
mod eff_d6849_ship_mod_focused_warp_disrupt_script;
mod eff_d7050_aoe_beacon_bioluminescence_cloud;
mod eff_d7051_aoe_beacon_caustic_cloud;
mod eff_d7053_aoe_beacon_pulse_01;
mod eff_d7058_aoe_beacon_filament_cloud;
mod eff_d7059_weather_caustic_toxin;
mod eff_d7060_weather_darkness;
mod eff_d7061_weather_electric_storm;
mod eff_d7062_weather_infernal;
mod eff_d7063_weather_xenon_gas;
mod shared;

pub(crate) static NTT_EFFECTS: LazyLock<Vec<NttEffect>> = LazyLock::new(get_ntt_effects);
pub(crate) static NTT_EFFECT_MAP: LazyLock<HashMap<ad::AEffectId, NttEffect>> = LazyLock::new(get_ntt_effect_map);

fn get_ntt_effects() -> Vec<NttEffect> {
    vec![
        eff_c1_char_missile_dmg::mk_ntt_effect(),
        eff_c2_aar_paste_boost::mk_ntt_effect(),
        eff_c3_stasis_web_probe::mk_ntt_effect(),
        eff_c4_missile_flight_time::mk_ntt_effect(),
        eff_d16_online::mk_ntt_effect(),
        eff_d101_use_missiles::mk_ntt_effect(),
        eff_d660_missile_em_dmg_bonus::mk_ntt_effect(),
        eff_d661_missile_expl_dmg_bonus::mk_ntt_effect(),
        eff_d662_missile_therm_dmg_bonus::mk_ntt_effect(),
        eff_d668_missile_kin_dmg_bonus::mk_ntt_effect(),
        eff_d1730_drone_dmg_bonus::mk_ntt_effect(),
        eff_d1851_self_rof::mk_ntt_effect(),
        eff_d3380_warp_disrupt_sphere::mk_ntt_effect(),
        eff_d3773_hardpoint_modifier_effect::mk_ntt_effect(),
        eff_d3774_slot_modifier::mk_ntt_effect(),
        eff_d4928_adaptive_armor_hardener::mk_ntt_effect(),
        eff_d4936_fueled_shield_boosting::mk_ntt_effect(),
        eff_d5275_fueled_armor_repair::mk_ntt_effect(),
        eff_d6222_struct_warp_scram_block_mwd_with_npc::mk_ntt_effect(),
        eff_d6426_remote_webifier_falloff::mk_ntt_effect(),
        eff_d6427_remote_sensor_boost_falloff::mk_ntt_effect(),
        eff_d6476_doomsday_aoe_web::mk_ntt_effect(),
        eff_d6485_ftr_abil_bomb::mk_ntt_effect(),
        eff_d6651_ship_module_raar::mk_ntt_effect(),
        eff_d6652_ship_module_rasb::mk_ntt_effect(),
        eff_d6682_struct_mod_effect_stasis_webifier::mk_ntt_effect(),
        eff_d6690_remote_webifier_entity::mk_ntt_effect(),
        eff_d6730_mod_bonus_microwarpdrive::mk_ntt_effect(),
        eff_d6731_mod_bonus_afterburner::mk_ntt_effect(),
        eff_d6732_warfare_link_armor::mk_ntt_effect(),
        eff_d6733_warfare_link_shield::mk_ntt_effect(),
        eff_d6734_warfare_link_skirmish::mk_ntt_effect(),
        eff_d6735_warfare_link_info::mk_ntt_effect(),
        eff_d6736_warfare_link_mining::mk_ntt_effect(),
        eff_d6753_mod_titan_effect_generator::mk_ntt_effect(),
        eff_d6848_ship_mod_focused_warp_scram_script::mk_ntt_effect(),
        eff_d6849_ship_mod_focused_warp_disrupt_script::mk_ntt_effect(),
        eff_d7050_aoe_beacon_bioluminescence_cloud::mk_ntt_effect(),
        eff_d7051_aoe_beacon_caustic_cloud::mk_ntt_effect(),
        eff_d7053_aoe_beacon_pulse_01::mk_ntt_effect(),
        eff_d7058_aoe_beacon_filament_cloud::mk_ntt_effect(),
        eff_d7059_weather_caustic_toxin::mk_ntt_effect(),
        eff_d7060_weather_darkness::mk_ntt_effect(),
        eff_d7061_weather_electric_storm::mk_ntt_effect(),
        eff_d7062_weather_infernal::mk_ntt_effect(),
        eff_d7063_weather_xenon_gas::mk_ntt_effect(),
        eff_d11691_debuff_lance::mk_ntt_effect(),
    ]
}

fn get_ntt_effect_map() -> HashMap<ad::AEffectId, NttEffect> {
    get_ntt_effects().into_iter().map(|v| (v.aid, v)).collect()
}
