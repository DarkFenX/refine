"""
This module contains tests for various mobility restrictions imposed by various item (either when
used by a fit, or applied onto certain items).

Results of testing are put as comments into specific tests. List mark signifies if something was
blocked or not (+ blocked, - not blocked, ? unclear). Most results mention type of error message
received, since it can be useful to figure out what's going on under the hood.
"""

from dataclasses import dataclass

from fw import approx
from fw.api import ItemStatsOptions, ValOptions


@dataclass(kw_only=True)
class EveBasics:
    # Attrs
    speed_attr_id: int
    speed_factor_attr_id: int
    warp_status_attr_id: int
    warp_scram_attr_id: int
    gate_status_attr_id: int
    gate_scram_attr_id: int
    disallow_tether_attr_id: int
    disallow_dock_attr_id: int
    can_cloak_attr_id: int
    # Items
    cloak_t2_id: int
    assist_id: int
    offense_id: int


# Use shared setup for all tests, even if specific effects do not need it. This reduces chance that
# something has been missed in test setup for a specific effect.
def setup_basics(*, client, consts) -> EveBasics:
    # Attrs
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_speed_factor_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_warp_scram_attr_id = client.mk_eve_attr(id_=consts.EveAttr.siege_mod_warp_status)
    eve_warp_status_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status)
    eve_gate_scram_attr_id = client.mk_eve_attr(id_=consts.EveAttr.gate_scramble_strength, def_val=1)
    eve_gate_status_attr_id = client.mk_eve_attr(id_=consts.EveAttr.gate_scramble_status, def_val=-1000)
    eve_disallow_cloak_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_cloaking)
    eve_disallow_tether_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_tethering)
    eve_disallow_dock_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_docking)
    eve_drive_jump_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_drive_jumping)
    client.mk_eve_attr(id_=consts.EveAttr.disallow_assistance)
    client.mk_eve_attr(id_=consts.EveAttr.disallow_offensive_modifiers)
    eve_can_cloak_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_cloak, def_val=1)
    eve_range_attr_id = client.mk_eve_attr()
    # Buffs
    eve_warp_buff_id = client.mk_eve_buff(
        id_=consts.EveBuff.warp_penalty,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.mod_add,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_warp_status_attr_id)])
    eve_dock_jump_buff_id = client.mk_eve_buff(
        id_=consts.EveBuff.disallow_dock_jump,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.mod_add,
        item_mods=[
            client.mk_eve_buff_mod(attr_id=eve_disallow_dock_attr_id),
            client.mk_eve_buff_mod(attr_id=eve_drive_jump_attr_id)])
    eve_tether_buff_id = client.mk_eve_buff(
        id_=consts.EveBuff.disallow_tether,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.mod_add,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_disallow_tether_attr_id)])
    eve_cloak_buff_id = client.mk_eve_buff(
        id_=consts.EveBuff.disallow_cloak,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.mod_add,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_disallow_cloak_attr_id)])
    # Cloak
    eve_cloak_t2_effect_id = client.mk_eve_effect(id_=consts.EveEffect.cloaking, cat_id=consts.EveEffCat.active)
    eve_cloak_t2_id = client.mk_eve_item(eff_ids=[eve_cloak_t2_effect_id], defeff_id=eve_cloak_t2_effect_id)
    # Assistive module
    eve_assist_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        is_assistance=True,
        range_attr_id=eve_range_attr_id)
    eve_assist_id = client.mk_eve_item(
        attrs={eve_range_attr_id: 10000},
        eff_ids=[eve_assist_effect_id],
        defeff_id=eve_assist_effect_id)
    # Offensive module
    eve_mod_src_attr_id = client.mk_eve_attr()
    eve_mod_tgt_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_src_attr_id,
        affectee_attr_id=eve_mod_tgt_attr_id)
    eve_offense_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        is_offensive=True,
        range_attr_id=eve_range_attr_id,
        mod_info=[eve_mod])
    eve_offense_id = client.mk_eve_item(
        attrs={eve_mod_src_attr_id: -50, eve_range_attr_id: 10000},
        eff_ids=[eve_offense_effect_id],
        defeff_id=eve_offense_effect_id)
    # Item to house all the unused buffs and effects, so that they do not get cleaned up
    eve_buff1_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff2_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_2_id)
    eve_buff3_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_3_id)
    eve_buff4_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_4_id)
    client.mk_eve_item(attrs={
        eve_buff1_attr_id: eve_warp_buff_id, eve_buff2_attr_id: eve_dock_jump_buff_id,
        eve_buff3_attr_id: eve_tether_buff_id, eve_buff4_attr_id: eve_cloak_buff_id})
    return EveBasics(
        speed_attr_id=eve_speed_attr_id,
        speed_factor_attr_id=eve_speed_factor_attr_id,
        warp_status_attr_id=eve_warp_status_attr_id,
        warp_scram_attr_id=eve_warp_scram_attr_id,
        gate_status_attr_id=eve_gate_status_attr_id,
        gate_scram_attr_id=eve_gate_scram_attr_id,
        disallow_tether_attr_id=eve_disallow_tether_attr_id,
        disallow_dock_attr_id=eve_disallow_dock_attr_id,
        can_cloak_attr_id=eve_can_cloak_attr_id,
        cloak_t2_id=eve_cloak_t2_id,
        assist_id=eve_assist_id,
        offense_id=eve_offense_id)


