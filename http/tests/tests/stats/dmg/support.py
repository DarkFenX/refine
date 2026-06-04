import typing
from dataclasses import dataclass

from fw.util import Absent, Default, conditional_insert

if typing.TYPE_CHECKING:
    from fw.client import TestClient


@dataclass(kw_only=True)
class DmgBasicInfo:
    # Attrs
    dmg_em_attr_id: int
    dmg_therm_attr_id: int
    dmg_kin_attr_id: int
    dmg_expl_attr_id: int
    dmg_mult_attr_id: int
    dmg_mult_spool_step_attr_id: int
    dmg_mult_spool_max_attr_id: int
    dmg_breach_abs_attr_id: int
    dmg_breach_rel_attr_id: int
    dmg_breach_duration_attr_id: int
    dd_delay1_attr_id: int
    dd_delay2_attr_id: int
    dd_dmg_interval_attr_id: int
    dd_dmg_duration_attr_id: int
    capacity_attr_id: int
    volume_attr_id: int
    charge_rate_attr_id: int
    cycle_time_attr_id: int
    reactivation_delay_attr_id: int
    reload_time_attr_id: int
    crystal_get_dmg_attr_id: int
    crystal_volatility_chance_attr_id: int
    crystal_volatility_dmg_attr_id: int
    ammo_loaded_attr_id: int
    sig_radius_attr_id: int
    prop_sig_radius_mult_attr_id: int
    radius_attr_id: int
    max_range_attr_id: int
    falloff_attr_id: int
    tracking_attr_id: int
    sig_resolution_attr_id: int
    emp_field_range_attr_id: int
    entity_cruise_speed_attr_id: int
    max_velocity_attr_id: int
    flight_time_attr_id: int
    mass_attr_id: int
    agility_attr_id: int
    max_fof_range_attr_id: int
    aoe_cloud_size_attr_id: int
    aoe_velocity_attr_id: int
    aoe_drf_attr_id: int
    shield_hp_attr_id: int
    armor_hp_attr_id: int
    hull_hp_attr_id: int
    resist_ref_attr_id: int
    neut_resist_attr_id: int
    max_ftr_count_attr_id: int
    refuel_duration_attr_id: int
    # Effects
    turret_proj_effect_id: int
    turret_spool_effect_id: int
    tgt_attack_effect_id: int
    vorton_effect_id: int
    launcher_effect_id: int
    missile_effect_id: int
    missile_fof_effect_id: int
    missile_defender_effect_id: int
    breacher_effect_id: int
    bomb_effect_id: int
    smartbomb_effect_id: int
    pds_effect_id: int
    dd_direct_amarr_effect_id: int
    dd_direct_caldari_effect_id: int
    dd_direct_gallente_effect_id: int
    dd_direct_minmatar_effect_id: int
    dd_lance_effect_id: int
    dd_lance_debuff_effect_id: int
    dd_reaper_effect_id: int
    dd_boson_effect_id: int
    dd_vorton_effect_id: int
    # Fighter "primary" missile attack
    ftr_abil_atkm_dmg_em_attr_id: int
    ftr_abil_atkm_dmg_therm_attr_id: int
    ftr_abil_atkm_dmg_kin_attr_id: int
    ftr_abil_atkm_dmg_expl_attr_id: int
    ftr_abil_atkm_dmg_mult_attr_id: int
    ftr_abil_atkm_cycle_time_attr_id: int
    ftr_abil_atkm_range_optimal_attr_id: int
    ftr_abil_atkm_range_falloff_attr_id: int
    ftr_abil_atkm_exp_radius_attr_id: int
    ftr_abil_atkm_exp_speed_attr_id: int
    ftr_abil_atkm_dr_factor_attr_id: int
    ftr_abil_atkm_dr_sens_attr_id: int
    ftr_abil_atkm_effect_id: int
    atkm_abil_id: int
    # Fighter "secondary" missile attack
    ftr_abil_missiles_dmg_em_attr_id: int
    ftr_abil_missiles_dmg_therm_attr_id: int
    ftr_abil_missiles_dmg_kin_attr_id: int
    ftr_abil_missiles_dmg_expl_attr_id: int
    ftr_abil_missiles_dmg_mult_attr_id: int
    ftr_abil_missiles_cycle_time_attr_id: int
    ftr_abil_missiles_range_attr_id: int
    ftr_abil_missiles_exp_radius_attr_id: int
    ftr_abil_missiles_exp_speed_attr_id: int
    ftr_abil_missiles_dr_factor_attr_id: int
    ftr_abil_missiles_dr_sens_attr_id: int
    ftr_abil_missiles_resist_ref_attr_id: int
    ftr_abil_missiles_effect_id: int
    missiles_abil_id: int
    # Fighter bomb attack
    ftr_abil_bomb_type_attr_id: int
    ftr_abil_bomb_cycle_time_attr_id: int
    ftr_abil_bomb_effect_id: int
    bomb_abil_id: int
    # Fighter kamikaze attack
    ftr_abil_kamikaze_dmg_em_attr_id: int
    ftr_abil_kamikaze_dmg_therm_attr_id: int
    ftr_abil_kamikaze_dmg_kin_attr_id: int
    ftr_abil_kamikaze_dmg_expl_attr_id: int
    ftr_abil_kamikaze_cycle_time_attr_id: int
    ftr_abil_kamikaze_range_attr_id: int
    ftr_abil_kamikaze_sig_radius_attr_id: int
    ftr_abil_kamikaze_resist_ref_attr_id: int
    ftr_abil_kamikaze_effect_id: int
    kamikaze_abil_id: int
    # Misc
    guided_bomb_group_id: int


