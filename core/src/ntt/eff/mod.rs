pub(in crate::ntt) use def::NttEffect;

mod def;
mod eff_d11691_debuff_lance;
mod eff_d6476_doomsday_aoe_web;
mod eff_d6732_warfare_link_armor;
mod eff_d6733_warfare_link_shield;
mod eff_d6734_warfare_link_skirmish;
mod eff_d6735_warfare_link_info;
mod eff_d6736_warfare_link_mining;
mod eff_d6753_mod_titan_effect_generator;
mod eff_d7050_aoe_beacon_bioluminescence_cloud;
mod eff_d7051_aoe_beacon_caustic_cloud;
mod eff_d7053_aoe_beacon_pulse_01;
mod eff_d7058_aoe_beacon_filament_cloud;
mod eff_d7059_weather_caustic_toxin;
mod eff_d7060_weather_darkness;
mod eff_d7061_weather_electric_storm;
mod eff_d7062_weather_infernal;
mod eff_d7063_weather_xenon_gas;

fn get_ntt_effects() -> Vec<NttEffect> {
    vec![
        eff_d6476_doomsday_aoe_web::mk_ntt_effect(),
        eff_d6732_warfare_link_armor::mk_ntt_effect(),
        eff_d6733_warfare_link_shield::mk_ntt_effect(),
        eff_d6734_warfare_link_skirmish::mk_ntt_effect(),
        eff_d6735_warfare_link_info::mk_ntt_effect(),
        eff_d6736_warfare_link_mining::mk_ntt_effect(),
        eff_d6753_mod_titan_effect_generator::mk_ntt_effect(),
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