def run_dd_test(*, client, consts, dd_effect_id: int, is_targeted: bool = False):
    eve_basics = setup_basics(client=client, consts=consts)
    eve_dd_effect_id = client.mk_eve_effect(
        id_=dd_effect_id,
        cat_id=consts.EveEffCat.target if is_targeted else consts.EveEffCat.active,
        is_offensive=True)
    eve_dd_id = client.mk_eve_item(
        attrs={
            eve_basics.warp_scram_attr_id: 100,
            eve_basics.disallow_tether_attr_id: 1, eve_basics.disallow_dock_attr_id: 1},
        eff_ids=[eve_dd_effect_id],
        defeff_id=eve_dd_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basics.speed_attr_id: 100, eve_basics.gate_status_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_dd_id, state=consts.ApiModuleState.active)
    # Verification
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        speed=True,
        can_warp=True,
        can_jump_gate=True,
        can_jump_wormhole=True,
        can_jump_drive=True,
        can_dock_station=True,
        can_dock_citadel=True,
        can_tether=True))
    api_ship.update()
    assert api_ship_stats.speed.one() == approx(100)
    assert api_ship_stats.can_warp.one() is False
    assert api_ship_stats.can_jump_gate.one() is False
    assert api_ship_stats.can_jump_wormhole.one() is True
    assert api_ship_stats.can_jump_drive.one() is False
    assert api_ship_stats.can_dock_station.one() is False
    assert api_ship_stats.can_dock_citadel.one() is False
    assert api_ship_stats.can_tether.one() is False
    # Action
    api_fit.add_module(type_id=eve_basics.cloak_t2_id, state=consts.ApiModuleState.active)
    # Verification
    assert api_fit.validate(options=ValOptions(cloaking_blocked=True)).passed is False
    # Action
    api_proj_fit = api_sol.create_fit()
    api_proj_fit.add_module(
        type_id=eve_basics.assist_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    api_proj_fit.add_module(
        type_id=eve_basics.offense_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    # Verification
    assert api_proj_fit.validate(options=ValOptions(assist_immunity=True)).passed is True
    assert api_proj_fit.validate(options=ValOptions(offense_immunity=True)).passed is True


def test_dd_direct(client, consts):
    """
    Tested on Singularity on 2026-06-15 and 2026-07-26, using Leviathan and kinetic direct DD. Rapid
    firing was trained, so cycle time was 240 and 252 (different for different tests). The target
    was a freighter released by the same pilot, so using DD did not yield any timers, but
    restrictions were applied regardless.

    Using DD applies following self-(de)buffs:
    - Warp Disabled/Warp penalty, 30s
    - Cloak Disruption/Disallow Cloak, 5m
    - Unable to Tether/Tether blocked, 5m

    Prevented actions/interactions:
    + warp (external factors, 30s)
    + jump gate (special, 5m)
    ? jump wormhole (ship size message is shown regardless)
    + jump drive (external factors, 30s; special, 5m)
    + dock station (external factors, 5m)
    + dock citadel (external factors, 5m)
    + tether (5m)
    + cloak (special, 5m)
    - regular movement
    - incoming assistance
    - incoming offensive mods
    """
    run_dd_test(client=client, consts=consts, dd_effect_id=consts.EveEffect.super_weapon_amarr, is_targeted=True)


def test_dd_lance(client, consts):
    """
    Tested on Singularity on 2026-06-15 and 2026-07-26, using Leviathan and kinetic lance DD. Using
    lance refreshes 1m weapons timer over whole duration of DD cycle. Rapid firing was trained, so
    cycle time was 240 and 252 (different for different tests).

    Using DD applies following self-(de)buffs:
    - Warp Disabled/Warp penalty, 30s
    - Cloak Disruption/Disallow Cloak, 5m
    - Unable to Tether/Tether blocked, 5m

    Prevented actions/interactions:
    + warp (external factors, 30s)
    + jump gate (special, 5m; weapons timer for effect duration + 1m)
    ? jump wormhole (ship size message is shown regardless)
    + jump drive (external factors, 30s; special, 5m)
    + dock station (external factors, 5m)
    + dock citadel (external factors, 5m)
    + tether (5m)
    + cloak (special, 5m)
    - regular movement
    - incoming assistance
    - incoming offensive mods
    """
    run_dd_test(client=client, consts=consts, dd_effect_id=consts.EveEffect.doomsday_beam_dot)


def test_dd_reaper(client, consts):
    """
    Tested on Singularity on 2026-06-15 and 2026-07-26, using Leviathan and kinetic reaper DD. Using
    reaper refreshes 1m weapons timer over whole duration of DD cycle. Rapid firing was trained, so
    cycle time was 240 and 252 (different for different tests).

    Using DD applies following self-(de)buffs:
    - Warp Disabled/Warp penalty, 30s
    - Cloak Disruption/Disallow Cloak, 5m
    - Unable to Tether/Tether blocked, 5m

    Prevented actions/interactions:
    + warp (external factors, 30s)
    + jump gate (special, 5m; weapons timer for effect duration + 1m)
    ? jump wormhole (ship size message is shown regardless)
    + jump drive (external factors, 30s; special, 5m)
    + dock station (external factors, 5m)
    + dock citadel (external factors, 5m)
    + tether (5m)
    + cloak (special, 5m)
    - regular movement
    - incoming assistance
    - incoming offensive mods
    """
    run_dd_test(client=client, consts=consts, dd_effect_id=consts.EveEffect.doomsday_slash)


def test_dd_bosonic(client, consts):
    """
    Tested on Singularity on 2026-06-15 and 2026-07-26, using Leviathan and bosonic DD. Using
    bosonic refreshes 1m weapons timer over whole duration of DD cycle. Rapid firing was trained, so
    cycle time was 240 and 252 (different for different tests).

    Using DD applies following self-(de)buffs:
    - Warp Disabled/Warp penalty, 30s
    - Cloak Disruption/Disallow Cloak, 5m
    - Unable to Tether/Tether blocked, 5m

    Prevented actions/interactions:
    + warp (external factors, 30s)
    + jump gate (special, 5m; weapons timer for effect duration + 1m)
    ? jump wormhole (ship size message is shown regardless)
    + jump drive (external factors, 30s; special, 5m)
    + dock station (external factors, 5m)
    + dock citadel (external factors, 5m)
    + tether (5m)
    + cloak (special, 5m)
    - regular movement
    - incoming assistance
    - incoming offensive mods
    """
    run_dd_test(client=client, consts=consts, dd_effect_id=consts.EveEffect.doomsday_cone_dot)


def test_dd_gtfo(client, consts):
    """
    Tested on Singularity on 2026-06-15, using Leviathan and GTFO DD. No weapons timer notes
    recorded for this specific test.

    Using DD applies following self-(de)buffs:
    - Warp Disabled/Warp penalty, 30s
    - Cloak Disruption/Disallow Cloak, 5m
    - Unable to Tether/Tether blocked, 5m

    Prevented actions/interactions:
    + warp (external factors, 30s)
    + jump gate (special, 5m; weapons timer for effect duration + 1m)
    ? jump wormhole (ship size message is shown regardless)
    + jump drive (external factors, 30s; special, 5m)
    + dock station (external factors, 5m)
    + dock citadel (external factors, 5m)
    + tether (5m)
    + cloak (special, 5m)
    - regular movement
    ? incoming assistance (assumed to be consistent with other DDs)
    ? incoming offensive mods (assumed to be consistent with other DDs)
    """
    run_dd_test(client=client, consts=consts, dd_effect_id=consts.EveEffect.doomsday_hog)


def test_dd_debuff_lance(client, consts):
    """
    Tested on Singularity on 2026-07-26, using Karura and kinetic debuff-lance DD. Using lance
    refreshes 1m weapons timer over whole duration of DD cycle. To separate effects of debuff lance
    from effects of siege, lance was used when siege cycle was about to end.

    Using DD applies following self-(de)buffs:
    - Warp Disabled/Warp penalty, 30s
    - Cloak Disruption/Disallow Cloak, 2m
    - Unable to Tether/Tether blocked, 2m

    Prevented actions/interactions:
    + warp (external factors)
    + jump gate (special, 2m; weapons timer for effect duration + 1m)
    - jump wormhole
    + jump drive (external factors 30s, custom "you can't jump now" until 2m)
    + dock station (external factors 2m, then weapons timer msg)
    + dock citadel (external factors 2m, then weapons timer msg)
    + tether (at least because it's weapons timer over whole effect duration + 1m)
    + cloak (special, for full cycle duration) - standard for cloak
    - regular movement
    - incoming assistance
    - incoming offensive mods
    """
    run_dd_test(client=client, consts=consts, dd_effect_id=consts.EveEffect.debuff_lance)


def test_phenom(client, consts):
    """
    Tested on Singularity on 2026-06-15 and 2026-07-26, using Leviathan and caldari phenom. Using
    phenom applies 1m weapons timer upon use, but then does not refresh it. It seems like a special
    mechanic, the effect itself is not marked as offensive. All the restrictions seem to be applied
    by the weapons timer.

    Prevented actions/interactions:
    - warp
    + jump gate (weapons timer, 1m)
    ? jump wormhole (ship size message is shown regardless)
    - jump drive
    + dock station (weapons timer, 1m)
    + dock citadel (weapons timer, 1m)
    + tether (weapons timer, 1m)
    - cloak
    - regular movement
    - incoming assistance
    - incoming offensive mods
    """
    eve_basics = setup_basics(client=client, consts=consts)
    eve_phenom_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.mod_titan_effect_generator,
        cat_id=consts.EveEffCat.active)
    eve_phenom_id = client.mk_eve_item(
        attrs={eve_basics.disallow_tether_attr_id: 1, eve_basics.disallow_dock_attr_id: 1},
        eff_ids=[eve_phenom_effect_id],
        defeff_id=eve_phenom_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basics.speed_attr_id: 100, eve_basics.gate_status_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_phenom_id, state=consts.ApiModuleState.active)
    # Verification
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        speed=True,
        can_warp=True,
        can_jump_gate=True,
        can_jump_wormhole=True,
        can_jump_drive=True,
        can_dock_station=True,
        can_dock_citadel=True,
        can_tether=True))
    api_ship.update()
    assert api_ship_stats.speed.one() == approx(100)
    assert api_ship_stats.can_warp.one() is True
    assert api_ship_stats.can_jump_gate.one() is False
    assert api_ship_stats.can_jump_wormhole.one() is True
    assert api_ship_stats.can_jump_drive.one() is True
    assert api_ship_stats.can_dock_station.one() is False
    assert api_ship_stats.can_dock_citadel.one() is False
    assert api_ship_stats.can_tether.one() is False
    # Action
    api_fit.add_module(type_id=eve_basics.cloak_t2_id, state=consts.ApiModuleState.active)
    # Verification
    assert api_fit.validate(options=ValOptions(cloaking_blocked=True)).passed is True
    # Action
    api_proj_fit = api_sol.create_fit()
    api_proj_fit.add_module(
        type_id=eve_basics.assist_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    api_proj_fit.add_module(
        type_id=eve_basics.offense_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    # Verification
    assert api_proj_fit.validate(options=ValOptions(assist_immunity=True)).passed is True
    assert api_proj_fit.validate(options=ValOptions(offense_immunity=True)).passed is True


def test_burst_projector(client, consts):
    """
    Tested on Singularity on 2026-06-15 and 2026-07-26, using Hel and various burst projectors: ECM
    and non-ECM went through full set of checks (ECM is special because it has canCloak=0); for some
    simpler tests (warp/cloak/jump drive), all of them were tested.

    Prevented actions/interactions:
    - warp
    + jump gate (weapons timer, effect duration + 1m)
    ? jump wormhole (ship size message is shown regardless)
    - jump drive
    + dock station (weapons timer would prevent, but ship size message is shown first)
    + dock citadel (weapons timer, effect duration + 1m)
    + tether
    + cloak (special)
    - regular movement
    - incoming assistance
    - incoming offensive mods
    """
    eve_basics = setup_basics(client=client, consts=consts)
    eve_phenom_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.doomsday_aoe_neut,
        cat_id=consts.EveEffCat.active,
        is_offensive=True)
    eve_phenom_id = client.mk_eve_item(
        attrs={eve_basics.disallow_dock_attr_id: 1},
        eff_ids=[eve_phenom_effect_id],
        defeff_id=eve_phenom_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basics.speed_attr_id: 100, eve_basics.gate_status_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_phenom_id, state=consts.ApiModuleState.active)
    # Verification
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        speed=True,
        can_warp=True,
        can_jump_gate=True,
        can_jump_wormhole=True,
        can_jump_drive=True,
        can_dock_station=True,
        can_dock_citadel=True,
        can_tether=True))
    api_ship.update()
    assert api_ship_stats.speed.one() == approx(100)
    assert api_ship_stats.can_warp.one() is True
    assert api_ship_stats.can_jump_gate.one() is False
    assert api_ship_stats.can_jump_wormhole.one() is True
    assert api_ship_stats.can_jump_drive.one() is True
    assert api_ship_stats.can_dock_station.one() is False
    assert api_ship_stats.can_dock_citadel.one() is False
    assert api_ship_stats.can_tether.one() is False
    # Action
    api_fit.add_module(type_id=eve_basics.cloak_t2_id, state=consts.ApiModuleState.active)
    # Verification
    assert api_fit.validate(options=ValOptions(cloaking_blocked=True)).passed is False
    # Action
    api_proj_fit = api_sol.create_fit()
    api_proj_fit.add_module(
        type_id=eve_basics.assist_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    api_proj_fit.add_module(
        type_id=eve_basics.offense_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    # Verification
    assert api_proj_fit.validate(options=ValOptions(assist_immunity=True)).passed is True
    assert api_proj_fit.validate(options=ValOptions(offense_immunity=True)).passed is True


def test_nsa(client, consts):
    """
    Tested on Singularity on 2026-06-15 and 2026-07-26, using Hel.

    Prevented actions/interactions:
    + warp (external factors)
    - jump gate
    ? jump wormhole (ship size message is shown regardless)
    + jump drive (external factors)
    + dock station (message is changed to external factors)
    + dock citadel (external factors)
    + tether
    + cloak (special)
    - regular movement
    - incoming assistance
    - incoming offensive mods
    """
    eve_basics = setup_basics(client=client, consts=consts)
    # NSA has many modifiers, but only 3 of those are relevant
    eve_warp_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_basics.warp_scram_attr_id,
        affectee_attr_id=eve_basics.warp_status_attr_id)
    eve_dock_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_basics.disallow_dock_attr_id,
        affectee_attr_id=eve_basics.disallow_dock_attr_id)
    eve_tether_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_basics.disallow_tether_attr_id,
        affectee_attr_id=eve_basics.disallow_tether_attr_id)
    eve_nsa_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_networked_sensor_array,
        cat_id=consts.EveEffCat.active,
        mod_info=[eve_warp_mod, eve_dock_mod, eve_tether_mod])
    eve_nsa_id = client.mk_eve_item(
        attrs={
            eve_basics.warp_scram_attr_id: 100, eve_basics.can_cloak_attr_id: 0,
            eve_basics.disallow_dock_attr_id: 1, eve_basics.disallow_tether_attr_id: 1},
        eff_ids=[eve_nsa_effect_id],
        defeff_id=eve_nsa_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basics.speed_attr_id: 100, eve_basics.gate_status_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_nsa_id, state=consts.ApiModuleState.active)
    # Verification
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        speed=True,
        can_warp=True,
        can_jump_gate=True,
        can_jump_wormhole=True,
        can_jump_drive=True,
        can_dock_station=True,
        can_dock_citadel=True,
        can_tether=True))
    api_ship.update()
    assert api_ship_stats.speed.one() == approx(100)
    assert api_ship_stats.can_warp.one() is False
    assert api_ship_stats.can_jump_gate.one() is True
    assert api_ship_stats.can_jump_wormhole.one() is True
    assert api_ship_stats.can_jump_drive.one() is False
    assert api_ship_stats.can_dock_station.one() is False
    assert api_ship_stats.can_dock_citadel.one() is False
    assert api_ship_stats.can_tether.one() is False
    # Action
    api_fit.add_module(type_id=eve_basics.cloak_t2_id, state=consts.ApiModuleState.active)
    # Verification
    assert api_fit.validate(options=ValOptions(cloaking_blocked=True)).passed is False
    # Action
    api_proj_fit = api_sol.create_fit()
    api_proj_fit.add_module(
        type_id=eve_basics.assist_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    api_proj_fit.add_module(
        type_id=eve_basics.offense_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    # Verification
    assert api_proj_fit.validate(options=ValOptions(assist_immunity=True)).passed is True
    assert api_proj_fit.validate(options=ValOptions(offense_immunity=True)).passed is True


def test_isa(client, consts):
    """
    Tested on Singularity on 2026-06-15 and 2026-07-26, using Nidhoggur.

    Prevented actions/interactions:
    + warp (external factors)
    - jump gate
    - jump wormhole
    + jump drive (external factors)
    + dock station (external factors)
    + dock citadel (external factors)
    + tether
    + cloak (special)
    - regular movement
    - incoming assistance
    - incoming offensive mods
    - MJDs/MJFGs (not tested here, but only NSA has effects to block those, despite supers being
      unable to fit them)
    """
    eve_basics = setup_basics(client=client, consts=consts)
    # ISA has many modifiers, but only 3 of those are relevant
    eve_warp_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_basics.warp_scram_attr_id,
        affectee_attr_id=eve_basics.warp_status_attr_id)
    eve_dock_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_basics.disallow_dock_attr_id,
        affectee_attr_id=eve_basics.disallow_dock_attr_id)
    eve_tether_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_basics.disallow_tether_attr_id,
        affectee_attr_id=eve_basics.disallow_tether_attr_id)
    eve_nsa_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_integrated_sensor_array,
        cat_id=consts.EveEffCat.active,
        mod_info=[eve_warp_mod, eve_dock_mod, eve_tether_mod])
    eve_nsa_id = client.mk_eve_item(
        attrs={
            eve_basics.warp_scram_attr_id: 100, eve_basics.can_cloak_attr_id: 0,
            eve_basics.disallow_dock_attr_id: 1, eve_basics.disallow_tether_attr_id: 1},
        eff_ids=[eve_nsa_effect_id],
        defeff_id=eve_nsa_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basics.speed_attr_id: 100, eve_basics.gate_status_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_nsa_id, state=consts.ApiModuleState.active)
    # Verification
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        speed=True,
        can_warp=True,
        can_jump_gate=True,
        can_jump_wormhole=True,
        can_jump_drive=True,
        can_dock_station=True,
        can_dock_citadel=True,
        can_tether=True))
    api_ship.update()
    assert api_ship_stats.speed.one() == approx(100)
    assert api_ship_stats.can_warp.one() is False
    assert api_ship_stats.can_jump_gate.one() is True
    assert api_ship_stats.can_jump_wormhole.one() is True
    assert api_ship_stats.can_jump_drive.one() is False
    assert api_ship_stats.can_dock_station.one() is False
    assert api_ship_stats.can_dock_citadel.one() is False
    assert api_ship_stats.can_tether.one() is False
    # Action
    api_fit.add_module(type_id=eve_basics.cloak_t2_id, state=consts.ApiModuleState.active)
    # Verification
    assert api_fit.validate(options=ValOptions(cloaking_blocked=True)).passed is False
    # Action
    api_proj_fit = api_sol.create_fit()
    api_proj_fit.add_module(
        type_id=eve_basics.assist_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    api_proj_fit.add_module(
        type_id=eve_basics.offense_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    # Verification
    assert api_proj_fit.validate(options=ValOptions(assist_immunity=True)).passed is True
    assert api_proj_fit.validate(options=ValOptions(offense_immunity=True)).passed is True


def test_tactical_recloner(client, consts):
    """
    Tested on Singularity on 2026-06-15 and 2026-07-26, using Hel and tactical recloner.

    Prevented actions/interactions:
    + warp (external factors)
    - jump gate
    ? jump wormhole (ship size message is shown regardless)
    + jump drive (external factors)
    + dock station (external factors)
    + dock citadel (external factors)
    - tether
    + cloak (special but generic - one or more module is making this ship unable to cloak)
    - regular movement
    - incoming assistance
    - incoming offensive mods
    """
    eve_basics = setup_basics(client=client, consts=consts)
    eve_clone_bay_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.clone_respawn_bay,
        cat_id=consts.EveEffCat.active)
    eve_clone_bay_id = client.mk_eve_item(
        attrs={
            eve_basics.warp_scram_attr_id: 100, eve_basics.disallow_dock_attr_id: 1,
            eve_basics.can_cloak_attr_id: 0},
        eff_ids=[eve_clone_bay_effect_id],
        defeff_id=eve_clone_bay_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basics.speed_attr_id: 100, eve_basics.gate_status_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_clone_bay_id, state=consts.ApiModuleState.active)
    # Verification
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        speed=True,
        can_warp=True,
        can_jump_gate=True,
        can_jump_wormhole=True,
        can_jump_drive=True,
        can_dock_station=True,
        can_dock_citadel=True,
        can_tether=True))
    api_ship.update()
    assert api_ship_stats.speed.one() == approx(100)
    assert api_ship_stats.can_warp.one() is False
    assert api_ship_stats.can_jump_gate.one() is True
    assert api_ship_stats.can_jump_wormhole.one() is True
    assert api_ship_stats.can_jump_drive.one() is False
    assert api_ship_stats.can_dock_station.one() is False
    assert api_ship_stats.can_dock_citadel.one() is False
    assert api_ship_stats.can_tether.one() is True
    # Action
    api_fit.add_module(type_id=eve_basics.cloak_t2_id, state=consts.ApiModuleState.active)
    # Verification
    assert api_fit.validate(options=ValOptions(cloaking_blocked=True)).passed is False
    # Action
    api_proj_fit = api_sol.create_fit()
    api_proj_fit.add_module(
        type_id=eve_basics.assist_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    api_proj_fit.add_module(
        type_id=eve_basics.offense_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    # Verification
    assert api_proj_fit.validate(options=ValOptions(assist_immunity=True)).passed is True
    assert api_proj_fit.validate(options=ValOptions(offense_immunity=True)).passed is True


def test_clone_bay(client, consts):
    """
    Tested on Singularity on 2026-06-15 and 2026-07-26, using Rorqual and clone vat bay.

    Prevented actions/interactions:
    + warp (external factors, full duration)
    - jump gate
    - jump wormhole
    + jump drive (external factors, full duration)
    - dock station
    + dock citadel (external factors, full duration)
    - tether
    + cloak (special but generic - one or more module is making this ship unable to cloak)
    + regular movement
    - incoming assistance
    - incoming offensive mods
    """
    eve_basics = setup_basics(client=client, consts=consts)
    eve_speed_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_basics.speed_factor_attr_id,
        affectee_attr_id=eve_basics.speed_attr_id)
    eve_warp_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_basics.warp_scram_attr_id,
        affectee_attr_id=eve_basics.warp_status_attr_id)
    eve_cloak_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_basics.can_cloak_attr_id,
        affectee_attr_id=eve_basics.can_cloak_attr_id)
    eve_clone_bay_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.clone_jump_accepting,
        cat_id=consts.EveEffCat.active,
        mod_info=[eve_speed_mod, eve_warp_mod, eve_cloak_mod])
    eve_clone_bay_id = client.mk_eve_item(
        attrs={
            eve_basics.speed_factor_attr_id: -100, eve_basics.warp_scram_attr_id: 100,
            eve_basics.can_cloak_attr_id: 0},
        eff_ids=[eve_clone_bay_effect_id],
        defeff_id=eve_clone_bay_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basics.speed_attr_id: 100, eve_basics.gate_status_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_clone_bay_id, state=consts.ApiModuleState.active)
    # Verification
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        speed=True,
        can_warp=True,
        can_jump_gate=True,
        can_jump_wormhole=True,
        can_jump_drive=True,
        can_dock_station=True,
        can_dock_citadel=True,
        can_tether=True))
    api_ship.update()
    assert api_ship_stats.speed.one() == approx(0)
    assert api_ship_stats.can_warp.one() is False
    assert api_ship_stats.can_jump_gate.one() is True
    assert api_ship_stats.can_jump_wormhole.one() is True
    assert api_ship_stats.can_jump_drive.one() is False
    assert api_ship_stats.can_dock_station.one() is True
    assert api_ship_stats.can_dock_citadel.one() is False
    assert api_ship_stats.can_tether.one() is True
    # Action
    api_fit.add_module(type_id=eve_basics.cloak_t2_id, state=consts.ApiModuleState.active)
    # Verification
    assert api_fit.validate(options=ValOptions(cloaking_blocked=True)).passed is False
    # Action
    api_proj_fit = api_sol.create_fit()
    api_proj_fit.add_module(
        type_id=eve_basics.assist_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    api_proj_fit.add_module(
        type_id=eve_basics.offense_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    # Verification
    assert api_proj_fit.validate(options=ValOptions(assist_immunity=True)).passed is True
    assert api_proj_fit.validate(options=ValOptions(offense_immunity=True)).passed is True


def test_siege_dread(client, consts):
    """
    Tested on Singularity on 2026-06-15 and 2026-07-26, using Revelation with t2 siege module.

    Prevented actions/interactions:
    + warp (external factors)
    + jump gate (external factors)
    ? jump wormhole (ship size message is shown)
    + jump drive (external factors)
    + dock station (external factors)
    + dock citadel (external factors)
    + tether
    + cloak (special)
    +- regular movement (via dogma modifier, sarathiel reduces penalty to -90%)
    +- incoming assistance (stops reps/cap transfers due to resistances, lets RSBs/RTCs run)
    - incoming offensive mods
    - MJD (Sarathiel, not tested here)
    """
    eve_basics = setup_basics(client=client, consts=consts)
    # Siege has many modifiers, but only 5 of those are relevant
    eve_speed_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_basics.speed_factor_attr_id,
        affectee_attr_id=eve_basics.speed_attr_id)
    eve_warp_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_basics.warp_scram_attr_id,
        affectee_attr_id=eve_basics.warp_status_attr_id)
    eve_gate_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_basics.gate_scram_attr_id,
        affectee_attr_id=eve_basics.gate_status_attr_id)
    eve_dock_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_basics.disallow_dock_attr_id,
        affectee_attr_id=eve_basics.disallow_dock_attr_id)
    eve_tether_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_basics.disallow_tether_attr_id,
        affectee_attr_id=eve_basics.disallow_tether_attr_id)
    eve_siege_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_siege,
        cat_id=consts.EveEffCat.active,
        mod_info=[eve_speed_mod, eve_warp_mod, eve_gate_mod, eve_dock_mod, eve_tether_mod])
    eve_siege_id = client.mk_eve_item(
        attrs={
            eve_basics.speed_factor_attr_id: -100, eve_basics.warp_scram_attr_id: 100,
            eve_basics.disallow_dock_attr_id: 1, eve_basics.disallow_tether_attr_id: 1},
        eff_ids=[eve_siege_effect_id],
        defeff_id=eve_siege_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basics.speed_attr_id: 100, eve_basics.gate_status_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_siege_id, state=consts.ApiModuleState.active)
    # Verification
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        speed=True,
        can_warp=True,
        can_jump_gate=True,
        can_jump_wormhole=True,
        can_jump_drive=True,
        can_dock_station=True,
        can_dock_citadel=True,
        can_tether=True))
    api_ship.update()
    assert api_ship_stats.speed.one() == approx(0)
    assert api_ship_stats.can_warp.one() is False
    assert api_ship_stats.can_jump_gate.one() is False
    assert api_ship_stats.can_jump_wormhole.one() is True
    assert api_ship_stats.can_jump_drive.one() is False
    assert api_ship_stats.can_dock_station.one() is False
    assert api_ship_stats.can_dock_citadel.one() is False
    assert api_ship_stats.can_tether.one() is False
    # Action
    api_fit.add_module(type_id=eve_basics.cloak_t2_id, state=consts.ApiModuleState.active)
    # Verification
    assert api_fit.validate(options=ValOptions(cloaking_blocked=True)).passed is False
    # Action
    api_proj_fit = api_sol.create_fit()
    api_proj_fit.add_module(
        type_id=eve_basics.assist_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    api_proj_fit.add_module(
        type_id=eve_basics.offense_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    # Verification - assistance and some offense is prevented via specific resistances, which is not
    # tested here
    assert api_proj_fit.validate(options=ValOptions(assist_immunity=True)).passed is True
    assert api_proj_fit.validate(options=ValOptions(offense_immunity=True)).passed is True


def test_siege_fax(client, consts):
    """
    Tested on Singularity on 2026-06-15 and 2026-07-26, using Minokawa with t2 triage module.

    Prevented actions/interactions:
    + warp (external factors)
    + jump gate (external factors)
    ? jump wormhole (ship size message is shown)
    + jump drive (external factors)
    + dock station (external factors)
    + dock citadel (external factors)
    + tether
    + cloak (special)
    + regular movement
    +- incoming assistance (stops reps/cap transfers due to resistances, lets RSBs/RTCs run)
    - incoming offensive mods
    """
    eve_basics = setup_basics(client=client, consts=consts)
    # Triage has many modifiers, but only 5 of those are relevant
    eve_speed_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_basics.speed_factor_attr_id,
        affectee_attr_id=eve_basics.speed_attr_id)
    eve_warp_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_basics.warp_scram_attr_id,
        affectee_attr_id=eve_basics.warp_status_attr_id)
    eve_gate_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_basics.gate_scram_attr_id,
        affectee_attr_id=eve_basics.gate_status_attr_id)
    eve_dock_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_basics.disallow_dock_attr_id,
        affectee_attr_id=eve_basics.disallow_dock_attr_id)
    eve_tether_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_basics.disallow_tether_attr_id,
        affectee_attr_id=eve_basics.disallow_tether_attr_id)
    eve_triage_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_triage_mod,
        cat_id=consts.EveEffCat.active,
        mod_info=[eve_speed_mod, eve_warp_mod, eve_gate_mod, eve_dock_mod, eve_tether_mod])
    eve_triage_id = client.mk_eve_item(
        attrs={
            eve_basics.speed_factor_attr_id: -100, eve_basics.warp_scram_attr_id: 100,
            eve_basics.disallow_dock_attr_id: 1, eve_basics.disallow_tether_attr_id: 1},
        eff_ids=[eve_triage_effect_id],
        defeff_id=eve_triage_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basics.speed_attr_id: 100, eve_basics.gate_status_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_triage_id, state=consts.ApiModuleState.active)
    # Verification
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        speed=True,
        can_warp=True,
        can_jump_gate=True,
        can_jump_wormhole=True,
        can_jump_drive=True,
        can_dock_station=True,
        can_dock_citadel=True,
        can_tether=True))
    api_ship.update()
    assert api_ship_stats.speed.one() == approx(0)
    assert api_ship_stats.can_warp.one() is False
    assert api_ship_stats.can_jump_gate.one() is False
    assert api_ship_stats.can_jump_wormhole.one() is True
    assert api_ship_stats.can_jump_drive.one() is False
    assert api_ship_stats.can_dock_station.one() is False
    assert api_ship_stats.can_dock_citadel.one() is False
    assert api_ship_stats.can_tether.one() is False
    # Action
    api_fit.add_module(type_id=eve_basics.cloak_t2_id, state=consts.ApiModuleState.active)
    # Verification
    assert api_fit.validate(options=ValOptions(cloaking_blocked=True)).passed is False
    # Action
    api_proj_fit = api_sol.create_fit()
    api_proj_fit.add_module(
        type_id=eve_basics.assist_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    api_proj_fit.add_module(
        type_id=eve_basics.offense_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    # Verification - assistance and some offense is prevented via specific resistances, which is not
    # tested here
    assert api_proj_fit.validate(options=ValOptions(assist_immunity=True)).passed is True
    assert api_proj_fit.validate(options=ValOptions(offense_immunity=True)).passed is True


def test_siege_industrial_cap(client, consts):
    """
    Tested on Singularity on 2026-06-15 and 2026-07-26, using Rorqual with t1 industrial core.

    Prevented actions/interactions:
    + warp (external factors)
    + jump gate (external factors)
    ? jump wormhole (ship size message is shown regardless)
    + jump drive (external factors)
    + dock station (special rorq)
    + dock citadel (special rorq)
    + tether
    + cloak (special)
    + regular movement
    +- incoming assistance (stops reps/cap transfers due to resistances, lets RSBs/RTCs run)
    - incoming offensive mods
    """
    eve_basics = setup_basics(client=client, consts=consts)
    # Triage has many modifiers, but only 5 of those are relevant
    eve_speed_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_basics.speed_factor_attr_id,
        affectee_attr_id=eve_basics.speed_attr_id)
    eve_warp_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_basics.warp_scram_attr_id,
        affectee_attr_id=eve_basics.warp_status_attr_id)
    eve_gate_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_basics.gate_scram_attr_id,
        affectee_attr_id=eve_basics.gate_status_attr_id)
    eve_dock_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_basics.disallow_dock_attr_id,
        affectee_attr_id=eve_basics.disallow_dock_attr_id)
    eve_tether_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_basics.disallow_tether_attr_id,
        affectee_attr_id=eve_basics.disallow_tether_attr_id)
    eve_industrial_core_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.industrial_core_effect2,
        cat_id=consts.EveEffCat.active,
        mod_info=[eve_speed_mod, eve_warp_mod, eve_gate_mod, eve_dock_mod, eve_tether_mod])
    eve_industrial_core_id = client.mk_eve_item(
        attrs={
            eve_basics.speed_factor_attr_id: -100, eve_basics.warp_scram_attr_id: 100,
            eve_basics.disallow_dock_attr_id: 1, eve_basics.disallow_tether_attr_id: 1},
        eff_ids=[eve_industrial_core_effect_id],
        defeff_id=eve_industrial_core_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basics.speed_attr_id: 100, eve_basics.gate_status_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_industrial_core_id, state=consts.ApiModuleState.active)
    # Verification
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        speed=True,
        can_warp=True,
        can_jump_gate=True,
        can_jump_wormhole=True,
        can_jump_drive=True,
        can_dock_station=True,
        can_dock_citadel=True,
        can_tether=True))
    api_ship.update()
    assert api_ship_stats.speed.one() == approx(0)
    assert api_ship_stats.can_warp.one() is False
    assert api_ship_stats.can_jump_gate.one() is False
    assert api_ship_stats.can_jump_wormhole.one() is True
    assert api_ship_stats.can_jump_drive.one() is False
    assert api_ship_stats.can_dock_station.one() is False
    assert api_ship_stats.can_dock_citadel.one() is False
    assert api_ship_stats.can_tether.one() is False
    # Action
    api_fit.add_module(type_id=eve_basics.cloak_t2_id, state=consts.ApiModuleState.active)
    # Verification
    assert api_fit.validate(options=ValOptions(cloaking_blocked=True)).passed is False
    # Action
    api_proj_fit = api_sol.create_fit()
    api_proj_fit.add_module(
        type_id=eve_basics.assist_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    api_proj_fit.add_module(
        type_id=eve_basics.offense_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    # Verification - assistance and some offense is prevented via specific resistances, which is not
    # tested here
    assert api_proj_fit.validate(options=ValOptions(assist_immunity=True)).passed is True
    assert api_proj_fit.validate(options=ValOptions(offense_immunity=True)).passed is True


def test_siege_industrial_subcap(client, consts):
    """
    Tested on Singularity on 2026-06-15 and 2026-07-26, using Orca and Porpoise with t1 industrial
    cores.

    Prevented actions/interactions:
    + warp (external factors)
    + jump gate (custom - gate scramble does not work because of lower gate scram status on subcaps)
    + jump wormhole (special)
    + jump drive (external factors)
    + dock station (did not record which message)
    + dock citadel (did not record which message)
    + tether
    + cloak (special)
    + regular movement
    +- incoming assistance (stops reps/cap transfers due to resistances, lets RSBs/RTCs run)
    - incoming offensive mods
    """
    eve_basics = setup_basics(client=client, consts=consts)
    # Triage has many modifiers, but only 5 of those are relevant
    eve_speed_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_basics.speed_factor_attr_id,
        affectee_attr_id=eve_basics.speed_attr_id)
    eve_warp_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_basics.warp_scram_attr_id,
        affectee_attr_id=eve_basics.warp_status_attr_id)
    eve_gate_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_basics.gate_scram_attr_id,
        affectee_attr_id=eve_basics.gate_status_attr_id)
    eve_dock_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_basics.disallow_dock_attr_id,
        affectee_attr_id=eve_basics.disallow_dock_attr_id)
    eve_tether_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_basics.disallow_tether_attr_id,
        affectee_attr_id=eve_basics.disallow_tether_attr_id)
    eve_industrial_core_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.industrial_compact_core_effect2,
        cat_id=consts.EveEffCat.active,
        mod_info=[eve_speed_mod, eve_warp_mod, eve_gate_mod, eve_dock_mod, eve_tether_mod])
    eve_industrial_core_id = client.mk_eve_item(
        attrs={
            eve_basics.speed_factor_attr_id: -100, eve_basics.warp_scram_attr_id: 100,
            eve_basics.disallow_dock_attr_id: 1, eve_basics.disallow_tether_attr_id: 1},
        eff_ids=[eve_industrial_core_effect_id],
        defeff_id=eve_industrial_core_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basics.speed_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_industrial_core_id, state=consts.ApiModuleState.active)
    # Verification
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        speed=True,
        can_warp=True,
        can_jump_gate=True,
        can_jump_wormhole=True,
        can_jump_drive=True,
        can_dock_station=True,
        can_dock_citadel=True,
        can_tether=True))
    api_ship.update()
    assert api_ship_stats.speed.one() == approx(0)
    assert api_ship_stats.can_warp.one() is False
    assert api_ship_stats.can_jump_gate.one() is False
    assert api_ship_stats.can_jump_wormhole.one() is False
    assert api_ship_stats.can_jump_drive.one() is False
    assert api_ship_stats.can_dock_station.one() is False
    assert api_ship_stats.can_dock_citadel.one() is False
    assert api_ship_stats.can_tether.one() is False
    # Action
    api_fit.add_module(type_id=eve_basics.cloak_t2_id, state=consts.ApiModuleState.active)
    # Verification
    assert api_fit.validate(options=ValOptions(cloaking_blocked=True)).passed is False
    # Action
    api_proj_fit = api_sol.create_fit()
    api_proj_fit.add_module(
        type_id=eve_basics.assist_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    api_proj_fit.add_module(
        type_id=eve_basics.offense_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    # Verification - assistance and some offense is prevented via specific resistances, which is not
    # tested here
    assert api_proj_fit.validate(options=ValOptions(assist_immunity=True)).passed is True
    assert api_proj_fit.validate(options=ValOptions(offense_immunity=True)).passed is True


def test_siege_bastion(client, consts):
    """
    Tested on Singularity on 2026-06-15 and 2026-07-26, using Paladin with t1 bastion module.

    Prevented actions/interactions:
    + warp (external factors)
    + jump gate (weapons timer, duration + 1m)
    - jump wormhole
    ? jump drive (no ships with bastion + jump drive, but warp scramble status would disallow it)
    + dock station (external factors)
    + dock citadel (external factors)
    + tether
    + cloak (special)
    + regular movement  +- incoming assistance (stops reps/cap transfers due to resistances, lets RSBs/RTCs run)
    - incoming offensive mods
    + MJD (not tested here, likely blocked by 0 max speed)
    """
    eve_basics = setup_basics(client=client, consts=consts)
    # Siege has many modifiers, but only 4 of those are relevant
    eve_speed_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_basics.speed_factor_attr_id,
        affectee_attr_id=eve_basics.speed_attr_id)
    eve_warp_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_basics.warp_scram_attr_id,
        affectee_attr_id=eve_basics.warp_status_attr_id)
    eve_dock_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_basics.disallow_dock_attr_id,
        affectee_attr_id=eve_basics.disallow_dock_attr_id)
    eve_tether_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_basics.disallow_tether_attr_id,
        affectee_attr_id=eve_basics.disallow_tether_attr_id)
    eve_bastion_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_siege,
        cat_id=consts.EveEffCat.active,
        is_offensive=True,
        mod_info=[eve_speed_mod, eve_warp_mod, eve_dock_mod, eve_tether_mod])
    eve_bastion_id = client.mk_eve_item(
        attrs={
            eve_basics.speed_factor_attr_id: -100, eve_basics.warp_scram_attr_id: 100,
            eve_basics.disallow_dock_attr_id: 1, eve_basics.disallow_tether_attr_id: 1},
        eff_ids=[eve_bastion_effect_id],
        defeff_id=eve_bastion_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basics.speed_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_bastion_id, state=consts.ApiModuleState.active)
    # Verification
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        speed=True,
        can_warp=True,
        can_jump_gate=True,
        can_jump_wormhole=True,
        can_jump_drive=True,
        can_dock_station=True,
        can_dock_citadel=True,
        can_tether=True))
    api_ship.update()
    assert api_ship_stats.speed.one() == approx(0)
    assert api_ship_stats.can_warp.one() is False
    assert api_ship_stats.can_jump_gate.one() is False
    assert api_ship_stats.can_jump_wormhole.one() is True
    assert api_ship_stats.can_jump_drive.one() is False
    assert api_ship_stats.can_dock_station.one() is False
    assert api_ship_stats.can_dock_citadel.one() is False
    assert api_ship_stats.can_tether.one() is False
    # Action
    api_fit.add_module(type_id=eve_basics.cloak_t2_id, state=consts.ApiModuleState.active)
    # Verification
    assert api_fit.validate(options=ValOptions(cloaking_blocked=True)).passed is False
    # Action
    api_proj_fit = api_sol.create_fit()
    api_proj_fit.add_module(
        type_id=eve_basics.assist_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    api_proj_fit.add_module(
        type_id=eve_basics.offense_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    # Verification - assistance and some offense is prevented via specific resistances, which is not
    # tested here
    assert api_proj_fit.validate(options=ValOptions(assist_immunity=True)).passed is True
    assert api_proj_fit.validate(options=ValOptions(offense_immunity=True)).passed is True


def test_cloak_t1(client, consts):
    """
    Not tested, assumed to be the same as t2.
    """
    eve_basics = setup_basics(client=client, consts=consts)
    eve_cloak_effect_id = client.mk_eve_effect(id_=consts.EveEffect.cloaking_prototype, cat_id=consts.EveEffCat.active)
    eve_cloak_id = client.mk_eve_item(eff_ids=[eve_cloak_effect_id], defeff_id=eve_cloak_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basics.speed_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_cloak_id, state=consts.ApiModuleState.active)
    # Verification
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        speed=True,
        can_warp=True,
        can_jump_gate=True,
        can_jump_wormhole=True,
        can_jump_drive=True,
        can_dock_station=True,
        can_dock_citadel=True,
        can_tether=True))
    api_ship.update()
    assert api_ship_stats.speed.one() == approx(100)
    assert api_ship_stats.can_warp.one() is False
    assert api_ship_stats.can_jump_gate.one() is False
    assert api_ship_stats.can_jump_wormhole.one() is False
    assert api_ship_stats.can_jump_drive.one() is False
    assert api_ship_stats.can_dock_station.one() is False
    assert api_ship_stats.can_dock_citadel.one() is False
    assert api_ship_stats.can_tether.one() is False
    # Action
    api_proj_fit = api_sol.create_fit()
    api_proj_fit.add_module(
        type_id=eve_basics.assist_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    api_proj_fit.add_module(
        type_id=eve_basics.offense_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    # Verification
    assert api_proj_fit.validate(options=ValOptions(assist_immunity=True)).passed is True
    assert api_proj_fit.validate(options=ValOptions(offense_immunity=True)).passed is True


def test_cloak_t2(client, consts):
    """
    Tested on Singularity on 2026-06-15, using Panther with t2 non-covops cloak.

    Prevented actions/interactions:
    + warp
    + jump gate
    + jump wormhole
    + jump drive
    + dock station
    + dock citadel
    + tether
    - regular movement
    ? disallow assistance (can't target)
    ? disallow offensive mods (can't target)
    """
    eve_basics = setup_basics(client=client, consts=consts)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basics.speed_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_basics.cloak_t2_id, state=consts.ApiModuleState.active)
    # Verification
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        speed=True,
        can_warp=True,
        can_jump_gate=True,
        can_jump_wormhole=True,
        can_jump_drive=True,
        can_dock_station=True,
        can_dock_citadel=True,
        can_tether=True))
    api_ship.update()
    assert api_ship_stats.speed.one() == approx(100)
    assert api_ship_stats.can_warp.one() is False
    assert api_ship_stats.can_jump_gate.one() is False
    assert api_ship_stats.can_jump_wormhole.one() is False
    assert api_ship_stats.can_jump_drive.one() is False
    assert api_ship_stats.can_dock_station.one() is False
    assert api_ship_stats.can_dock_citadel.one() is False
    assert api_ship_stats.can_tether.one() is False
    # Action
    api_proj_fit = api_sol.create_fit()
    api_proj_fit.add_module(
        type_id=eve_basics.assist_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    api_proj_fit.add_module(
        type_id=eve_basics.offense_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    # Verification
    assert api_proj_fit.validate(options=ValOptions(assist_immunity=True)).passed is True
    assert api_proj_fit.validate(options=ValOptions(offense_immunity=True)).passed is True


def test_cloak_covops(client, consts):
    """
    Tested on Singularity on 2026-06-15, using Purifier with t2 covops cloak.

    Prevented actions/interactions:
    + warp
    + jump gate
    + jump wormhole
    ? jump drive (no covops-capable ships with jump drive, but likely blocked like on non-covops)
    + dock station
    + dock citadel
    + tether
    - regular movement
    ? disallow assistance (can't target)
    ? disallow offensive mods (can't target)
    """
    eve_basics = setup_basics(client=client, consts=consts)
    eve_cloak_effect_id = client.mk_eve_effect(id_=consts.EveEffect.cloaking_warp_safe, cat_id=consts.EveEffCat.active)
    eve_cloak_id = client.mk_eve_item(eff_ids=[eve_cloak_effect_id], defeff_id=eve_cloak_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basics.speed_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_cloak_id, state=consts.ApiModuleState.active)
    # Verification
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        speed=True,
        can_warp=True,
        can_jump_gate=True,
        can_jump_wormhole=True,
        can_jump_drive=True,
        can_dock_station=True,
        can_dock_citadel=True,
        can_tether=True))
    api_ship.update()
    assert api_ship_stats.speed.one() == approx(100)
    assert api_ship_stats.can_warp.one() is True
    assert api_ship_stats.can_jump_gate.one() is False
    assert api_ship_stats.can_jump_wormhole.one() is False
    assert api_ship_stats.can_jump_drive.one() is False
    assert api_ship_stats.can_dock_station.one() is False
    assert api_ship_stats.can_dock_citadel.one() is False
    assert api_ship_stats.can_tether.one() is False
    # Action
    api_proj_fit = api_sol.create_fit()
    api_proj_fit.add_module(
        type_id=eve_basics.assist_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    api_proj_fit.add_module(
        type_id=eve_basics.offense_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    # Verification
    assert api_proj_fit.validate(options=ValOptions(assist_immunity=True)).passed is True
    assert api_proj_fit.validate(options=ValOptions(offense_immunity=True)).passed is True


def test_cyno(client, consts):
    """
    Tested on Singularity on 2026-06-15 and 2026-07-26, using Panther and covops cyno.

    Prevented actions/interactions:
    + warp (external factors)
    + jump gate (special, about jump drive)
    + jump wormhole (special)
    + jump drive (external factors)
    + dock station (special)
    + dock citadel (special)
    + tether
    + cloak (special but generic - one or more module is making this ship unable to cloak)
    + regular movement
    - incoming assistance
    - incoming offensive mods
    + MJD (not tested here, likely blocked by 0 max speed)
    """
    eve_basics = setup_basics(client=client, consts=consts)
    eve_speed_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_basics.speed_factor_attr_id,
        affectee_attr_id=eve_basics.speed_attr_id)
    eve_warp_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_basics.warp_scram_attr_id,
        affectee_attr_id=eve_basics.warp_status_attr_id)
    eve_cloak_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_basics.can_cloak_attr_id,
        affectee_attr_id=eve_basics.can_cloak_attr_id)
    eve_cyno_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.cynosural_generation,
        cat_id=consts.EveEffCat.active,
        mod_info=[eve_speed_mod, eve_warp_mod, eve_cloak_mod])
    eve_cyno_id = client.mk_eve_item(
        attrs={
            eve_basics.speed_factor_attr_id: -100, eve_basics.warp_scram_attr_id: 100,
            eve_basics.disallow_dock_attr_id: 1, eve_basics.disallow_tether_attr_id: 1,
            eve_basics.can_cloak_attr_id: 0},
        eff_ids=[eve_cyno_effect_id],
        defeff_id=eve_cyno_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basics.speed_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_cyno_id, state=consts.ApiModuleState.active)
    # Verification
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        speed=True,
        can_warp=True,
        can_jump_gate=True,
        can_jump_wormhole=True,
        can_jump_drive=True,
        can_dock_station=True,
        can_dock_citadel=True,
        can_tether=True))
    api_ship.update()
    assert api_ship_stats.speed.one() == approx(0)
    assert api_ship_stats.can_warp.one() is False
    assert api_ship_stats.can_jump_gate.one() is False
    assert api_ship_stats.can_jump_wormhole.one() is False
    assert api_ship_stats.can_jump_drive.one() is False
    assert api_ship_stats.can_dock_station.one() is False
    assert api_ship_stats.can_dock_citadel.one() is False
    assert api_ship_stats.can_tether.one() is False
    # Action
    api_fit.add_module(type_id=eve_basics.cloak_t2_id, state=consts.ApiModuleState.active)
    # Verification
    assert api_fit.validate(options=ValOptions(cloaking_blocked=True)).passed is False
    # Action
    api_proj_fit = api_sol.create_fit()
    api_proj_fit.add_module(
        type_id=eve_basics.assist_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    api_proj_fit.add_module(
        type_id=eve_basics.offense_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    # Verification
    assert api_proj_fit.validate(options=ValOptions(assist_immunity=True)).passed is True
    assert api_proj_fit.validate(options=ValOptions(offense_immunity=True)).passed is True


def test_mjd(client, consts):
    """
    Tested on Tranquility sometime in May 2026, using Occator and t1 medium MJD, and on Singularity
    on 2026-06-15 using Nidhoggur and t1 capital MJD.

    Prevented actions/interactions:
    + warp
    + jump gate
    + jump wormhole
    + jump drive
    + dock station
    + dock citadel
    - tether
    + cloak
    + regular movement
    - incoming assistance
    - incoming offensive mods
    """
    eve_basics = setup_basics(client=client, consts=consts)
    eve_mjd_effect_id = client.mk_eve_effect(id_=consts.EveEffect.micro_jump_drive, cat_id=consts.EveEffCat.active)
    eve_mjd_id = client.mk_eve_item(eff_ids=[eve_mjd_effect_id], defeff_id=eve_mjd_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basics.speed_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_mjd_id, state=consts.ApiModuleState.active)
    # Verification
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        speed=True,
        can_warp=True,
        can_jump_gate=True,
        can_jump_wormhole=True,
        can_jump_drive=True,
        can_dock_station=True,
        can_dock_citadel=True,
        can_tether=True))
    api_ship.update()
    assert api_ship_stats.speed.one() == approx(100)
    assert api_ship_stats.can_warp.one() is False
    assert api_ship_stats.can_jump_gate.one() is False
    assert api_ship_stats.can_jump_wormhole.one() is False
    assert api_ship_stats.can_jump_drive.one() is False
    assert api_ship_stats.can_dock_station.one() is False
    assert api_ship_stats.can_dock_citadel.one() is False
    assert api_ship_stats.can_tether.one() is True
    # Action
    api_fit.add_module(type_id=eve_basics.cloak_t2_id, state=consts.ApiModuleState.active)
    # Verification
    assert api_fit.validate(options=ValOptions(cloaking_blocked=True)).passed is False
    # Action
    api_proj_fit = api_sol.create_fit()
    api_proj_fit.add_module(
        type_id=eve_basics.assist_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    api_proj_fit.add_module(
        type_id=eve_basics.offense_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    # Verification
    assert api_proj_fit.validate(options=ValOptions(assist_immunity=True)).passed is True
    assert api_proj_fit.validate(options=ValOptions(offense_immunity=True)).passed is True


def test_mjfg_cap(client, consts):
    """
    Tested on Singularity on 2026-06-15 using Nidhoggur and t1 capital MJFG.

    Prevented actions/interactions:
    + warp
    + jump gate
    + jump wormhole
    + jump drive
    + dock station
    + dock citadel
    + tether
    + cloak
    + regular movement
    - incoming assistance
    - incoming offensive mods
    """
    eve_basics = setup_basics(client=client, consts=consts)
    eve_mjfg_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.micro_jump_portal_drive_capital,
        cat_id=consts.EveEffCat.active,
        is_offensive=True)
    eve_mjfg_id = client.mk_eve_item(eff_ids=[eve_mjfg_effect_id], defeff_id=eve_mjfg_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basics.speed_attr_id: 100, eve_basics.gate_status_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_mjfg_id, state=consts.ApiModuleState.active)
    # Verification
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        speed=True,
        can_warp=True,
        can_jump_gate=True,
        can_jump_wormhole=True,
        can_jump_drive=True,
        can_dock_station=True,
        can_dock_citadel=True,
        can_tether=True))
    api_ship.update()
    assert api_ship_stats.speed.one() == approx(100)
    assert api_ship_stats.can_warp.one() is False
    assert api_ship_stats.can_jump_gate.one() is False
    assert api_ship_stats.can_jump_wormhole.one() is False
    assert api_ship_stats.can_jump_drive.one() is False
    assert api_ship_stats.can_dock_station.one() is False
    assert api_ship_stats.can_dock_citadel.one() is False
    assert api_ship_stats.can_tether.one() is False
    # Action
    api_fit.add_module(type_id=eve_basics.cloak_t2_id, state=consts.ApiModuleState.active)
    # Verification
    assert api_fit.validate(options=ValOptions(cloaking_blocked=True)).passed is False
    # Action
    api_proj_fit = api_sol.create_fit()
    api_proj_fit.add_module(
        type_id=eve_basics.assist_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    api_proj_fit.add_module(
        type_id=eve_basics.offense_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    # Verification
    assert api_proj_fit.validate(options=ValOptions(assist_immunity=True)).passed is True
    assert api_proj_fit.validate(options=ValOptions(offense_immunity=True)).passed is True


def test_mjfg_subcap(client, consts):
    """
    Tested on Singularity on 2026-06-15 using Bifrost and t1 subcap MJFG.

    Prevented actions/interactions:
    + warp
    + jump gate
    + jump wormhole
    + jump drive
    + dock station
    + dock citadel
    + tether
    + cloak
    + regular movement
    - incoming assistance
    - incoming offensive mods
    """
    eve_basics = setup_basics(client=client, consts=consts)
    eve_mjfg_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.micro_jump_portal_drive,
        cat_id=consts.EveEffCat.active,
        is_offensive=True)
    eve_mjfg_id = client.mk_eve_item(eff_ids=[eve_mjfg_effect_id], defeff_id=eve_mjfg_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basics.speed_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_mjfg_id, state=consts.ApiModuleState.active)
    # Verification
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        speed=True,
        can_warp=True,
        can_jump_gate=True,
        can_jump_wormhole=True,
        can_jump_drive=True,
        can_dock_station=True,
        can_dock_citadel=True,
        can_tether=True))
    api_ship.update()
    assert api_ship_stats.speed.one() == approx(100)
    assert api_ship_stats.can_warp.one() is False
    assert api_ship_stats.can_jump_gate.one() is False
    assert api_ship_stats.can_jump_wormhole.one() is False
    assert api_ship_stats.can_jump_drive.one() is False
    assert api_ship_stats.can_dock_station.one() is False
    assert api_ship_stats.can_dock_citadel.one() is False
    assert api_ship_stats.can_tether.one() is False
    # Action
    api_fit.add_module(type_id=eve_basics.cloak_t2_id, state=consts.ApiModuleState.active)
    # Verification
    assert api_fit.validate(options=ValOptions(cloaking_blocked=True)).passed is False
    # Action
    api_proj_fit = api_sol.create_fit()
    api_proj_fit.add_module(
        type_id=eve_basics.assist_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    api_proj_fit.add_module(
        type_id=eve_basics.offense_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_ship.id])
    # Verification
    assert api_proj_fit.validate(options=ValOptions(assist_immunity=True)).passed is True
    assert api_proj_fit.validate(options=ValOptions(offense_immunity=True)).passed is True