def setup_dmg_basics(
        *,
        client: TestClient,
        consts,  # noqa: ANN001
        effect_duration: bool = True,
        effect_range: bool = True,
        effect_falloff: bool = True,
        effect_tracking: bool = True,
) -> DmgBasicInfo:
    # Attrs
    dmg_em_attr_id = client.mk_eve_attr(id_=consts.EveAttr.em_dmg)
    dmg_therm_attr_id = client.mk_eve_attr(id_=consts.EveAttr.therm_dmg)
    dmg_kin_attr_id = client.mk_eve_attr(id_=consts.EveAttr.kin_dmg)
    dmg_expl_attr_id = client.mk_eve_attr(id_=consts.EveAttr.expl_dmg)
    dmg_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.dmg_mult)
    dmg_mult_spool_step_attr_id = client.mk_eve_attr(id_=consts.EveAttr.dmg_mult_bonus_per_cycle)
    dmg_mult_spool_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.dmg_mult_bonus_max)
    dmg_breach_abs_attr_id = client.mk_eve_attr(id_=consts.EveAttr.dot_max_dmg_per_tick)
    dmg_breach_rel_attr_id = client.mk_eve_attr(id_=consts.EveAttr.dot_max_hp_perc_per_tick)
    dmg_breach_duration_attr_id = client.mk_eve_attr(id_=consts.EveAttr.dot_duration)
    dd_delay1_attr_id = client.mk_eve_attr(id_=consts.EveAttr.dmg_delay_duration)
    dd_delay2_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_warning_duration)
    dd_dmg_interval_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_dmg_cycle_time)
    dd_dmg_duration_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_dmg_duration)
    capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    cycle_time_attr_id = client.mk_eve_attr()
    reactivation_delay_attr_id = client.mk_eve_attr(id_=consts.EveAttr.module_reactivation_delay)
    reload_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.reload_time)
    crystal_get_dmg_attr_id = client.mk_eve_attr(id_=consts.EveAttr.crystals_get_damaged)
    crystal_volatility_chance_attr_id = client.mk_eve_attr(id_=consts.EveAttr.crystal_volatility_chance)
    crystal_volatility_dmg_attr_id = client.mk_eve_attr(id_=consts.EveAttr.crystal_volatility_damage)
    ammo_loaded_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ammo_loaded)
    sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    prop_sig_radius_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.entity_max_velocity_sig_radius_mult)
    radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    max_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_range)
    falloff_attr_id = client.mk_eve_attr(id_=consts.EveAttr.falloff)
    tracking_attr_id = client.mk_eve_attr(id_=consts.EveAttr.tracking_speed)
    sig_resolution_attr_id = client.mk_eve_attr(id_=consts.EveAttr.optimal_sig_radius)
    emp_field_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.emp_field_range)
    entity_cruise_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.entity_cruise_speed)
    max_velocity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    flight_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.explosion_delay)
    mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    agility_attr_id = client.mk_eve_attr(id_=consts.EveAttr.agility)
    max_fof_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_fof_target_range)
    aoe_cloud_size_attr_id = client.mk_eve_attr(id_=consts.EveAttr.aoe_cloud_size)
    aoe_velocity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.aoe_velocity)
    aoe_drf_attr_id = client.mk_eve_attr(id_=consts.EveAttr.aoe_damage_reduction_factor)
    shield_hp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_capacity)
    armor_hp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_hp)
    hull_hp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hp)
    resist_def_attr_id = client.mk_eve_attr(id_=consts.EveAttr.remote_resistance_id)
    neut_resist_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_warfare_resist)
    max_ftr_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    refuel_duration_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_refueling_time)
    # Effects
    turret_proj_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.projectile_fired,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=cycle_time_attr_id if effect_duration else Default,
        range_attr_id=max_range_attr_id if effect_range else Default,
        falloff_attr_id=falloff_attr_id if effect_falloff else Default,
        tracking_attr_id=tracking_attr_id if effect_tracking else Default)
    turret_spool_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.tgt_disintegrator_attack,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=cycle_time_attr_id if effect_duration else Default,
        range_attr_id=max_range_attr_id if effect_range else Default,
        falloff_attr_id=falloff_attr_id if effect_falloff else Default,
        tracking_attr_id=tracking_attr_id if effect_tracking else Default)
    tgt_attack_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.tgt_attack,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=cycle_time_attr_id if effect_duration else Default,
        range_attr_id=max_range_attr_id if effect_range else Default,
        falloff_attr_id=falloff_attr_id if effect_falloff else Default,
        tracking_attr_id=tracking_attr_id if effect_tracking else Default)
    vorton_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.chain_lightning,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=cycle_time_attr_id if effect_duration else Default,
        range_attr_id=max_range_attr_id if effect_range else Default)
    launcher_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.use_missiles,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=cycle_time_attr_id if effect_duration else Default)
    missile_effect_id = client.mk_eve_effect(id_=consts.EveEffect.missile_launching, cat_id=consts.EveEffCat.target)
    missile_fof_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.fof_missile_launching,
        cat_id=consts.EveEffCat.active)
    missile_defender_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.defender_missile_launching,
        cat_id=consts.EveEffCat.active)
    breacher_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.dot_missile_launching,
        cat_id=consts.EveEffCat.target)
    bomb_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.bomb_launching,
        cat_id=consts.EveEffCat.active)
    smartbomb_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.emp_wave,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=cycle_time_attr_id if effect_duration else Default,
        range_attr_id=emp_field_range_attr_id if effect_range else Default)
    pds_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.point_defense,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=cycle_time_attr_id if effect_duration else Default,
        range_attr_id=emp_field_range_attr_id if effect_range else Default)
    dd_direct_amarr_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.super_weapon_amarr,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=cycle_time_attr_id if effect_duration else Default)
    dd_direct_caldari_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.super_weapon_caldari,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=cycle_time_attr_id if effect_duration else Default)
    dd_direct_gallente_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.super_weapon_gallente,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=cycle_time_attr_id if effect_duration else Default)
    dd_direct_minmatar_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.super_weapon_minmatar,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=cycle_time_attr_id if effect_duration else Default)
    dd_lance_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.doomsday_beam_dot,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=cycle_time_attr_id if effect_duration else Default)
    dd_lance_debuff_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.debuff_lance,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=cycle_time_attr_id if effect_duration else Default)
    dd_reaper_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.doomsday_slash,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=cycle_time_attr_id if effect_duration else Default)
    dd_boson_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.doomsday_cone_dot,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=cycle_time_attr_id if effect_duration else Default)
    dd_vorton_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.lightning_weapon,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=cycle_time_attr_id if effect_duration else Default)
    # Fighter "primary" missile attack
    ftr_abil_atkm_dmg_em_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_atkm_dmg_em)
    ftr_abil_atkm_dmg_therm_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_atkm_dmg_therm)
    ftr_abil_atkm_dmg_kin_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_atkm_dmg_kin)
    ftr_abil_atkm_dmg_expl_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_atkm_dmg_expl)
    ftr_abil_atkm_dmg_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_atkm_dmg_mult)
    ftr_abil_atkm_cycle_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_atkm_duration)
    ftr_abil_atkm_range_optimal_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_atkm_range_optimal)
    ftr_abil_atkm_range_falloff_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_atkm_range_falloff)
    ftr_abil_atkm_exp_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_atkm_expl_radius)
    ftr_abil_atkm_exp_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_atkm_expl_velocity)
    ftr_abil_atkm_dr_factor_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_atkm_reduction_factor)
    ftr_abil_atkm_dr_sens_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_atkm_reduction_sens)
    ftr_abil_atkm_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_attack_m,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=ftr_abil_atkm_cycle_time_attr_id if effect_duration else Default,
        range_attr_id=ftr_abil_atkm_range_optimal_attr_id if effect_range else Default,
        falloff_attr_id=ftr_abil_atkm_range_falloff_attr_id if effect_falloff else Default)
    atkm_abil_id = client.mk_eve_abil(id_=consts.EveAbil.pulse_cannon)
    # Fighter "secondary" missile attack
    ftr_abil_missiles_dmg_em_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_missiles_dmg_em)
    ftr_abil_missiles_dmg_therm_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_missiles_dmg_therm)
    ftr_abil_missiles_dmg_kin_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_missiles_dmg_kin)
    ftr_abil_missiles_dmg_expl_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_missiles_dmg_expl)
    ftr_abil_missiles_dmg_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_missiles_dmg_mult)
    ftr_abil_missiles_cycle_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_missiles_duration)
    ftr_abil_missiles_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_missiles_range)
    ftr_abil_missiles_exp_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_missiles_expl_radius)
    ftr_abil_missiles_exp_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_missiles_expl_velocity)
    ftr_abil_missiles_dr_factor_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.ftr_abil_missiles_dmg_reduction_factor)
    ftr_abil_missiles_dr_sens_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_missiles_dmg_reduction_sens)
    ftr_abil_missiles_resist_ref_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_missiles_resist_id)
    ftr_abil_missiles_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_missiles,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=ftr_abil_missiles_cycle_time_attr_id if effect_duration else Default,
        range_attr_id=ftr_abil_missiles_range_attr_id if effect_range else Default)
    missiles_abil_id = client.mk_eve_abil(id_=consts.EveAbil.heavy_rocket_salvo)
    # Fighter bomb attack
    ftr_abil_bomb_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    ftr_abil_bomb_cycle_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_duration)
    ftr_abil_bomb_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_launch_bomb,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=ftr_abil_bomb_cycle_time_attr_id if effect_duration else Default)
    bomb_abil_id = client.mk_eve_abil(id_=consts.EveAbil.launch_bomb)
    # Fighter kamikaze attack
    ftr_abil_kamikaze_dmg_em_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_kamikaze_dmg_em)
    ftr_abil_kamikaze_dmg_therm_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_kamikaze_dmg_therm)
    ftr_abil_kamikaze_dmg_kin_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_kamikaze_dmg_kin)
    ftr_abil_kamikaze_dmg_expl_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_kamikaze_dmg_exp)
    ftr_abil_kamikaze_cycle_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_kamikaze_duration)
    ftr_abil_kamikaze_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_kamikaze_range)
    ftr_abil_kamikaze_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_kamikaze_sig_radius)
    ftr_abil_kamikaze_resist_ref_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_kamikaze_resist_id)
    ftr_abil_kamikaze_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_kamikaze,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=ftr_abil_kamikaze_cycle_time_attr_id if effect_duration else Default)
    kamikaze_abil_id = client.mk_eve_abil(id_=consts.EveAbil.true_sacrifice)
    # Ensure effects and abilities are not cleaned up even if not all of them are used in a test
    client.mk_eve_fighter(
        eff_ids=[
            turret_proj_effect_id,
            turret_spool_effect_id,
            tgt_attack_effect_id,
            vorton_effect_id,
            launcher_effect_id,
            missile_effect_id,
            missile_fof_effect_id,
            missile_defender_effect_id,
            breacher_effect_id,
            bomb_effect_id,
            smartbomb_effect_id,
            pds_effect_id,
            dd_direct_amarr_effect_id,
            dd_direct_caldari_effect_id,
            dd_direct_gallente_effect_id,
            dd_direct_minmatar_effect_id,
            dd_lance_effect_id,
            dd_lance_debuff_effect_id,
            dd_reaper_effect_id,
            dd_boson_effect_id,
            dd_vorton_effect_id,
            ftr_abil_atkm_effect_id,
            ftr_abil_missiles_effect_id,
            ftr_abil_bomb_effect_id,
            ftr_abil_kamikaze_effect_id],
        abils=[
            client.mk_eve_item_abil(id_=atkm_abil_id),
            client.mk_eve_item_abil(id_=missiles_abil_id),
            client.mk_eve_item_abil(id_=bomb_abil_id),
            client.mk_eve_item_abil(id_=kamikaze_abil_id)])
    return DmgBasicInfo(
        # Attrs
        dmg_em_attr_id=dmg_em_attr_id,
        dmg_therm_attr_id=dmg_therm_attr_id,
        dmg_kin_attr_id=dmg_kin_attr_id,
        dmg_expl_attr_id=dmg_expl_attr_id,
        dmg_mult_attr_id=dmg_mult_attr_id,
        dmg_mult_spool_step_attr_id=dmg_mult_spool_step_attr_id,
        dmg_mult_spool_max_attr_id=dmg_mult_spool_max_attr_id,
        dmg_breach_abs_attr_id=dmg_breach_abs_attr_id,
        dmg_breach_rel_attr_id=dmg_breach_rel_attr_id,
        dmg_breach_duration_attr_id=dmg_breach_duration_attr_id,
        dd_delay1_attr_id=dd_delay1_attr_id,
        dd_delay2_attr_id=dd_delay2_attr_id,
        dd_dmg_interval_attr_id=dd_dmg_interval_attr_id,
        dd_dmg_duration_attr_id=dd_dmg_duration_attr_id,
        cycle_time_attr_id=cycle_time_attr_id,
        reactivation_delay_attr_id=reactivation_delay_attr_id,
        volume_attr_id=volume_attr_id,
        capacity_attr_id=capacity_attr_id,
        charge_rate_attr_id=charge_rate_attr_id,
        reload_time_attr_id=reload_time_attr_id,
        crystal_get_dmg_attr_id=crystal_get_dmg_attr_id,
        crystal_volatility_chance_attr_id=crystal_volatility_chance_attr_id,
        crystal_volatility_dmg_attr_id=crystal_volatility_dmg_attr_id,
        ammo_loaded_attr_id=ammo_loaded_attr_id,
        sig_radius_attr_id=sig_radius_attr_id,
        prop_sig_radius_mult_attr_id=prop_sig_radius_mult_attr_id,
        radius_attr_id=radius_attr_id,
        max_range_attr_id=max_range_attr_id,
        falloff_attr_id=falloff_attr_id,
        tracking_attr_id=tracking_attr_id,
        sig_resolution_attr_id=sig_resolution_attr_id,
        emp_field_range_attr_id=emp_field_range_attr_id,
        entity_cruise_speed_attr_id=entity_cruise_speed_attr_id,
        max_velocity_attr_id=max_velocity_attr_id,
        flight_time_attr_id=flight_time_attr_id,
        mass_attr_id=mass_attr_id,
        agility_attr_id=agility_attr_id,
        max_fof_range_attr_id=max_fof_range_attr_id,
        aoe_cloud_size_attr_id=aoe_cloud_size_attr_id,
        aoe_velocity_attr_id=aoe_velocity_attr_id,
        aoe_drf_attr_id=aoe_drf_attr_id,
        shield_hp_attr_id=shield_hp_attr_id,
        armor_hp_attr_id=armor_hp_attr_id,
        hull_hp_attr_id=hull_hp_attr_id,
        resist_ref_attr_id=resist_def_attr_id,
        neut_resist_attr_id=neut_resist_attr_id,
        max_ftr_count_attr_id=max_ftr_count_attr_id,
        refuel_duration_attr_id=refuel_duration_attr_id,
        # Effects
        turret_proj_effect_id=turret_proj_effect_id,
        turret_spool_effect_id=turret_spool_effect_id,
        tgt_attack_effect_id=tgt_attack_effect_id,
        vorton_effect_id=vorton_effect_id,
        launcher_effect_id=launcher_effect_id,
        missile_effect_id=missile_effect_id,
        missile_fof_effect_id=missile_fof_effect_id,
        missile_defender_effect_id=missile_defender_effect_id,
        breacher_effect_id=breacher_effect_id,
        bomb_effect_id=bomb_effect_id,
        smartbomb_effect_id=smartbomb_effect_id,
        pds_effect_id=pds_effect_id,
        dd_direct_amarr_effect_id=dd_direct_amarr_effect_id,
        dd_direct_caldari_effect_id=dd_direct_caldari_effect_id,
        dd_direct_gallente_effect_id=dd_direct_gallente_effect_id,
        dd_direct_minmatar_effect_id=dd_direct_minmatar_effect_id,
        dd_lance_effect_id=dd_lance_effect_id,
        dd_lance_debuff_effect_id=dd_lance_debuff_effect_id,
        dd_reaper_effect_id=dd_reaper_effect_id,
        dd_boson_effect_id=dd_boson_effect_id,
        dd_vorton_effect_id=dd_vorton_effect_id,
        # Fighter "primary" missile attack
        ftr_abil_atkm_dmg_em_attr_id=ftr_abil_atkm_dmg_em_attr_id,
        ftr_abil_atkm_dmg_therm_attr_id=ftr_abil_atkm_dmg_therm_attr_id,
        ftr_abil_atkm_dmg_kin_attr_id=ftr_abil_atkm_dmg_kin_attr_id,
        ftr_abil_atkm_dmg_expl_attr_id=ftr_abil_atkm_dmg_expl_attr_id,
        ftr_abil_atkm_dmg_mult_attr_id=ftr_abil_atkm_dmg_mult_attr_id,
        ftr_abil_atkm_cycle_time_attr_id=ftr_abil_atkm_cycle_time_attr_id,
        ftr_abil_atkm_range_optimal_attr_id=ftr_abil_atkm_range_optimal_attr_id,
        ftr_abil_atkm_range_falloff_attr_id=ftr_abil_atkm_range_falloff_attr_id,
        ftr_abil_atkm_exp_radius_attr_id=ftr_abil_atkm_exp_radius_attr_id,
        ftr_abil_atkm_exp_speed_attr_id=ftr_abil_atkm_exp_speed_attr_id,
        ftr_abil_atkm_dr_factor_attr_id=ftr_abil_atkm_dr_factor_attr_id,
        ftr_abil_atkm_dr_sens_attr_id=ftr_abil_atkm_dr_sens_attr_id,
        ftr_abil_atkm_effect_id=ftr_abil_atkm_effect_id,
        atkm_abil_id=atkm_abil_id,
        # Fighter "secondary" missile attack
        ftr_abil_missiles_dmg_em_attr_id=ftr_abil_missiles_dmg_em_attr_id,
        ftr_abil_missiles_dmg_therm_attr_id=ftr_abil_missiles_dmg_therm_attr_id,
        ftr_abil_missiles_dmg_kin_attr_id=ftr_abil_missiles_dmg_kin_attr_id,
        ftr_abil_missiles_dmg_expl_attr_id=ftr_abil_missiles_dmg_expl_attr_id,
        ftr_abil_missiles_dmg_mult_attr_id=ftr_abil_missiles_dmg_mult_attr_id,
        ftr_abil_missiles_cycle_time_attr_id=ftr_abil_missiles_cycle_time_attr_id,
        ftr_abil_missiles_range_attr_id=ftr_abil_missiles_range_attr_id,
        ftr_abil_missiles_exp_radius_attr_id=ftr_abil_missiles_exp_radius_attr_id,
        ftr_abil_missiles_exp_speed_attr_id=ftr_abil_missiles_exp_speed_attr_id,
        ftr_abil_missiles_dr_factor_attr_id=ftr_abil_missiles_dr_factor_attr_id,
        ftr_abil_missiles_dr_sens_attr_id=ftr_abil_missiles_dr_sens_attr_id,
        ftr_abil_missiles_resist_ref_attr_id=ftr_abil_missiles_resist_ref_attr_id,
        ftr_abil_missiles_effect_id=ftr_abil_missiles_effect_id,
        missiles_abil_id=missiles_abil_id,
        # Fighter bomb attack
        ftr_abil_bomb_type_attr_id=ftr_abil_bomb_type_attr_id,
        ftr_abil_bomb_cycle_time_attr_id=ftr_abil_bomb_cycle_time_attr_id,
        ftr_abil_bomb_effect_id=ftr_abil_bomb_effect_id,
        bomb_abil_id=bomb_abil_id,
        # Fighter kamikaze attack
        ftr_abil_kamikaze_dmg_em_attr_id=ftr_abil_kamikaze_dmg_em_attr_id,
        ftr_abil_kamikaze_dmg_therm_attr_id=ftr_abil_kamikaze_dmg_therm_attr_id,
        ftr_abil_kamikaze_dmg_kin_attr_id=ftr_abil_kamikaze_dmg_kin_attr_id,
        ftr_abil_kamikaze_dmg_expl_attr_id=ftr_abil_kamikaze_dmg_expl_attr_id,
        ftr_abil_kamikaze_cycle_time_attr_id=ftr_abil_kamikaze_cycle_time_attr_id,
        ftr_abil_kamikaze_range_attr_id=ftr_abil_kamikaze_range_attr_id,
        ftr_abil_kamikaze_sig_radius_attr_id=ftr_abil_kamikaze_sig_radius_attr_id,
        ftr_abil_kamikaze_resist_ref_attr_id=ftr_abil_kamikaze_resist_ref_attr_id,
        ftr_abil_kamikaze_effect_id=ftr_abil_kamikaze_effect_id,
        kamikaze_abil_id=kamikaze_abil_id,
        # Misc
        guided_bomb_group_id=consts.EveItemGrp.guided_bomb)


