from tests import approx
from tests.fw.api import FitStatsOptions, ItemStatsOptions, StatsOptionErps
from tests.tests.stats.tank import (
    make_eve_local_ar,
    make_eve_local_hr,
    make_eve_local_sb,
    make_eve_remote_ar,
    make_eve_remote_hr,
    make_eve_remote_sb,
    make_eve_tankable,
    setup_tank_basics,
)


def test_ship_dps_profiles(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(
        client=client,
        basic_info=eve_basic_info,
        hps=(100000, 100000, 100000),
        shield_regen=11250000,
        resos_shield=(1, 0.8, 0.6, 0.4),
        resos_armor=(0.5, 0.65, 0.75, 0.7),
        resos_hull=(0.67, 0.67, 0.67, 0.67))
    eve_module_lsb_id = make_eve_local_sb(client=client, basic_info=eve_basic_info, rep_amount=45000, cycle_time=11250)
    eve_module_lar_id = make_eve_local_ar(client=client, basic_info=eve_basic_info, rep_amount=45000, cycle_time=11250)
    eve_module_lhr_id = make_eve_local_hr(client=client, basic_info=eve_basic_info, rep_amount=45000, cycle_time=11250)
    eve_module_rsb_id = make_eve_remote_sb(client=client, basic_info=eve_basic_info, rep_amount=8594, cycle_time=5000)
    eve_module_rar_id = make_eve_remote_ar(client=client, basic_info=eve_basic_info, rep_amount=8594, cycle_time=5000)
    eve_module_rhr_id = make_eve_remote_hr(client=client, basic_info=eve_basic_info, rep_amount=8594, cycle_time=5000)
    client.create_sources()
    api_sol = client.create_sol(default_incoming_dps=(1, 1, 0, 0))
    api_src_fit = api_sol.create_fit()
    api_module_rsb = api_src_fit.add_module(type_id=eve_module_rsb_id, state=consts.ApiModuleState.active)
    api_module_rar = api_src_fit.add_module(type_id=eve_module_rar_id, state=consts.ApiModuleState.active)
    api_module_rhr = api_src_fit.add_module(type_id=eve_module_rhr_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_tgt_fit.add_module(type_id=eve_module_lsb_id, state=consts.ApiModuleState.active)
    api_tgt_fit.add_module(type_id=eve_module_lar_id, state=consts.ApiModuleState.active)
    api_tgt_fit.add_module(type_id=eve_module_lhr_id, state=consts.ApiModuleState.active)
    api_module_rsb.change_module(add_projs=[api_tgt_ship.id])
    api_module_rar.change_module(add_projs=[api_tgt_ship.id])
    api_module_rhr.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_stat_options = [
        StatsOptionErps(incoming_dps=(1, 1, 1, 1)),
        StatsOptionErps(),
        StatsOptionErps(incoming_dps=(100, 100, -5, -25)),
        StatsOptionErps(incoming_dps=(0, 0, 0, 0)),
        StatsOptionErps(incoming_dps=(0, 0, 1, 1))]
    api_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(erps=(True, api_stat_options)))
    (api_fit_uniform,
     api_fit_default,
     api_fit_negative,
     api_fit_zero,
     api_fit_kin_exp) = api_fit_stats.erps
    api_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(erps=(True, api_stat_options)))
    (api_ship_uniform,
     api_ship_default,
     api_ship_negative,
     api_ship_zero,
     api_ship_kin_exp) = api_ship_stats.erps
    assert api_fit_uniform.shield == [
        approx(5714.285714), approx(2455.428571), approx(2455.428571), approx(31.746032), approx(1.428571)]
    assert api_fit_uniform.armor == [approx(6153.846153), approx(2644.307692), approx(2644.307692), approx(1.538462)]
    assert api_fit_uniform.hull == [approx(5970.149254), approx(2565.373134), approx(2565.373134), approx(1.492537)]
    assert api_ship_uniform.shield == [
        approx(5714.285714), approx(2455.428571), approx(2455.428571), approx(31.746032), approx(1.428571)]
    assert api_ship_uniform.armor == [approx(6153.846153), approx(2644.307692), approx(2644.307692), approx(1.538462)]
    assert api_ship_uniform.hull == [approx(5970.149254), approx(2565.373134), approx(2565.373134), approx(1.492537)]
    # No dps profile was specified, so default one was used
    assert api_fit_default.shield == [
        approx(4444.444444), approx(1909.777778), approx(1909.777778), approx(24.691358), approx(1.111111)]
    assert api_fit_default.armor == [approx(6956.521739), approx(2989.217391), approx(2989.217391), approx(1.73913)]
    assert api_fit_default.hull == [approx(5970.149254), approx(2565.373134), approx(2565.373134), approx(1.492537)]
    assert api_ship_default.shield == [
        approx(4444.444444), approx(1909.777778), approx(1909.777778), approx(24.691358), approx(1.111111)]
    assert api_ship_default.armor == [approx(6956.521739), approx(2989.217391), approx(2989.217391), approx(1.73913)]
    assert api_ship_default.hull == [approx(5970.149254), approx(2565.373134), approx(2565.373134), approx(1.492537)]
    # Negative values were clamped to 0
    assert api_fit_negative.shield == [
        approx(4444.444444), approx(1909.777778), approx(1909.777778), approx(24.691358), approx(1.111111)]
    assert api_fit_negative.armor == [approx(6956.521739), approx(2989.217391), approx(2989.217391), approx(1.73913)]
    assert api_fit_negative.hull == [approx(5970.149254), approx(2565.373134), approx(2565.373134), approx(1.492537)]
    assert api_ship_negative.shield == [
        approx(4444.444444), approx(1909.777778), approx(1909.777778), approx(24.691358), approx(1.111111)]
    assert api_ship_negative.armor == [approx(6956.521739), approx(2989.217391), approx(2989.217391), approx(1.73913)]
    assert api_ship_negative.hull == [approx(5970.149254), approx(2565.373134), approx(2565.373134), approx(1.492537)]
    # Total damage = 0 means no stats returned
    assert api_fit_zero.shield is None
    assert api_fit_zero.armor is None
    assert api_fit_zero.hull is None
    assert api_ship_zero.shield is None
    assert api_ship_zero.armor is None
    assert api_ship_zero.hull is None
    # Kin-explosive
    assert api_fit_kin_exp.shield == [approx(8000), approx(3437.6), approx(3437.6), approx(44.444444), approx(2)]
    assert api_fit_kin_exp.armor == [approx(5517.241379), approx(2370.758621), approx(2370.758621), approx(1.37931)]
    assert api_fit_kin_exp.hull == [approx(5970.149254), approx(2565.373134), approx(2565.373134), approx(1.492537)]
    assert api_ship_kin_exp.shield == [approx(8000), approx(3437.6), approx(3437.6), approx(44.444444), approx(2)]
    assert api_ship_kin_exp.armor == [approx(5517.241379), approx(2370.758621), approx(2370.758621), approx(1.37931)]
    assert api_ship_kin_exp.hull == [approx(5970.149254), approx(2565.373134), approx(2565.373134), approx(1.492537)]


