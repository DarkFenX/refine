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

const ALL_EFFECTS: [NttEffect; 17] = [
    eff_d6476_doomsday_aoe_web::EFF_D6476,
    eff_d6732_warfare_link_armor::EFF_D6732,
    eff_d6733_warfare_link_shield::EFF_D6733,
    eff_d6734_warfare_link_skirmish::EFF_D6734,
    eff_d6735_warfare_link_info::EFF_D6735,
    eff_d6736_warfare_link_mining::EFF_D6736,
    eff_d6753_mod_titan_effect_generator::EFF_D6753,
    eff_d7050_aoe_beacon_bioluminescence_cloud::EFF_D7050,
    eff_d7051_aoe_beacon_caustic_cloud::EFF_D7051,
    eff_d7053_aoe_beacon_pulse_01::EFF_D7053,
    eff_d7058_aoe_beacon_filament_cloud::EFF_D7058,
    eff_d7059_weather_caustic_toxin::EFF_D7059,
    eff_d7060_weather_darkness::EFF_D7060,
    eff_d7061_weather_electric_storm::EFF_D7061,
    eff_d7062_weather_infernal::EFF_D7062,
    eff_d7063_weather_xenon_gas::EFF_D7063,
    eff_d11691_debuff_lance::EFF_D11691,
];