def make_eve_ship(
        *,
        client: TestClient,
        basic_info: DmgBasicInfo,
        hps: tuple[float | None, float | None, float | None] | None = None,
        speed: float | type[Absent] = Absent,
        sig_radius: float | type[Absent] = Absent,
        neut_resist: float | type[Absent] = Absent,
        radius: float | type[Absent] = Absent,
        extra_attrs: dict[int, float] | None = None,
) -> int:
    attrs = {}
    if hps is not None:
        hp_attr_ids = (
            basic_info.shield_hp_attr_id,
            basic_info.armor_hp_attr_id,
            basic_info.hull_hp_attr_id)
        attrs.update({k: v for k, v in zip(hp_attr_ids, hps, strict=True) if v is not None})
    conditional_insert(container=attrs, path=[basic_info.max_velocity_attr_id], value=speed)
    conditional_insert(container=attrs, path=[basic_info.sig_radius_attr_id], value=sig_radius)
    conditional_insert(container=attrs, path=[basic_info.neut_resist_attr_id], value=neut_resist)
    conditional_insert(container=attrs, path=[basic_info.radius_attr_id], value=radius)
    if extra_attrs:
        attrs.update(extra_attrs)
    return client.mk_eve_ship(attrs=attrs)


def make_eve_turret_proj(
        *,
        client: TestClient,
        basic_info: DmgBasicInfo,
        dmg_mult: float | type[Absent] = Absent,
        cycle_time: float | type[Absent] = Absent,
        capacity: float | type[Absent] = Absent,
        reload_time: float | type[Absent] = Absent,
        range_optimal: float | type[Absent] = Absent,
        range_falloff: float | type[Absent] = Absent,
        tracking: float | type[Absent] = Absent,
        sig_resolution: float | type[Absent] = Absent,
) -> int:
    attrs = {basic_info.charge_rate_attr_id: 1.0}
    conditional_insert(container=attrs, path=[basic_info.dmg_mult_attr_id], value=dmg_mult)
    conditional_insert(container=attrs, path=[basic_info.cycle_time_attr_id], value=cycle_time)
    conditional_insert(container=attrs, path=[basic_info.capacity_attr_id], value=capacity)
    conditional_insert(container=attrs, path=[basic_info.reload_time_attr_id], value=reload_time)
    conditional_insert(container=attrs, path=[basic_info.max_range_attr_id], value=range_optimal)
    conditional_insert(container=attrs, path=[basic_info.falloff_attr_id], value=range_falloff)
    conditional_insert(container=attrs, path=[basic_info.tracking_attr_id], value=tracking)
    conditional_insert(container=attrs, path=[basic_info.sig_resolution_attr_id], value=sig_resolution)
    return client.mk_eve_item(
        attrs=attrs,
        eff_ids=[basic_info.turret_proj_effect_id],
        defeff_id=basic_info.turret_proj_effect_id)