def test_drone_dps_profiles(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_drone_id = make_eve_tankable(
        client=client,
        basic_info=eve_basic_info,
        hps=(720, 3420, 1275),
        shield_regen=250000,
        resos_shield=(1, 0.8, 0.6, 0.5),
        resos_armor=(0.5, 0.65, 0.75, 0.8),
        resos_hull=(1, 1, 1, 1))
    eve_module_rsb_id = make_eve_remote_sb(client=client, basic_info=eve_basic_info, rep_amount=408, cycle_time=7200)
    eve_module_rar_id = make_eve_remote_ar(client=client, basic_info=eve_basic_info, rep_amount=304, cycle_time=5400)
    eve_module_rhr_id = make_eve_remote_hr(client=client, basic_info=eve_basic_info, rep_amount=115, cycle_time=6000)
    client.create_sources()
    api_sol = client.create_sol(default_incoming_dps=(1, 1, 0, 0))
    api_src_fit = api_sol.create_fit()
    api_module_rsb = api_src_fit.add_module(type_id=eve_module_rsb_id, state=consts.ApiModuleState.active)
    api_module_rar = api_src_fit.add_module(type_id=eve_module_rar_id, state=consts.ApiModuleState.active)
    api_module_rhr = api_src_fit.add_module(type_id=eve_module_rhr_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_drone = api_tgt_fit.set_ship(type_id=eve_drone_id)
    api_module_rsb.change_module(add_projs=[api_tgt_drone.id])
    api_module_rar.change_module(add_projs=[api_tgt_drone.id])
    api_module_rhr.change_module(add_projs=[api_tgt_drone.id])
    # Verification
    api_stat_options = [
        StatsOptionErps(incoming_dps=(1, 1, 1, 1)),
        StatsOptionErps(),
        StatsOptionErps(incoming_dps=(100, 100, -5, -25)),
        StatsOptionErps(incoming_dps=(0, 0, 0, 0)),
        StatsOptionErps(incoming_dps=(0, 0, 1, 1))]
    api_drone_stats = api_tgt_drone.get_stats(options=ItemStatsOptions(erps=(True, api_stat_options)))
    (api_drone_uniform,
     api_drone_default,
     api_drone_negative,
     api_drone_zero,
     api_drone_kin_exp) = api_drone_stats.erps
    assert api_drone_uniform.shield == [0, approx(78.16092), approx(78.16092), approx(9.931034), approx(1.37931)]
    assert api_drone_uniform.armor == [0, approx(83.40192), approx(83.40192), approx(1.481481)]
    assert api_drone_uniform.hull == [0, approx(19.166667), approx(19.166667), approx(1)]
    # No dps profile was specified, so default one was used
    assert api_drone_default.shield == [0, approx(62.962963), approx(62.962963), approx(8), approx(1.111111)]
    assert api_drone_default.armor == [0, approx(97.906602), approx(97.906602), approx(1.73913)]
    assert api_drone_default.hull == [0, approx(19.166667), approx(19.166667), approx(1)]
    # Negative values were clamped to 0
    assert api_drone_negative.shield == [0, approx(62.962963), approx(62.962963), approx(8), approx(1.111111)]
    assert api_drone_negative.armor == [0, approx(97.906602), approx(97.906602), approx(1.73913)]
    assert api_drone_negative.hull == [0, approx(19.166667), approx(19.166667), approx(1)]
    # Total damage = 0 means no stats returned
    assert api_drone_zero.shield is None
    assert api_drone_zero.armor is None
    assert api_drone_zero.hull is None
    # Kin-explosive
    assert api_drone_kin_exp.shield == [0, approx(103.030303), approx(103.030303), approx(13.090909), approx(1.818182)]
    assert api_drone_kin_exp.armor == [0, approx(72.640382), approx(72.640382), approx(1.290323)]
    assert api_drone_kin_exp.hull == [0, approx(19.166667), approx(19.166667), approx(1)]


def test_immunity(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship1_id = make_eve_tankable(
        client=client,
        basic_info=eve_basic_info,
        hps=(100000, 100000, 100000),
        shield_regen=11250000,
        resos_shield=(1, 0.8, 0.6, 0.4),
        resos_armor=(0, 0, 0, 0),
        resos_hull=(0.67, 0.67, 0.67, 0.67))
    eve_ship2_id = make_eve_tankable(
        client=client,
        basic_info=eve_basic_info,
        hps=(100000, 100000, 100000),
        shield_regen=11250000,
        resos_shield=(1, 0.8, 0.6, 0.4),
        resos_armor=(0, 0.65, 0.75, 0.7),
        resos_hull=(0.67, 0.67, 0.67, 0.67))
    eve_module_lsb_id = make_eve_local_sb(client=client, basic_info=eve_basic_info, rep_amount=45000, cycle_time=11250)
    eve_module_lar_id = make_eve_local_ar(client=client, basic_info=eve_basic_info, rep_amount=45000, cycle_time=11250)
    eve_module_lhr_id = make_eve_local_hr(client=client, basic_info=eve_basic_info, rep_amount=45000, cycle_time=11250)
    eve_module_rsb_id = make_eve_remote_sb(client=client, basic_info=eve_basic_info, rep_amount=8594, cycle_time=5000)
    eve_module_rar_id = make_eve_remote_ar(client=client, basic_info=eve_basic_info, rep_amount=8594, cycle_time=5000)
    eve_module_rhr_id = make_eve_remote_hr(client=client, basic_info=eve_basic_info, rep_amount=8594, cycle_time=5000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_module_rsb = api_src_fit.add_module(type_id=eve_module_rsb_id, state=consts.ApiModuleState.active)
    api_module_rar = api_src_fit.add_module(type_id=eve_module_rar_id, state=consts.ApiModuleState.active)
    api_module_rhr = api_src_fit.add_module(type_id=eve_module_rhr_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship1_id)
    api_tgt_fit.add_module(type_id=eve_module_lsb_id, state=consts.ApiModuleState.active)
    api_tgt_fit.add_module(type_id=eve_module_lar_id, state=consts.ApiModuleState.active)
    api_tgt_fit.add_module(type_id=eve_module_lhr_id, state=consts.ApiModuleState.active)
    api_module_rsb.change_module(add_projs=[api_tgt_ship.id])
    api_module_rar.change_module(add_projs=[api_tgt_ship.id])
    api_module_rhr.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_profiles = [StatsOptionErps(incoming_dps=(1, 0, 0, 0)), StatsOptionErps(incoming_dps=(1, 1, 1, 1))]
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(erps=(True, api_profiles)))
    api_fit_em, api_fit_uniform = api_tgt_fit_stats.erps
    assert api_fit_em.shield == [approx(4000), approx(1718.8), approx(1718.8), approx(22.222222), approx(1)]
    assert api_fit_em.armor is None
    assert api_fit_em.hull == [approx(5970.149254), approx(2565.373134), approx(2565.373134), approx(1.492537)]
    assert api_fit_uniform.shield == [
        approx(5714.285714), approx(2455.428571), approx(2455.428571), approx(31.746032), approx(1.428571)]
    assert api_fit_uniform.armor is None
    assert api_fit_uniform.hull == [approx(5970.149254), approx(2565.373134), approx(2565.373134), approx(1.492537)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(erps=(True, api_profiles)))
    api_ship_em, api_ship_uniform = api_tgt_ship_stats.erps
    assert api_ship_em.shield == [approx(4000), approx(1718.8), approx(1718.8), approx(22.222222), approx(1)]
    assert api_ship_em.armor is None
    assert api_ship_em.hull == [approx(5970.149254), approx(2565.373134), approx(2565.373134), approx(1.492537)]
    assert api_ship_uniform.shield == [
        approx(5714.285714), approx(2455.428571), approx(2455.428571), approx(31.746032), approx(1.428571)]
    assert api_ship_uniform.armor is None
    assert api_ship_uniform.hull == [approx(5970.149254), approx(2565.373134), approx(2565.373134), approx(1.492537)]
    # Action
    api_tgt_ship.change_ship(type_id=eve_ship2_id)
    # Verification
    api_profiles = [StatsOptionErps(incoming_dps=(1, 0, 0, 0)), StatsOptionErps(incoming_dps=(1, 1, 1, 1))]
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(erps=(True, api_profiles)))
    api_fit_em, api_fit_uniform = api_tgt_fit_stats.erps
    assert api_fit_em.shield == [approx(4000), approx(1718.8), approx(1718.8), approx(22.222222), approx(1)]
    assert api_fit_em.armor is None
    assert api_fit_em.hull == [approx(5970.149254), approx(2565.373134), approx(2565.373134), approx(1.492537)]
    assert api_fit_uniform.shield == [
        approx(5714.285714), approx(2455.428571), approx(2455.428571), approx(31.746032), approx(1.428571)]
    assert api_fit_uniform.armor == [approx(7619.047619), approx(3273.904761), approx(3273.904761), approx(1.904762)]
    assert api_fit_uniform.hull == [approx(5970.149254), approx(2565.373134), approx(2565.373134), approx(1.492537)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(erps=(True, api_profiles)))
    api_ship_em, api_ship_uniform = api_tgt_ship_stats.erps
    assert api_ship_em.shield == [approx(4000), approx(1718.8), approx(1718.8), approx(22.222222), approx(1)]
    assert api_ship_em.armor is None
    assert api_ship_em.hull == [approx(5970.149254), approx(2565.373134), approx(2565.373134), approx(1.492537)]
    assert api_ship_uniform.shield == [
        approx(5714.285714), approx(2455.428571), approx(2455.428571), approx(31.746032), approx(1.428571)]
    assert api_ship_uniform.armor == [approx(7619.047619), approx(3273.904761), approx(3273.904761), approx(1.904762)]
    assert api_ship_uniform.hull == [approx(5970.149254), approx(2565.373134), approx(2565.373134), approx(1.492537)]


def test_shield_regen(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(
        client=client,
        basic_info=eve_basic_info,
        hps=(100000, 100000, 100000),
        shield_regen=11250000,
        resos_shield=(1, 0.8, 0.6, 0.4))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_options = [
        StatsOptionErps(),
        StatsOptionErps(shield_perc=0),
        StatsOptionErps(shield_perc=0.1),
        StatsOptionErps(shield_perc=0.25),
        StatsOptionErps(shield_perc=0.7),
        StatsOptionErps(shield_perc=1)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(erps=(True, api_options)))
    assert api_fit_stats.erps.map(lambda i: i.shield.regen) == [
        approx(31.746032), 0, approx(27.457494), approx(31.746032), approx(17.353654), 0]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(erps=(True, api_options)))
    assert api_ship_stats.erps.map(lambda i: i.shield.regen) == [
        approx(31.746032), 0, approx(27.457494), approx(31.746032), approx(17.353654), 0]
