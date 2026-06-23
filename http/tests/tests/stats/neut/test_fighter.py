from fw import approx
from fw.api import (
    FitStatsOptions,
    FleetStatsOptions,
    ItemStatsOptions,
    StatNeutItemKinds,
    StatsOptionFitOutNps,
    StatsOptionItemOutNps,
    StatTimeBurst,
    StatTimeSim,
)


def test_state(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_energy_neut_amount)
    eve_max_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_energy_neut,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_abil_id = client.mk_eve_abil(id_=consts.EveAbil.energy_neut)
    eve_fighter_id = client.mk_eve_fighter(
        attrs={eve_neut_amount_attr_id: 99, eve_cycle_time_attr_id: 6000, eve_max_count_attr_id: 3},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id,
        abils=[client.mk_eve_item_abil(id_=eve_abil_id)])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification - fighter is engaging, ability is on - neut is always included in stats
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_nps=True))
    assert api_fleet_stats.outgoing_nps.one() == approx(49.5)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_nps=True))
    assert api_fit_stats.outgoing_nps.one() == approx(49.5)
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(outgoing_nps=True))
    assert api_fighter_stats.outgoing_nps.one() == approx(49.5)
    # Action
    api_fighter.change_fighter(abilities={eve_abil_id: False})
    # Verification - fighter is engaging, ability is off - neut is never included in stats
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_nps=True))
    assert api_fleet_stats.outgoing_nps.one() == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_nps=True))
    assert api_fit_stats.outgoing_nps.one() == 0
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(outgoing_nps=(True, [
        StatsOptionItemOutNps(ignore_state=False),
        StatsOptionItemOutNps(ignore_state=True)])))
    assert api_fighter_stats.outgoing_nps == [0, 0]
    # Action
    api_fighter.change_fighter(state=consts.ApiMinionState.in_space)
    # Verification - fighter is not engaging, ability is off - neut is never included in stats
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_nps=True))
    assert api_fleet_stats.outgoing_nps.one() == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_nps=True))
    assert api_fit_stats.outgoing_nps.one() == 0
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(outgoing_nps=(True, [
        StatsOptionItemOutNps(ignore_state=False),
        StatsOptionItemOutNps(ignore_state=True)])))
    assert api_fighter_stats.outgoing_nps == [0, 0]
    # Action
    api_fighter.change_fighter(abilities={eve_abil_id: True})
    # Verification - fighter is not engaging, ability is on - neut is included in stats only if
    # state was requested to be ignored
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_nps=True))
    assert api_fleet_stats.outgoing_nps.one() == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_nps=True))
    assert api_fit_stats.outgoing_nps.one() == 0
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(outgoing_nps=(True, [
        StatsOptionItemOutNps(ignore_state=False),
        StatsOptionItemOutNps(ignore_state=True)])))
    assert api_fighter_stats.outgoing_nps == [0, approx(49.5)]
    # Action
    api_fighter.change_fighter(state=consts.ApiMinionState.engaging)
    # Verification - fighter is engaging, ability is on - neut is always included in stats
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_nps=True))
    assert api_fleet_stats.outgoing_nps.one() == approx(49.5)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_nps=True))
    assert api_fit_stats.outgoing_nps.one() == approx(49.5)
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(outgoing_nps=True))
    assert api_fighter_stats.outgoing_nps.one() == approx(49.5)


def test_time(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_energy_neut_amount)
    eve_max_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_energy_neut,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_abil_id = client.mk_eve_abil(id_=consts.EveAbil.energy_neut)
    eve_fighter_id = client.mk_eve_fighter(
        attrs={eve_neut_amount_attr_id: 99, eve_cycle_time_attr_id: 6000, eve_max_count_attr_id: 3},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id,
        abils=[client.mk_eve_item_abil(id_=eve_abil_id)])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification - burst stats (first cycle)
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(time_options=StatTimeBurst())])))
    assert api_fleet_stats.outgoing_nps.one() == approx(49.5)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(time_options=StatTimeBurst())])))
    assert api_fit_stats.outgoing_nps.one() == approx(49.5)
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(time_options=StatTimeBurst())])))
    assert api_fighter_stats.outgoing_nps.one() == approx(49.5)
    # Sim stats without time - loop stats are exposed
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(time_options=StatTimeSim(time=None))])))
    assert api_fleet_stats.outgoing_nps.one() == approx(49.5)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(time_options=StatTimeSim(time=None))])))
    assert api_fit_stats.outgoing_nps.one() == approx(49.5)
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(time_options=StatTimeSim(time=None))])))
    assert api_fighter_stats.outgoing_nps.one() == approx(49.5)
    # Sim with time 1 second after first cycle has started
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(time_options=StatTimeSim(time=1))])))
    assert api_fleet_stats.outgoing_nps.one() == approx(297)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(time_options=StatTimeSim(time=1))])))
    assert api_fit_stats.outgoing_nps.one() == approx(297)
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(time_options=StatTimeSim(time=1))])))
    assert api_fighter_stats.outgoing_nps.one() == approx(297)
    # Sim with time when 1st cycle is about to complete
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(time_options=StatTimeSim(time=5))])))
    assert api_fleet_stats.outgoing_nps.one() == approx(59.4)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(time_options=StatTimeSim(time=5))])))
    assert api_fit_stats.outgoing_nps.one() == approx(59.4)
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(time_options=StatTimeSim(time=5))])))
    assert api_fighter_stats.outgoing_nps.one() == approx(59.4)
    # Sim with time when 2nd cycle has just started
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(time_options=StatTimeSim(time=7))])))
    assert api_fleet_stats.outgoing_nps.one() == approx(84.857143)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(time_options=StatTimeSim(time=7))])))
    assert api_fit_stats.outgoing_nps.one() == approx(84.857143)
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(time_options=StatTimeSim(time=7))])))
    assert api_fighter_stats.outgoing_nps.one() == approx(84.857143)