def make_eve_turret_laser(
        *,
        client: TestClient,
        basic_info: DmgBasicInfo,
        dmg_mult: float | type[Absent] = Absent,
        cycle_time: float | type[Absent] = Absent,
        capacity: float | type[Absent] = Absent,
        reload_time: float | type[Absent] = Absent,
        range_optimal: float | type[Absent] = Absent,
        range_falloff: float | type[Absent] = Absent,
        tracking: float | type[Absent] = Absent,
        sig_resolution: float | type[Absent] = Absent,
) -> int:
    attrs = {}
    conditional_insert(container=attrs, path=[basic_info.dmg_mult_attr_id], value=dmg_mult)
    conditional_insert(container=attrs, path=[basic_info.cycle_time_attr_id], value=cycle_time)
    conditional_insert(container=attrs, path=[basic_info.capacity_attr_id], value=capacity)
    conditional_insert(container=attrs, path=[basic_info.reload_time_attr_id], value=reload_time)
    conditional_insert(container=attrs, path=[basic_info.max_range_attr_id], value=range_optimal)
    conditional_insert(container=attrs, path=[basic_info.falloff_attr_id], value=range_falloff)
    conditional_insert(container=attrs, path=[basic_info.tracking_attr_id], value=tracking)
    conditional_insert(container=attrs, path=[basic_info.sig_resolution_attr_id], value=sig_resolution)
    return client.mk_eve_item(
        attrs=attrs,
        eff_ids=[basic_info.tgt_attack_effect_id],
        defeff_id=basic_info.tgt_attack_effect_id)


def make_eve_turret_spool(
        *,
        client: TestClient,
        basic_info: DmgBasicInfo,
        dmg_mult: float | type[Absent] = Absent,
        spool_step: float | type[Absent] = Absent,
        spool_max: float | type[Absent] = Absent,
        cycle_time: float | type[Absent] = Absent,
        capacity: float | type[Absent] = Absent,
        reload_time: float | type[Absent] = Absent,
        range_optimal: float | type[Absent] = Absent,
        tracking: float | type[Absent] = Absent,
        sig_resolution: float | type[Absent] = Absent,
) -> int:
    attrs = {}
    conditional_insert(container=attrs, path=[basic_info.dmg_mult_attr_id], value=dmg_mult)
    conditional_insert(container=attrs, path=[basic_info.dmg_mult_spool_step_attr_id], value=spool_step)
    conditional_insert(container=attrs, path=[basic_info.dmg_mult_spool_max_attr_id], value=spool_max)
    conditional_insert(container=attrs, path=[basic_info.cycle_time_attr_id], value=cycle_time)
    conditional_insert(container=attrs, path=[basic_info.capacity_attr_id], value=capacity)
    conditional_insert(container=attrs, path=[basic_info.reload_time_attr_id], value=reload_time)
    conditional_insert(container=attrs, path=[basic_info.max_range_attr_id], value=range_optimal)
    conditional_insert(container=attrs, path=[basic_info.tracking_attr_id], value=tracking)
    conditional_insert(container=attrs, path=[basic_info.sig_resolution_attr_id], value=sig_resolution)
    return client.mk_eve_item(
        attrs=attrs,
        eff_ids=[basic_info.turret_spool_effect_id],
        defeff_id=basic_info.turret_spool_effect_id)


def make_eve_turret_civilian(
        *,
        client: TestClient,
        basic_info: DmgBasicInfo,
        dmgs: tuple[float | None, float | None, float | None, float | None] | None = None,
        dmg_mult: float | type[Absent] = Absent,
        cycle_time: float | type[Absent] = Absent,
        reload_time: float | type[Absent] = Absent,
        charge_type_id: float | type[Absent] = Absent,
) -> int:
    # The lib doesn't make use of module-level autocharges, but we fill the data needed for it for
    # completeness anyway
    attrs = {}
    _add_dmgs(basic_info=basic_info, attrs=attrs, dmgs=dmgs)
    conditional_insert(container=attrs, path=[basic_info.dmg_mult_attr_id], value=dmg_mult)
    conditional_insert(container=attrs, path=[basic_info.cycle_time_attr_id], value=cycle_time)
    conditional_insert(container=attrs, path=[basic_info.reload_time_attr_id], value=reload_time)
    conditional_insert(container=attrs, path=[basic_info.ammo_loaded_attr_id], value=charge_type_id)
    return client.mk_eve_item(
        attrs=attrs,
        eff_ids=[basic_info.tgt_attack_effect_id],
        defeff_id=basic_info.tgt_attack_effect_id)


def make_eve_vorton(
        *,
        client: TestClient,
        basic_info: DmgBasicInfo,
        dmg_mult: float | type[Absent] = Absent,
        cycle_time: float | type[Absent] = Absent,
        capacity: float | type[Absent] = Absent,
        reload_time: float | type[Absent] = Absent,
        range_optimal: float | type[Absent] = Absent,
        exp_radius: float | type[Absent] = Absent,
        exp_speed: float | type[Absent] = Absent,
        drf: float | type[Absent] = Absent,
) -> int:
    attrs = {basic_info.charge_rate_attr_id: 1.0}
    conditional_insert(container=attrs, path=[basic_info.dmg_mult_attr_id], value=dmg_mult)
    conditional_insert(container=attrs, path=[basic_info.cycle_time_attr_id], value=cycle_time)
    conditional_insert(container=attrs, path=[basic_info.capacity_attr_id], value=capacity)
    conditional_insert(container=attrs, path=[basic_info.reload_time_attr_id], value=reload_time)
    conditional_insert(container=attrs, path=[basic_info.max_range_attr_id], value=range_optimal)
    conditional_insert(container=attrs, path=[basic_info.aoe_cloud_size_attr_id], value=exp_radius)
    conditional_insert(container=attrs, path=[basic_info.aoe_velocity_attr_id], value=exp_speed)
    conditional_insert(container=attrs, path=[basic_info.aoe_drf_attr_id], value=drf)
    return client.mk_eve_item(
        attrs=attrs,
        eff_ids=[basic_info.vorton_effect_id],
        defeff_id=basic_info.vorton_effect_id)


def make_eve_charge_normal(
        *,
        client: TestClient,
        basic_info: DmgBasicInfo,
        dmgs: tuple[float | None, float | None, float | None, float | None] | None = None,
        volume: float | type[Absent] = Absent,
) -> int:
    attrs = {}
    _add_dmgs(basic_info=basic_info, attrs=attrs, dmgs=dmgs)
    conditional_insert(container=attrs, path=[basic_info.volume_attr_id], value=volume)
    return client.mk_eve_item(attrs=attrs)


