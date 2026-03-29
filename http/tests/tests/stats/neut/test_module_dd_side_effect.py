from fw import approx
from fw.api import FitStatsOptions, FleetStatsOptions, ItemStatsOptions, StatsOptionItemOutNps


def test_state(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_energy_neut_amount)
    eve_neut_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_energy_neut_radius)
    eve_neut_sigres_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_energy_neut_sig_radius)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_boson_id = client.mk_eve_effect(
        id_=consts.EveEffect.doomsday_cone_dot,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_effect_lance_id = client.mk_eve_effect(
        id_=consts.EveEffect.doomsday_beam_dot,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_effect_lance_debuff_id = client.mk_eve_effect(
        id_=consts.EveEffect.debuff_lance,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_effect_reaper_id = client.mk_eve_effect(
        id_=consts.EveEffect.doomsday_slash,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_boson_id = client.mk_eve_item(
        attrs={
            eve_neut_amount_attr_id: 30000, eve_neut_range_attr_id: 10000,
            eve_neut_sigres_attr_id: 20000, eve_cycle_time_attr_id: 240000},
        eff_ids=[eve_effect_boson_id],
        defeff_id=eve_effect_boson_id)
    eve_module_lance_id = client.mk_eve_item(
        attrs={
            eve_neut_amount_attr_id: 32500, eve_neut_range_attr_id: 10000,
            eve_neut_sigres_attr_id: 20000, eve_cycle_time_attr_id: 240000},
        eff_ids=[eve_effect_lance_id],
        defeff_id=eve_effect_lance_id)
    eve_module_lance_debuff_id = client.mk_eve_item(
        attrs={
            eve_neut_amount_attr_id: 33750, eve_neut_range_attr_id: 10000,
            eve_neut_sigres_attr_id: 20000, eve_cycle_time_attr_id: 300000},
        eff_ids=[eve_effect_lance_debuff_id],
        defeff_id=eve_effect_lance_debuff_id)
    eve_module_reaper_id = client.mk_eve_item(
        attrs={
            eve_neut_amount_attr_id: 32500, eve_neut_range_attr_id: 10000,
            eve_neut_sigres_attr_id: 20000, eve_cycle_time_attr_id: 240000},
        eff_ids=[eve_effect_reaper_id],
        defeff_id=eve_effect_reaper_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module_boson = api_fit.add_module(type_id=eve_module_boson_id, state=consts.ApiModuleState.active)
    api_module_lance = api_fit.add_module(type_id=eve_module_lance_id, state=consts.ApiModuleState.active)
    api_module_lance_debuff = api_fit.add_module(type_id=eve_module_lance_debuff_id, state=consts.ApiModuleState.active)
    api_module_reaper = api_fit.add_module(type_id=eve_module_reaper_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_nps=True))
    assert api_fleet_stats.outgoing_nps.one() == approx(508.333333)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_nps=True))
    assert api_fit_stats.outgoing_nps.one() == approx(508.333333)
    api_module_boson_stats = api_module_boson.get_stats(options=ItemStatsOptions(outgoing_nps=True))
    assert api_module_boson_stats.outgoing_nps.one() == approx(125)
    api_module_lance_stats = api_module_lance.get_stats(options=ItemStatsOptions(outgoing_nps=True))
    assert api_module_lance_stats.outgoing_nps.one() == approx(135.416667)
    api_module_lance_debuff_stats = api_module_lance_debuff.get_stats(options=ItemStatsOptions(outgoing_nps=True))
    assert api_module_lance_debuff_stats.outgoing_nps.one() == approx(112.5)
    api_module_reaper_stats = api_module_reaper.get_stats(options=ItemStatsOptions(outgoing_nps=True))
    assert api_module_reaper_stats.outgoing_nps.one() == approx(135.416667)
    # Action
    api_module_boson.change_module(state=consts.ApiModuleState.online)
    api_module_lance.change_module(state=consts.ApiModuleState.online)
    api_module_lance_debuff.change_module(state=consts.ApiModuleState.online)
    api_module_reaper.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_nps=True))
    assert api_fleet_stats.outgoing_nps.one() == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_nps=True))
    assert api_fit_stats.outgoing_nps.one() == 0
    api_module_boson_stats = api_module_boson.get_stats(options=ItemStatsOptions(outgoing_nps=(True, [
        StatsOptionItemOutNps(ignore_state=False),
        StatsOptionItemOutNps(ignore_state=True)])))
    assert api_module_boson_stats.outgoing_nps == [0, approx(125)]
    api_module_lance_stats = api_module_lance.get_stats(options=ItemStatsOptions(outgoing_nps=(True, [
        StatsOptionItemOutNps(ignore_state=False),
        StatsOptionItemOutNps(ignore_state=True)])))
    assert api_module_lance_stats.outgoing_nps == [0, approx(135.416667)]
    api_module_lance_debuff_stats = api_module_lance_debuff.get_stats(options=ItemStatsOptions(outgoing_nps=(True, [
        StatsOptionItemOutNps(ignore_state=False),
        StatsOptionItemOutNps(ignore_state=True)])))
    assert api_module_lance_debuff_stats.outgoing_nps == [0, approx(112.5)]
    api_module_reaper_stats = api_module_reaper.get_stats(options=ItemStatsOptions(outgoing_nps=(True, [
        StatsOptionItemOutNps(ignore_state=False),
        StatsOptionItemOutNps(ignore_state=True)])))
    assert api_module_reaper_stats.outgoing_nps == [0, approx(135.416667)]
    # Action
    api_module_boson.change_module(state=consts.ApiModuleState.active)
    api_module_lance.change_module(state=consts.ApiModuleState.active)
    api_module_lance_debuff.change_module(state=consts.ApiModuleState.active)
    api_module_reaper.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_nps=True))
    assert api_fleet_stats.outgoing_nps.one() == approx(508.333333)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_nps=True))
    assert api_fit_stats.outgoing_nps.one() == approx(508.333333)
    api_module_boson_stats = api_module_boson.get_stats(options=ItemStatsOptions(outgoing_nps=True))
    assert api_module_boson_stats.outgoing_nps.one() == approx(125)
    api_module_lance_stats = api_module_lance.get_stats(options=ItemStatsOptions(outgoing_nps=True))
    assert api_module_lance_stats.outgoing_nps.one() == approx(135.416667)
    api_module_lance_debuff_stats = api_module_lance_debuff.get_stats(options=ItemStatsOptions(outgoing_nps=True))
    assert api_module_lance_debuff_stats.outgoing_nps.one() == approx(112.5)
    api_module_reaper_stats = api_module_reaper.get_stats(options=ItemStatsOptions(outgoing_nps=True))
    assert api_module_reaper_stats.outgoing_nps.one() == approx(135.416667)