def test_range_and_cap_limit(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_energy_neut_amount)
    eve_max_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_cap_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_energy_neut,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id)
    eve_abil_id = client.mk_eve_abil(id_=consts.EveAbil.energy_neut)
    eve_fighter_id = client.mk_eve_fighter(
        attrs={
            eve_neut_amount_attr_id: 99, eve_cycle_time_attr_id: 6000,
            eve_optimal_attr_id: 12500, eve_falloff_attr_id: 5000,
            eve_max_count_attr_id: 3, eve_radius_attr_id: 35},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id,
        abils=[client.mk_eve_item_abil(id_=eve_abil_id)])
    eve_tgt_ship1_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 220, eve_cap_attr_id: 1000})
    eve_tgt_ship2_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 220, eve_cap_attr_id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(
        type_id=eve_fighter_id, state=consts.ApiMinionState.engaging, coordinates=(0, 0, 0))
    api_src_fighter_nonproj = api_src_fit.add_fighter(
        type_id=eve_fighter_id, state=consts.ApiMinionState.engaging, coordinates=(0, 0, 0))
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship1_id, coordinates=(0, 12755, 0))
    api_fleet = api_sol.create_fleet(fit_ids=[api_src_fit.id])
    api_src_fighter_proj.change_fighter(add_projs=[api_tgt_ship.id])
    # Verification - target has high enough cap pool, so full strength is exposed
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.outgoing_nps.one() == approx(99)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.outgoing_nps.one() == approx(99)
    api_src_fighter_proj_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fighter_proj_stats.outgoing_nps.one() == approx(49.5)
    api_src_fighter_nonproj_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fighter_nonproj_stats.outgoing_nps.one() == approx(49.5)
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship2_id)
    # Verification - cap pool is lower than neut amount, so strength is reduced
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.outgoing_nps.one() == approx(66.666667)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.outgoing_nps.one() == approx(66.666667)
    api_src_fighter_proj_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fighter_proj_stats.outgoing_nps.one() == approx(33.333333)
    api_src_fighter_nonproj_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fighter_nonproj_stats.outgoing_nps.one() == approx(33.333333)
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 17755, 0))
    # Verification - target now has high enough cap pool, considering amount is reduced by range
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.outgoing_nps.one() == approx(49.5)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.outgoing_nps.one() == approx(49.5)
    api_src_fighter_proj_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fighter_proj_stats.outgoing_nps.one() == approx(24.75)
    api_src_fighter_nonproj_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fighter_nonproj_stats.outgoing_nps.one() == approx(24.75)
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 22755, 0))
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.outgoing_nps.one() == approx(6.1875)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.outgoing_nps.one() == approx(6.1875)
    api_src_fighter_proj_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fighter_proj_stats.outgoing_nps.one() == approx(3.09375)
    api_src_fighter_nonproj_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fighter_nonproj_stats.outgoing_nps.one() == approx(3.09375)
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 27754, 0))
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.outgoing_nps.one() == approx(0.1935203)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.outgoing_nps.one() == approx(0.1935203)
    api_src_fighter_proj_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fighter_proj_stats.outgoing_nps.one() == approx(0.09676013)
    api_src_fighter_nonproj_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fighter_nonproj_stats.outgoing_nps.one() == approx(0.09676013)
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 27756, 0))
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.outgoing_nps.one() == 0
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.outgoing_nps.one() == 0
    api_src_fighter_proj_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fighter_proj_stats.outgoing_nps.one() == 0
    api_src_fighter_nonproj_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fighter_nonproj_stats.outgoing_nps.one() == 0