def make_eve_charge_crystal(
        *,
        client: TestClient,
        basic_info: DmgBasicInfo,
        dmgs: tuple[float | None, float | None, float | None, float | None] | None = None,
        volume: float | type[Absent] = Absent,
        get_damaged: float | type[Absent] = Absent,
        hp: float | type[Absent] = Absent,
        vol_dmg: float | type[Absent] = Absent,
        vol_chance: float | type[Absent] = Absent,
) -> int:
    attrs = {}
    _add_dmgs(basic_info=basic_info, attrs=attrs, dmgs=dmgs)
    conditional_insert(container=attrs, path=[basic_info.volume_attr_id], value=volume)
    conditional_insert(container=attrs, path=[basic_info.crystal_get_dmg_attr_id], value=get_damaged)
    conditional_insert(container=attrs, path=[basic_info.hull_hp_attr_id], value=hp)
    conditional_insert(container=attrs, path=[basic_info.crystal_volatility_dmg_attr_id], value=vol_dmg)
    conditional_insert(container=attrs, path=[basic_info.crystal_volatility_chance_attr_id], value=vol_chance)
    return client.mk_eve_item(attrs=attrs)


def make_eve_launcher(
        *,
        client: TestClient,
        basic_info: DmgBasicInfo,
        cycle_time: float | type[Absent] = Absent,
        capacity: float | type[Absent] = Absent,
        reactivation_delay: float | type[Absent] = Absent,
        reload_time: float | type[Absent] = Absent,
) -> int:
    attrs = {basic_info.charge_rate_attr_id: 1.0}
    conditional_insert(container=attrs, path=[basic_info.cycle_time_attr_id], value=cycle_time)
    conditional_insert(container=attrs, path=[basic_info.capacity_attr_id], value=capacity)
    conditional_insert(container=attrs, path=[basic_info.reactivation_delay_attr_id], value=reactivation_delay)
    conditional_insert(container=attrs, path=[basic_info.reload_time_attr_id], value=reload_time)
    return client.mk_eve_item(
        attrs=attrs,
        eff_ids=[basic_info.launcher_effect_id],
        defeff_id=basic_info.launcher_effect_id)


def make_eve_missile(
        *,
        client: TestClient,
        basic_info: DmgBasicInfo,
        dmgs: tuple[float | None, float | None, float | None, float | None] | None = None,
        volume: float | type[Absent] = Absent,
        speed: float | type[Absent] = Absent,
        flight_time: float | type[Absent] = Absent,
        mass: float | type[Absent] = Absent,
        agility: float | type[Absent] = Absent,
        exp_radius: float | type[Absent] = Absent,
        exp_speed: float | type[Absent] = Absent,
        drf: float | type[Absent] = Absent,
) -> int:
    attrs = {}
    _add_dmgs(basic_info=basic_info, attrs=attrs, dmgs=dmgs)
    conditional_insert(container=attrs, path=[basic_info.volume_attr_id], value=volume)
    conditional_insert(container=attrs, path=[basic_info.max_velocity_attr_id], value=speed)
    conditional_insert(container=attrs, path=[basic_info.flight_time_attr_id], value=flight_time)
    conditional_insert(container=attrs, path=[basic_info.mass_attr_id], value=mass)
    conditional_insert(container=attrs, path=[basic_info.agility_attr_id], value=agility)
    conditional_insert(container=attrs, path=[basic_info.aoe_cloud_size_attr_id], value=exp_radius)
    conditional_insert(container=attrs, path=[basic_info.aoe_velocity_attr_id], value=exp_speed)
    conditional_insert(container=attrs, path=[basic_info.aoe_drf_attr_id], value=drf)
    return client.mk_eve_item(
        attrs=attrs,
        eff_ids=[basic_info.missile_effect_id],
        defeff_id=basic_info.missile_effect_id)


def make_eve_missile_fof(
        *,
        client: TestClient,
        basic_info: DmgBasicInfo,
        dmgs: tuple[float | None, float | None, float | None, float | None] | None = None,
        volume: float | type[Absent] = Absent,
        speed: float | type[Absent] = Absent,
        flight_time: float | type[Absent] = Absent,
        mass: float | type[Absent] = Absent,
        agility: float | type[Absent] = Absent,
        max_range: float | type[Absent] = Absent,
        exp_radius: float | type[Absent] = Absent,
        exp_speed: float | type[Absent] = Absent,
        drf: float | type[Absent] = Absent,
) -> int:
    attrs = {}
    _add_dmgs(basic_info=basic_info, attrs=attrs, dmgs=dmgs)
    conditional_insert(container=attrs, path=[basic_info.volume_attr_id], value=volume)
    conditional_insert(container=attrs, path=[basic_info.max_velocity_attr_id], value=speed)
    conditional_insert(container=attrs, path=[basic_info.flight_time_attr_id], value=flight_time)
    conditional_insert(container=attrs, path=[basic_info.mass_attr_id], value=mass)
    conditional_insert(container=attrs, path=[basic_info.agility_attr_id], value=agility)
    conditional_insert(container=attrs, path=[basic_info.max_fof_range_attr_id], value=max_range)
    conditional_insert(container=attrs, path=[basic_info.aoe_cloud_size_attr_id], value=exp_radius)
    conditional_insert(container=attrs, path=[basic_info.aoe_velocity_attr_id], value=exp_speed)
    conditional_insert(container=attrs, path=[basic_info.aoe_drf_attr_id], value=drf)
    return client.mk_eve_item(
        attrs=attrs,
        eff_ids=[basic_info.missile_fof_effect_id],
        defeff_id=basic_info.missile_fof_effect_id)


def make_eve_missile_defender(
        *,
        client: TestClient,
        basic_info: DmgBasicInfo,
        dmgs: tuple[float | None, float | None, float | None, float | None] | None = None,
        volume: float | type[Absent] = Absent,
        speed: float | type[Absent] = Absent,
        flight_time: float | type[Absent] = Absent,
        mass: float | type[Absent] = Absent,
        agility: float | type[Absent] = Absent,
        exp_radius: float | type[Absent] = Absent,
        exp_speed: float | type[Absent] = Absent,
        drf: float | type[Absent] = Absent,
) -> int:
    attrs = {}
    _add_dmgs(basic_info=basic_info, attrs=attrs, dmgs=dmgs)
    conditional_insert(container=attrs, path=[basic_info.volume_attr_id], value=volume)
    conditional_insert(container=attrs, path=[basic_info.max_velocity_attr_id], value=speed)
    conditional_insert(container=attrs, path=[basic_info.flight_time_attr_id], value=flight_time)
    conditional_insert(container=attrs, path=[basic_info.mass_attr_id], value=mass)
    conditional_insert(container=attrs, path=[basic_info.agility_attr_id], value=agility)
    conditional_insert(container=attrs, path=[basic_info.aoe_cloud_size_attr_id], value=exp_radius)
    conditional_insert(container=attrs, path=[basic_info.aoe_velocity_attr_id], value=exp_speed)
    conditional_insert(container=attrs, path=[basic_info.aoe_drf_attr_id], value=drf)
    return client.mk_eve_item(
        attrs=attrs,
        eff_ids=[basic_info.missile_defender_effect_id],
        defeff_id=basic_info.missile_defender_effect_id)


def make_eve_breacher(
        *,
        client: TestClient,
        basic_info: DmgBasicInfo,
        dmg_abs: float | type[Absent] = Absent,
        dmg_rel: float | type[Absent] = Absent,
        dmg_duration: float | type[Absent] = Absent,
        volume: float | type[Absent] = Absent,
        speed: float | type[Absent] = Absent,
        flight_time: float | type[Absent] = Absent,
        mass: float | type[Absent] = Absent,
        agility: float | type[Absent] = Absent,
) -> int:
    attrs = {}
    conditional_insert(container=attrs, path=[basic_info.dmg_breach_abs_attr_id], value=dmg_abs)
    conditional_insert(container=attrs, path=[basic_info.dmg_breach_rel_attr_id], value=dmg_rel)
    conditional_insert(container=attrs, path=[basic_info.dmg_breach_duration_attr_id], value=dmg_duration)
    conditional_insert(container=attrs, path=[basic_info.volume_attr_id], value=volume)
    conditional_insert(container=attrs, path=[basic_info.max_velocity_attr_id], value=speed)
    conditional_insert(container=attrs, path=[basic_info.flight_time_attr_id], value=flight_time)
    conditional_insert(container=attrs, path=[basic_info.mass_attr_id], value=mass)
    conditional_insert(container=attrs, path=[basic_info.agility_attr_id], value=agility)
    return client.mk_eve_item(
        attrs=attrs,
        eff_ids=[basic_info.breacher_effect_id],
        defeff_id=basic_info.breacher_effect_id)


def make_eve_bomb(
        *,
        client: TestClient,
        basic_info: DmgBasicInfo,
        dmgs: tuple[float | None, float | None, float | None, float | None] | None = None,
        volume: float | type[Absent] = Absent,
        speed: float | type[Absent] = Absent,
        flight_time: float | type[Absent] = Absent,
        mass: float | type[Absent] = Absent,
        agility: float | type[Absent] = Absent,
        exp_range: float | type[Absent] = Absent,
        exp_radius: float | type[Absent] = Absent,
        neut_resist_attr: bool = False,
) -> int:
    attrs = {}
    _add_dmgs(basic_info=basic_info, attrs=attrs, dmgs=dmgs)
    conditional_insert(container=attrs, path=[basic_info.volume_attr_id], value=volume)
    conditional_insert(container=attrs, path=[basic_info.max_velocity_attr_id], value=speed)
    conditional_insert(container=attrs, path=[basic_info.flight_time_attr_id], value=flight_time)
    conditional_insert(container=attrs, path=[basic_info.mass_attr_id], value=mass)
    conditional_insert(container=attrs, path=[basic_info.agility_attr_id], value=agility)
    conditional_insert(container=attrs, path=[basic_info.emp_field_range_attr_id], value=exp_range)
    conditional_insert(container=attrs, path=[basic_info.aoe_cloud_size_attr_id], value=exp_radius)
    if neut_resist_attr:
        attrs[basic_info.resist_ref_attr_id] = basic_info.neut_resist_attr_id
    return client.mk_eve_item(
        attrs=attrs,
        eff_ids=[basic_info.bomb_effect_id],
        defeff_id=basic_info.bomb_effect_id)


def make_eve_bomb_guided(
        *,
        client: TestClient,
        basic_info: DmgBasicInfo,
        dmgs: tuple[float | None, float | None, float | None, float | None] | None = None,
        volume: float | type[Absent] = Absent,
        speed: float | type[Absent] = Absent,
        flight_time: float | type[Absent] = Absent,
        mass: float | type[Absent] = Absent,
        agility: float | type[Absent] = Absent,
        exp_range: float | type[Absent] = Absent,
        exp_radius: float | type[Absent] = Absent,
        exp_speed: float | type[Absent] = Absent,
        drf: float | type[Absent] = Absent,
) -> int:
    attrs = {}
    _add_dmgs(basic_info=basic_info, attrs=attrs, dmgs=dmgs)
    conditional_insert(container=attrs, path=[basic_info.volume_attr_id], value=volume)
    conditional_insert(container=attrs, path=[basic_info.max_velocity_attr_id], value=speed)
    conditional_insert(container=attrs, path=[basic_info.flight_time_attr_id], value=flight_time)
    conditional_insert(container=attrs, path=[basic_info.mass_attr_id], value=mass)
    conditional_insert(container=attrs, path=[basic_info.agility_attr_id], value=agility)
    conditional_insert(container=attrs, path=[basic_info.emp_field_range_attr_id], value=exp_range)
    conditional_insert(container=attrs, path=[basic_info.aoe_cloud_size_attr_id], value=exp_radius)
    conditional_insert(container=attrs, path=[basic_info.aoe_velocity_attr_id], value=exp_speed)
    conditional_insert(container=attrs, path=[basic_info.aoe_drf_attr_id], value=drf)
    return client.mk_eve_item(
        grp_id=basic_info.guided_bomb_group_id,
        attrs=attrs,
        eff_ids=[basic_info.missile_effect_id],
        defeff_id=basic_info.missile_effect_id)


def make_eve_drone(
        *,
        client: TestClient,
        basic_info: DmgBasicInfo,
        dmgs: tuple[float | None, float | None, float | None, float | None] | None = None,
        dmg_mult: float | type[Absent] = Absent,
        cycle_time: float | type[Absent] = Absent,
        range_optimal: float | type[Absent] = Absent,
        range_falloff: float | type[Absent] = Absent,
        tracking: float | type[Absent] = Absent,
        sig_resolution: float | type[Absent] = Absent,
        speed_cruise: float | type[Absent] = Absent,
        speed_chase: float | type[Absent] = Absent,
        radius: float | type[Absent] = Absent,
        sig_radius: float | type[Absent] = Absent,
        prop_sig_radius_mult: float | type[Absent] = Absent,
) -> int:
    attrs = {}
    _add_dmgs(basic_info=basic_info, attrs=attrs, dmgs=dmgs)
    conditional_insert(container=attrs, path=[basic_info.dmg_mult_attr_id], value=dmg_mult)
    conditional_insert(container=attrs, path=[basic_info.cycle_time_attr_id], value=cycle_time)
    conditional_insert(container=attrs, path=[basic_info.max_range_attr_id], value=range_optimal)
    conditional_insert(container=attrs, path=[basic_info.falloff_attr_id], value=range_falloff)
    conditional_insert(container=attrs, path=[basic_info.tracking_attr_id], value=tracking)
    conditional_insert(container=attrs, path=[basic_info.sig_resolution_attr_id], value=sig_resolution)
    conditional_insert(container=attrs, path=[basic_info.entity_cruise_speed_attr_id], value=speed_cruise)
    conditional_insert(container=attrs, path=[basic_info.max_velocity_attr_id], value=speed_chase)
    conditional_insert(container=attrs, path=[basic_info.radius_attr_id], value=radius)
    conditional_insert(container=attrs, path=[basic_info.sig_radius_attr_id], value=sig_radius)
    conditional_insert(container=attrs, path=[basic_info.prop_sig_radius_mult_attr_id], value=prop_sig_radius_mult)
    return client.mk_eve_drone(
        attrs=attrs,
        eff_ids=[basic_info.tgt_attack_effect_id],
        defeff_id=basic_info.tgt_attack_effect_id)


def make_eve_fighter_assault(
        *,
        client: TestClient,
        basic_info: DmgBasicInfo,
        prm_dmgs: tuple[float | None, float | None, float | None, float | None] | None = None,
        prm_dmg_mult: float | type[Absent] = Absent,
        prm_cycle_time: float | type[Absent] = Absent,
        prm_range_optimal: float | type[Absent] = Absent,
        prm_range_falloff: float | type[Absent] = Absent,
        prm_exp_radius: float | type[Absent] = Absent,
        prm_exp_speed: float | type[Absent] = Absent,
        prm_dr_factor: float | type[Absent] = Absent,
        prm_dr_sens: float | type[Absent] = Absent,
        sec_dmgs: tuple[float | None, float | None, float | None, float | None] | None = None,
        sec_dmg_mult: float | type[Absent] = Absent,
        sec_cycle_time: float | type[Absent] = Absent,
        sec_range: float | type[Absent] = Absent,
        sec_exp_radius: float | type[Absent] = Absent,
        sec_exp_speed: float | type[Absent] = Absent,
        sec_dr_factor: float | type[Absent] = Absent,
        sec_dr_sens: float | type[Absent] = Absent,
        sec_resist_attr_id: float | type[Absent] = Absent,
        sec_charge_count: int | type[Absent] = Absent,
        sec_charge_rearm_time: float | type[Absent] = Absent,
        sq_size: float | type[Absent] = Absent,
        refuel_duration: float | type[Absent] = Absent,
        speed: float | type[Absent] = Absent,
        radius: float | type[Absent] = Absent,
        sig_radius: float | type[Absent] = Absent,
) -> int:
    attrs = {}
    _add_dmgs(basic_info=basic_info, attrs=attrs, dmgs=prm_dmgs, dmg_attr_ids=(
        basic_info.ftr_abil_atkm_dmg_em_attr_id,
        basic_info.ftr_abil_atkm_dmg_therm_attr_id,
        basic_info.ftr_abil_atkm_dmg_kin_attr_id,
        basic_info.ftr_abil_atkm_dmg_expl_attr_id))
    conditional_insert(container=attrs, path=[basic_info.ftr_abil_atkm_dmg_mult_attr_id], value=prm_dmg_mult)
    conditional_insert(container=attrs, path=[basic_info.ftr_abil_atkm_cycle_time_attr_id], value=prm_cycle_time)
    conditional_insert(container=attrs, path=[basic_info.ftr_abil_atkm_range_optimal_attr_id], value=prm_range_optimal)
    conditional_insert(container=attrs, path=[basic_info.ftr_abil_atkm_range_falloff_attr_id], value=prm_range_falloff)
    conditional_insert(container=attrs, path=[basic_info.ftr_abil_atkm_exp_radius_attr_id], value=prm_exp_radius)
    conditional_insert(container=attrs, path=[basic_info.ftr_abil_atkm_exp_speed_attr_id], value=prm_exp_speed)
    conditional_insert(container=attrs, path=[basic_info.ftr_abil_atkm_dr_factor_attr_id], value=prm_dr_factor)
    conditional_insert(container=attrs, path=[basic_info.ftr_abil_atkm_dr_sens_attr_id], value=prm_dr_sens)
    _add_dmgs(basic_info=basic_info, attrs=attrs, dmgs=sec_dmgs, dmg_attr_ids=(
        basic_info.ftr_abil_missiles_dmg_em_attr_id,
        basic_info.ftr_abil_missiles_dmg_therm_attr_id,
        basic_info.ftr_abil_missiles_dmg_kin_attr_id,
        basic_info.ftr_abil_missiles_dmg_expl_attr_id))
    conditional_insert(container=attrs, path=[basic_info.ftr_abil_missiles_dmg_mult_attr_id], value=sec_dmg_mult)
    conditional_insert(container=attrs, path=[basic_info.ftr_abil_missiles_cycle_time_attr_id], value=sec_cycle_time)
    conditional_insert(container=attrs, path=[basic_info.ftr_abil_missiles_range_attr_id], value=sec_range)
    conditional_insert(container=attrs, path=[basic_info.ftr_abil_missiles_exp_radius_attr_id], value=sec_exp_radius)
    conditional_insert(container=attrs, path=[basic_info.ftr_abil_missiles_exp_speed_attr_id], value=sec_exp_speed)
    conditional_insert(container=attrs, path=[basic_info.ftr_abil_missiles_dr_factor_attr_id], value=sec_dr_factor)
    conditional_insert(container=attrs, path=[basic_info.ftr_abil_missiles_dr_sens_attr_id], value=sec_dr_sens)
    conditional_insert(
        container=attrs, path=[basic_info.ftr_abil_missiles_resist_ref_attr_id], value=sec_resist_attr_id)
    conditional_insert(container=attrs, path=[basic_info.max_ftr_count_attr_id], value=sq_size)
    conditional_insert(container=attrs, path=[basic_info.refuel_duration_attr_id], value=refuel_duration)
    conditional_insert(container=attrs, path=[basic_info.max_velocity_attr_id], value=speed)
    conditional_insert(container=attrs, path=[basic_info.radius_attr_id], value=radius)
    conditional_insert(container=attrs, path=[basic_info.sig_radius_attr_id], value=sig_radius)
    return client.mk_eve_fighter(
        attrs=attrs,
        eff_ids=[basic_info.ftr_abil_atkm_effect_id, basic_info.ftr_abil_missiles_effect_id],
        defeff_id=basic_info.ftr_abil_atkm_effect_id,
        abils=[
            client.mk_eve_item_abil(id_=basic_info.atkm_abil_id),
            client.mk_eve_item_abil(
                id_=basic_info.missiles_abil_id,
                charge_count=sec_charge_count,
                charge_rearm_time=sec_charge_rearm_time)])