def test_resist_and_cap_limit(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_energy_neut_amount)
    eve_max_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_cap_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_resist_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_warfare_resist)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_energy_neut,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id,
        resist_attr_id=eve_resist_attr_id)
    eve_abil_id = client.mk_eve_abil(id_=consts.EveAbil.energy_neut)
    eve_fighter_id = client.mk_eve_fighter(
        attrs={eve_neut_amount_attr_id: 99, eve_cycle_time_attr_id: 6000, eve_max_count_attr_id: 3},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id,
        abils=[client.mk_eve_item_abil(id_=eve_abil_id)])
    eve_tgt_ship1_id = client.mk_eve_ship(attrs={eve_resist_attr_id: 0.8, eve_cap_attr_id: 150})
    eve_tgt_ship2_id = client.mk_eve_ship(attrs={eve_resist_attr_id: 0.4, eve_cap_attr_id: 150})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_src_fighter_nonproj = api_src_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship1_id)
    api_fleet = api_sol.create_fleet(fit_ids=[api_src_fit.id])
    api_src_fighter_proj.change_fighter(add_projs=[api_tgt_ship.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.outgoing_nps.one() == approx(50)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.outgoing_nps.one() == approx(50)
    api_src_fighter_proj_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fighter_proj_stats.outgoing_nps.one() == approx(25)
    api_src_fighter_nonproj_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fighter_nonproj_stats.outgoing_nps.one() == approx(25)
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship2_id)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.outgoing_nps.one() == approx(39.6)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.outgoing_nps.one() == approx(39.6)
    api_src_fighter_proj_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fighter_proj_stats.outgoing_nps.one() == approx(19.8)
    api_src_fighter_nonproj_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fighter_nonproj_stats.outgoing_nps.one() == approx(19.8)


def test_item_kind(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_energy_neut_amount)
    eve_max_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_energy_neut,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_abil_id = client.mk_eve_abil(id_=consts.EveAbil.energy_neut)
    eve_fighter_id = client.mk_eve_fighter(
        attrs={eve_neut_amount_attr_id: 99, eve_cycle_time_attr_id: 6000, eve_max_count_attr_id: 3},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id,
        abils=[client.mk_eve_item_abil(id_=eve_abil_id)])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_nps=(True, [
        StatsOptionFitOutNps(),
        StatsOptionFitOutNps(item_kinds=StatNeutItemKinds(default=False, minion=True)),
        StatsOptionFitOutNps(item_kinds=StatNeutItemKinds(default=True, minion=False))])))
    assert api_fleet_stats.outgoing_nps == [approx(49.5), approx(49.5), 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_nps=(True, [
        StatsOptionFitOutNps(),
        StatsOptionFitOutNps(item_kinds=StatNeutItemKinds(default=False, minion=True)),
        StatsOptionFitOutNps(item_kinds=StatNeutItemKinds(default=True, minion=False))])))
    assert api_fit_stats.outgoing_nps == [approx(49.5), approx(49.5), 0]


def test_count_override(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_energy_neut_amount)
    eve_max_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_energy_neut,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_abil_id = client.mk_eve_abil(id_=consts.EveAbil.energy_neut)
    eve_fighter_id = client.mk_eve_fighter(
        attrs={eve_neut_amount_attr_id: 99, eve_cycle_time_attr_id: 6000, eve_max_count_attr_id: 3},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id,
        abils=[client.mk_eve_item_abil(id_=eve_abil_id)])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging, count=2)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_nps=True))
    assert api_fleet_stats.outgoing_nps.one() == approx(33)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_nps=True))
    assert api_fit_stats.outgoing_nps.one() == approx(33)
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(outgoing_nps=True))
    assert api_fighter_stats.outgoing_nps.one() == approx(33)
    # Action
    api_fighter.change_fighter(count=4)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_nps=True))
    assert api_fleet_stats.outgoing_nps.one() == approx(66)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_nps=True))
    assert api_fit_stats.outgoing_nps.one() == approx(66)
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(outgoing_nps=True))
    assert api_fighter_stats.outgoing_nps.one() == approx(66)


def test_zero_cycle_time(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_energy_neut_amount)
    eve_max_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_energy_neut,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_abil_id = client.mk_eve_abil(id_=consts.EveAbil.energy_neut)
    eve_fighter_id = client.mk_eve_fighter(
        attrs={eve_neut_amount_attr_id: 99, eve_cycle_time_attr_id: 0, eve_max_count_attr_id: 3},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id,
        abils=[client.mk_eve_item_abil(id_=eve_abil_id)])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_nps=True))
    assert api_fleet_stats.outgoing_nps.one() == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_nps=True))
    assert api_fit_stats.outgoing_nps.one() == 0
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(outgoing_nps=True))
    assert api_fighter_stats.outgoing_nps.one() == 0


def test_no_cycle_time(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_energy_neut_amount)
    eve_max_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.ftr_abil_energy_neut, cat_id=consts.EveEffCat.target)
    eve_abil_id = client.mk_eve_abil(id_=consts.EveAbil.energy_neut)
    eve_fighter_id = client.mk_eve_fighter(
        attrs={eve_neut_amount_attr_id: 99, eve_cycle_time_attr_id: 6000, eve_max_count_attr_id: 3},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id,
        abils=[client.mk_eve_item_abil(id_=eve_abil_id)])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_nps=True))
    assert api_fleet_stats.outgoing_nps.one() == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_nps=True))
    assert api_fit_stats.outgoing_nps.one() == 0
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(outgoing_nps=True))
    assert api_fighter_stats.outgoing_nps.one() == 0