def make_eve_fighter_lr(
        *,
        client: TestClient,
        basic_info: DmgBasicInfo,
        prm_dmgs: tuple[float | None, float | None, float | None, float | None] | None = None,
        prm_dmg_mult: float | type[Absent] = Absent,
        prm_cycle_time: float | type[Absent] = Absent,
        prm_range_optimal: float | type[Absent] = Absent,
        prm_range_falloff: float | type[Absent] = Absent,
        prm_exp_radius: float | type[Absent] = Absent,
        prm_exp_speed: float | type[Absent] = Absent,
        prm_dr_factor: float | type[Absent] = Absent,
        prm_dr_sens: float | type[Absent] = Absent,
        sec_bomb_type_id: float | type[Absent] = Absent,
        sec_cycle_time: float | type[Absent] = Absent,
        sec_charge_count: int | type[Absent] = Absent,
        sec_charge_rearm_time: float | type[Absent] = Absent,
        sq_size: float | type[Absent] = Absent,
        refuel_duration: float | type[Absent] = Absent,
        speed: float | type[Absent] = Absent,
        radius: float | type[Absent] = Absent,
        sig_radius: float | type[Absent] = Absent,
) -> int:
    attrs = {}
    _add_dmgs(basic_info=basic_info, attrs=attrs, dmgs=prm_dmgs, dmg_attr_ids=(
        basic_info.ftr_abil_atkm_dmg_em_attr_id,
        basic_info.ftr_abil_atkm_dmg_therm_attr_id,
        basic_info.ftr_abil_atkm_dmg_kin_attr_id,
        basic_info.ftr_abil_atkm_dmg_expl_attr_id))
    conditional_insert(container=attrs, path=[basic_info.ftr_abil_atkm_dmg_mult_attr_id], value=prm_dmg_mult)
    conditional_insert(container=attrs, path=[basic_info.ftr_abil_atkm_cycle_time_attr_id], value=prm_cycle_time)
    conditional_insert(container=attrs, path=[basic_info.ftr_abil_atkm_range_optimal_attr_id], value=prm_range_optimal)
    conditional_insert(container=attrs, path=[basic_info.ftr_abil_atkm_range_falloff_attr_id], value=prm_range_falloff)
    conditional_insert(container=attrs, path=[basic_info.ftr_abil_atkm_exp_radius_attr_id], value=prm_exp_radius)
    conditional_insert(container=attrs, path=[basic_info.ftr_abil_atkm_exp_speed_attr_id], value=prm_exp_speed)
    conditional_insert(container=attrs, path=[basic_info.ftr_abil_atkm_dr_factor_attr_id], value=prm_dr_factor)
    conditional_insert(container=attrs, path=[basic_info.ftr_abil_atkm_dr_sens_attr_id], value=prm_dr_sens)
    conditional_insert(container=attrs, path=[basic_info.ftr_abil_bomb_type_attr_id], value=sec_bomb_type_id)
    conditional_insert(container=attrs, path=[basic_info.ftr_abil_bomb_cycle_time_attr_id], value=sec_cycle_time)
    conditional_insert(container=attrs, path=[basic_info.max_ftr_count_attr_id], value=sq_size)
    conditional_insert(container=attrs, path=[basic_info.refuel_duration_attr_id], value=refuel_duration)
    conditional_insert(container=attrs, path=[basic_info.max_velocity_attr_id], value=speed)
    conditional_insert(container=attrs, path=[basic_info.radius_attr_id], value=radius)
    conditional_insert(container=attrs, path=[basic_info.sig_radius_attr_id], value=sig_radius)
    return client.mk_eve_fighter(
        attrs=attrs,
        eff_ids=[basic_info.ftr_abil_atkm_effect_id, basic_info.ftr_abil_bomb_effect_id],
        defeff_id=basic_info.ftr_abil_atkm_effect_id,
        abils=[
            client.mk_eve_item_abil(id_=basic_info.atkm_abil_id),
            client.mk_eve_item_abil(
                id_=basic_info.bomb_abil_id,
                charge_count=sec_charge_count,
                charge_rearm_time=sec_charge_rearm_time)])


def make_eve_smartbomb(
        *,
        client: TestClient,
        basic_info: DmgBasicInfo,
        dmgs: tuple[float | None, float | None, float | None, float | None] | None = None,
        cycle_time: float | type[Absent] = Absent,
        range_optimal: float | type[Absent] = Absent,
) -> int:
    attrs = {}
    _add_dmgs(basic_info=basic_info, attrs=attrs, dmgs=dmgs)
    conditional_insert(container=attrs, path=[basic_info.cycle_time_attr_id], value=cycle_time)
    conditional_insert(container=attrs, path=[basic_info.emp_field_range_attr_id], value=range_optimal)
    return client.mk_eve_item(
        attrs=attrs,
        eff_ids=[basic_info.smartbomb_effect_id],
        defeff_id=basic_info.smartbomb_effect_id)


def make_eve_pds(
        *,
        client: TestClient,
        basic_info: DmgBasicInfo,
        dmg_mult: float | type[Absent] = Absent,
        cycle_time: float | type[Absent] = Absent,
        capacity: float | type[Absent] = Absent,
        reload_time: float | type[Absent] = Absent,
        range_optimal: float | type[Absent] = Absent,
) -> int:
    attrs = {basic_info.charge_rate_attr_id: 200}
    conditional_insert(container=attrs, path=[basic_info.dmg_mult_attr_id], value=dmg_mult)
    conditional_insert(container=attrs, path=[basic_info.cycle_time_attr_id], value=cycle_time)
    conditional_insert(container=attrs, path=[basic_info.capacity_attr_id], value=capacity)
    conditional_insert(container=attrs, path=[basic_info.reload_time_attr_id], value=reload_time)
    conditional_insert(container=attrs, path=[basic_info.emp_field_range_attr_id], value=range_optimal)
    return client.mk_eve_item(
        attrs=attrs,
        eff_ids=[basic_info.pds_effect_id],
        defeff_id=basic_info.pds_effect_id)


def make_eve_dd_direct_amarr(
        *,
        client: TestClient,
        basic_info: DmgBasicInfo,
        dmgs: tuple[float | None, float | None, float | None, float | None] | None = None,
        cycle_time: float | type[Absent] = Absent,
        delay: float | type[Absent] = Absent,
) -> int:
    attrs = {}
    _add_dmgs(basic_info=basic_info, attrs=attrs, dmgs=dmgs)
    conditional_insert(container=attrs, path=[basic_info.cycle_time_attr_id], value=cycle_time)
    conditional_insert(container=attrs, path=[basic_info.dd_delay1_attr_id], value=delay)
    return client.mk_eve_item(
        attrs=attrs,
        eff_ids=[basic_info.dd_direct_amarr_effect_id],
        defeff_id=basic_info.dd_direct_amarr_effect_id)


def make_eve_dd_direct_caldari(
        *,
        client: TestClient,
        basic_info: DmgBasicInfo,
        dmgs: tuple[float | None, float | None, float | None, float | None] | None = None,
        cycle_time: float | type[Absent] = Absent,
        delay: float | type[Absent] = Absent,
) -> int:
    attrs = {}
    _add_dmgs(basic_info=basic_info, attrs=attrs, dmgs=dmgs)
    conditional_insert(container=attrs, path=[basic_info.cycle_time_attr_id], value=cycle_time)
    conditional_insert(container=attrs, path=[basic_info.dd_delay1_attr_id], value=delay)
    return client.mk_eve_item(
        attrs=attrs,
        eff_ids=[basic_info.dd_direct_caldari_effect_id],
        defeff_id=basic_info.dd_direct_caldari_effect_id)


def make_eve_dd_direct_gallente(
        *,
        client: TestClient,
        basic_info: DmgBasicInfo,
        dmgs: tuple[float | None, float | None, float | None, float | None] | None = None,
        cycle_time: float | type[Absent] = Absent,
        delay: float | type[Absent] = Absent,
) -> int:
    attrs = {}
    _add_dmgs(basic_info=basic_info, attrs=attrs, dmgs=dmgs)
    conditional_insert(container=attrs, path=[basic_info.cycle_time_attr_id], value=cycle_time)
    conditional_insert(container=attrs, path=[basic_info.dd_delay1_attr_id], value=delay)
    return client.mk_eve_item(
        attrs=attrs,
        eff_ids=[basic_info.dd_direct_gallente_effect_id],
        defeff_id=basic_info.dd_direct_gallente_effect_id)


def make_eve_dd_direct_minmatar(
        *,
        client: TestClient,
        basic_info: DmgBasicInfo,
        dmgs: tuple[float | None, float | None, float | None, float | None] | None = None,
        cycle_time: float | type[Absent] = Absent,
        delay: float | type[Absent] = Absent,
) -> int:
    attrs = {}
    _add_dmgs(basic_info=basic_info, attrs=attrs, dmgs=dmgs)
    conditional_insert(container=attrs, path=[basic_info.cycle_time_attr_id], value=cycle_time)
    conditional_insert(container=attrs, path=[basic_info.dd_delay1_attr_id], value=delay)
    return client.mk_eve_item(
        attrs=attrs,
        eff_ids=[basic_info.dd_direct_minmatar_effect_id],
        defeff_id=basic_info.dd_direct_minmatar_effect_id)


def make_eve_dd_lance(
        *,
        client: TestClient,
        basic_info: DmgBasicInfo,
        dmgs: tuple[float | None, float | None, float | None, float | None] | None = None,
        cycle_time: float | type[Absent] = Absent,
        delay: float | type[Absent] = Absent,
        dmg_interval: float | type[Absent] = Absent,
        dmg_duration: float | type[Absent] = Absent,
        range_optimal: float | type[Absent] = Absent,
        dmg_radius: float | type[Absent] = Absent,
) -> int:
    attrs = {}
    _add_dmgs(basic_info=basic_info, attrs=attrs, dmgs=dmgs)
    conditional_insert(container=attrs, path=[basic_info.cycle_time_attr_id], value=cycle_time)
    conditional_insert(container=attrs, path=[basic_info.dd_delay2_attr_id], value=delay)
    conditional_insert(container=attrs, path=[basic_info.dd_dmg_interval_attr_id], value=dmg_interval)
    conditional_insert(container=attrs, path=[basic_info.dd_dmg_duration_attr_id], value=dmg_duration)
    conditional_insert(container=attrs, path=[basic_info.max_range_attr_id], value=range_optimal)
    conditional_insert(container=attrs, path=[basic_info.sig_radius_attr_id], value=dmg_radius)
    return client.mk_eve_item(
        attrs=attrs,
        eff_ids=[basic_info.dd_lance_effect_id],
        defeff_id=basic_info.dd_lance_effect_id)


def make_eve_dd_lance_debuff(
        *,
        client: TestClient,
        basic_info: DmgBasicInfo,
        dmgs: tuple[float | None, float | None, float | None, float | None] | None = None,
        cycle_time: float | type[Absent] = Absent,
        delay: float | type[Absent] = Absent,
        dmg_interval: float | type[Absent] = Absent,
        dmg_duration: float | type[Absent] = Absent,
        range_optimal: float | type[Absent] = Absent,
        dmg_radius: float | type[Absent] = Absent,
) -> int:
    attrs = {}
    _add_dmgs(basic_info=basic_info, attrs=attrs, dmgs=dmgs)
    conditional_insert(container=attrs, path=[basic_info.cycle_time_attr_id], value=cycle_time)
    conditional_insert(container=attrs, path=[basic_info.dd_delay2_attr_id], value=delay)
    conditional_insert(container=attrs, path=[basic_info.dd_dmg_interval_attr_id], value=dmg_interval)
    conditional_insert(container=attrs, path=[basic_info.dd_dmg_duration_attr_id], value=dmg_duration)
    conditional_insert(container=attrs, path=[basic_info.max_range_attr_id], value=range_optimal)
    conditional_insert(container=attrs, path=[basic_info.sig_radius_attr_id], value=dmg_radius)
    return client.mk_eve_item(
        attrs=attrs,
        eff_ids=[basic_info.dd_lance_debuff_effect_id],
        defeff_id=basic_info.dd_lance_debuff_effect_id)


def make_eve_dd_reaper(
        *,
        client: TestClient,
        basic_info: DmgBasicInfo,
        dmgs: tuple[float | None, float | None, float | None, float | None] | None = None,
        cycle_time: float | type[Absent] = Absent,
        delay: float | type[Absent] = Absent,
        dmg_interval: float | type[Absent] = Absent,
        dmg_duration: float | type[Absent] = Absent,
        range_optimal: float | type[Absent] = Absent,
        dmg_radius: float | type[Absent] = Absent,
) -> int:
    attrs = {}
    _add_dmgs(basic_info=basic_info, attrs=attrs, dmgs=dmgs)
    conditional_insert(container=attrs, path=[basic_info.cycle_time_attr_id], value=cycle_time)
    conditional_insert(container=attrs, path=[basic_info.dd_delay2_attr_id], value=delay)
    conditional_insert(container=attrs, path=[basic_info.dd_dmg_interval_attr_id], value=dmg_interval)
    conditional_insert(container=attrs, path=[basic_info.dd_dmg_duration_attr_id], value=dmg_duration)
    conditional_insert(container=attrs, path=[basic_info.max_range_attr_id], value=range_optimal)
    conditional_insert(container=attrs, path=[basic_info.sig_radius_attr_id], value=dmg_radius)
    return client.mk_eve_item(
        attrs=attrs,
        eff_ids=[basic_info.dd_reaper_effect_id],
        defeff_id=basic_info.dd_reaper_effect_id)


def make_eve_dd_boson(
        *,
        client: TestClient,
        basic_info: DmgBasicInfo,
        dmgs: tuple[float | None, float | None, float | None, float | None] | None = None,
        cycle_time: float | type[Absent] = Absent,
        delay: float | type[Absent] = Absent,
        dmg_interval: float | type[Absent] = Absent,
        dmg_duration: float | type[Absent] = Absent,
        range_optimal: float | type[Absent] = Absent,
        dmg_radius: float | type[Absent] = Absent,
) -> int:
    attrs = {}
    _add_dmgs(basic_info=basic_info, attrs=attrs, dmgs=dmgs)
    conditional_insert(container=attrs, path=[basic_info.cycle_time_attr_id], value=cycle_time)
    conditional_insert(container=attrs, path=[basic_info.dd_delay2_attr_id], value=delay)
    conditional_insert(container=attrs, path=[basic_info.dd_dmg_interval_attr_id], value=dmg_interval)
    conditional_insert(container=attrs, path=[basic_info.dd_dmg_duration_attr_id], value=dmg_duration)
    conditional_insert(container=attrs, path=[basic_info.max_range_attr_id], value=range_optimal)
    conditional_insert(container=attrs, path=[basic_info.sig_radius_attr_id], value=dmg_radius)
    return client.mk_eve_item(
        attrs=attrs,
        eff_ids=[basic_info.dd_boson_effect_id],
        defeff_id=basic_info.dd_boson_effect_id)


def make_eve_dd_vorton(
        *,
        client: TestClient,
        basic_info: DmgBasicInfo,
        dmgs: tuple[float | None, float | None, float | None, float | None] | None = None,
        cycle_time: float | type[Absent] = Absent,
        delay: float | type[Absent] = Absent,
) -> int:
    attrs = {}
    _add_dmgs(basic_info=basic_info, attrs=attrs, dmgs=dmgs)
    conditional_insert(container=attrs, path=[basic_info.cycle_time_attr_id], value=cycle_time)
    conditional_insert(container=attrs, path=[basic_info.dd_delay1_attr_id], value=delay)
    return client.mk_eve_item(
        attrs=attrs,
        eff_ids=[basic_info.dd_vorton_effect_id],
        defeff_id=basic_info.dd_vorton_effect_id)


def _add_dmgs(
        *,
        basic_info: DmgBasicInfo,
        attrs: dict[int, float],
        dmgs: tuple[float | None, float | None, float | None, float | None] | None = None,
        dmg_attr_ids: tuple[float, float, float, float] | None = None,
) -> None:
    if dmgs is not None:
        if dmg_attr_ids is None:
            dmg_attr_ids = (
                basic_info.dmg_em_attr_id,
                basic_info.dmg_therm_attr_id,
                basic_info.dmg_kin_attr_id,
                basic_info.dmg_expl_attr_id)
        attrs.update({k: v for k, v in zip(dmg_attr_ids, dmgs, strict=True) if v is not None})
