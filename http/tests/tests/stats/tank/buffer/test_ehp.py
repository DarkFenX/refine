from tests import approx, check_no_field
from tests.fw.api import FitStatsOptions, ItemStatsOptions, StatsOptionEhp
from tests.tests.stats.tank import (
    make_eve_local_aar,
    make_eve_local_asb,
    make_eve_remote_aar,
    make_eve_remote_asb,
    make_eve_tankable,
    setup_tank_basics,
)


def test_dps_profiles_ship(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(
        client=client,
        basic_info=eve_basic_info,
        hps=(225, 575, 525),
        resos_shield=(1, 0.8, 0.6, 0.4),
        resos_armor=(0.5, 0.65, 0.75, 0.7),
        resos_hull=(0.67, 0.67, 0.67, 0.67))
    client.create_sources()
    api_sol = client.create_sol(default_incoming_dps=(1, 1, 0, 0))
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_profiles = [
        StatsOptionEhp(incoming_dps=(1, 1, 1, 1)),
        StatsOptionEhp(),
        StatsOptionEhp(incoming_dps=(100, 100, -5, -25)),
        StatsOptionEhp(incoming_dps=(0, 0, 0, 0)),
        StatsOptionEhp(incoming_dps=(0, 0, 1, 1))]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(ehp=(True, api_profiles)))
    (api_fit_ehp_uniform,
     api_fit_ehp_default,
     api_fit_ehp_negative,
     api_fit_ehp_zero,
     api_fit_ehp_kin_exp) = api_fit_stats.ehp
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(ehp=(True, api_profiles)))
    (api_ship_ehp_uniform,
     api_ship_ehp_default,
     api_ship_ehp_negative,
     api_ship_ehp_zero,
     api_ship_ehp_kin_exp) = api_ship_stats.ehp
    assert api_fit_ehp_uniform.shield == (approx(321.428571), 0, 0, approx(1.428571))
    assert api_fit_ehp_uniform.armor == (approx(884.615385), 0, 0, approx(1.538462))
    assert api_fit_ehp_uniform.hull == (approx(783.58209), 0, 0, approx(1.492537))
    assert api_ship_ehp_uniform.shield == (approx(321.428571), 0, 0, approx(1.428571))
    assert api_ship_ehp_uniform.armor == (approx(884.615385), 0, 0, approx(1.538462))
    assert api_ship_ehp_uniform.hull == (approx(783.58209), 0, 0, approx(1.492537))
    # No dps profile was specified, so default one was used
    assert api_fit_ehp_default.shield == (approx(250), 0, 0, approx(1.111111))
    assert api_fit_ehp_default.armor == (approx(1000), 0, 0, approx(1.73913))
    assert api_fit_ehp_default.hull == (approx(783.58209), 0, 0, approx(1.492537))
    assert api_ship_ehp_default.shield == (approx(250), 0, 0, approx(1.111111))
    assert api_ship_ehp_default.armor == (approx(1000), 0, 0, approx(1.73913))
    assert api_ship_ehp_default.hull == (approx(783.58209), 0, 0, approx(1.492537))
    # Negative values were clamped to 0
    assert api_fit_ehp_negative.shield == (approx(250), 0, 0, approx(1.111111))
    assert api_fit_ehp_negative.armor == (approx(1000), 0, 0, approx(1.73913))
    assert api_fit_ehp_negative.hull == (approx(783.58209), 0, 0, approx(1.492537))
    assert api_ship_ehp_negative.shield == (approx(250), 0, 0, approx(1.111111))
    assert api_ship_ehp_negative.armor == (approx(1000), 0, 0, approx(1.73913))
    assert api_ship_ehp_negative.hull == (approx(783.58209), 0, 0, approx(1.492537))
    # Total damage = 0 means no stats returned
    assert api_fit_ehp_zero.shield is None
    assert api_fit_ehp_zero.armor is None
    assert api_fit_ehp_zero.hull is None
    assert api_ship_ehp_zero.shield is None
    assert api_ship_ehp_zero.armor is None
    assert api_ship_ehp_zero.hull is None
    # Kin-explosive
    assert api_fit_ehp_kin_exp.shield == (approx(450), 0, 0, approx(2))
    assert api_fit_ehp_kin_exp.armor == (approx(793.103448), 0, 0, approx(1.37931))
    assert api_fit_ehp_kin_exp.hull == (approx(783.58209), 0, 0, approx(1.492537))
    assert api_ship_ehp_kin_exp.shield == (approx(450), 0, 0, approx(2))
    assert api_ship_ehp_kin_exp.armor == (approx(793.103448), 0, 0, approx(1.37931))
    assert api_ship_ehp_kin_exp.hull == (approx(783.58209), 0, 0, approx(1.492537))


def test_dps_profiles_drone(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_drone_id = make_eve_tankable(
        client=client,
        basic_info=eve_basic_info,
        hps=(1728, 672, 600),
        resos_shield=(1, 0.8, 0.6, 0.5),
        resos_armor=(0.5, 0.55, 0.75, 0.9),
        resos_hull=(1, 1, 1, 1))
    client.create_sources()
    api_sol = client.create_sol(default_incoming_dps=(1, 1, 0, 0))
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(ehp=(True, [
        StatsOptionEhp(incoming_dps=(1, 1, 1, 1)),
        StatsOptionEhp(),
        StatsOptionEhp(incoming_dps=(100, 100, -5, -25)),
        StatsOptionEhp(incoming_dps=(0, 0, 0, 0)),
        StatsOptionEhp(incoming_dps=(0, 0, 1, 1))])))
    (api_drone_ehp_uniform,
     api_drone_ehp_default,
     api_drone_ehp_negative,
     api_drone_ehp_zero,
     api_drone_ehp_kin_exp) = api_drone_stats.ehp
    assert api_drone_ehp_uniform.shield == (approx(2383.448276), 0, 0, approx(1.37931))
    assert api_drone_ehp_uniform.armor == (approx(995.555556), 0, 0, approx(1.481481))
    assert api_drone_ehp_uniform.hull == (approx(600), 0, 0, approx(1))
    # No dps profile was specified, so default one was used
    assert api_drone_ehp_default.shield == (approx(1920), 0, 0, approx(1.111111))
    assert api_drone_ehp_default.armor == (approx(1280), 0, 0, approx(1.904762))
    assert api_drone_ehp_default.hull == (approx(600), 0, 0, approx(1))
    # Negative values were clamped to 0
    assert api_drone_ehp_negative.shield == (approx(1920), 0, 0, approx(1.111111))
    assert api_drone_ehp_negative.armor == (approx(1280), 0, 0, approx(1.904762))
    assert api_drone_ehp_negative.hull == (approx(600), 0, 0, approx(1))
    # Total damage = 0 means no stats returned
    assert api_drone_ehp_zero.shield is None
    assert api_drone_ehp_zero.armor is None
    assert api_drone_ehp_zero.hull is None
    # Kin-explosive
    assert api_drone_ehp_kin_exp.shield == (approx(3141.818182), 0, 0, approx(1.818182))
    assert api_drone_ehp_kin_exp.armor == (approx(814.545455), 0, 0, approx(1.212121))
    assert api_drone_ehp_kin_exp.hull == (approx(600), 0, 0, approx(1))


def test_dps_profiles_fighter(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_fighter_id = make_eve_tankable(
        client=client,
        basic_info=eve_basic_info,
        hps=(2190, None, 100),
        resos_shield=(0.7, 0.85, 1, 1))
    client.create_sources()
    api_sol = client.create_sol(default_incoming_dps=(1, 1, 0, 0))
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(ehp=(True, [
        StatsOptionEhp(incoming_dps=(1, 1, 1, 1)),
        StatsOptionEhp(),
        StatsOptionEhp(incoming_dps=(100, 100, -5, -25)),
        StatsOptionEhp(incoming_dps=(0, 0, 0, 0)),
        StatsOptionEhp(incoming_dps=(0, 0, 1, 1))])))
    (api_fighter_ehp_uniform,
     api_fighter_ehp_default,
     api_fighter_ehp_negative,
     api_fighter_ehp_zero,
     api_fighter_ehp_kin_exp) = api_fighter_stats.ehp
    assert api_fighter_ehp_uniform.shield == (approx(2467.605634), 0, 0, approx(1.126761))
    assert api_fighter_ehp_uniform.armor == (0, 0, 0, approx(1))
    assert api_fighter_ehp_uniform.hull == (approx(100), 0, 0, approx(1))
    # No dps profile was specified, so default one was used
    assert api_fighter_ehp_default.shield == (approx(2825.806452), 0, 0, approx(1.290323))
    assert api_fighter_ehp_default.armor == (0, 0, 0, approx(1))
    assert api_fighter_ehp_default.hull == (approx(100), 0, 0, approx(1))
    # Negative values were clamped to 0
    assert api_fighter_ehp_negative.shield == (approx(2825.806452), 0, 0, approx(1.290323))
    assert api_fighter_ehp_negative.armor == (0, 0, 0, approx(1))
    assert api_fighter_ehp_negative.hull == (approx(100), 0, 0, approx(1))
    # Total damage = 0 means no stats returned
    assert api_fighter_ehp_zero.shield is None
    assert api_fighter_ehp_zero.armor is None
    assert api_fighter_ehp_zero.hull is None
    # Kin-explosive
    assert api_fighter_ehp_kin_exp.shield == (approx(2190), 0, 0, approx(1))
    assert api_fighter_ehp_kin_exp.armor == (0, 0, 0, approx(1))
    assert api_fighter_ehp_kin_exp.hull == (approx(100), 0, 0, approx(1))


def test_immunity(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship1_id = make_eve_tankable(
        client=client,
        basic_info=eve_basic_info,
        hps=(225, 575, 525),
        resos_shield=(1, 0.8, 0.6, 0.4),
        resos_armor=(0, 0, 0, 0),
        resos_hull=(0.67, 0.67, 0.67, 0.67))
    eve_ship2_id = make_eve_tankable(
        client=client,
        basic_info=eve_basic_info,
        hps=(225, 575, 525),
        resos_shield=(1, 0.8, 0.6, 0.4),
        resos_armor=(0, 0.65, 0.75, 0.7),
        resos_hull=(0.67, 0.67, 0.67, 0.67))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship1_id)
    # Verification
    api_profiles = [StatsOptionEhp(incoming_dps=(1, 0, 0, 0)), StatsOptionEhp(incoming_dps=(1, 1, 1, 1))]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(ehp=(True, api_profiles)))
    api_fit_ehp_em, api_fit_ehp_uniform = api_fit_stats.ehp
    assert api_fit_ehp_em.shield == (approx(225), 0, 0, approx(1))
    assert api_fit_ehp_em.armor is None
    assert api_fit_ehp_em.hull == (approx(783.58209), 0, 0, approx(1.492537))
    assert api_fit_ehp_uniform.shield == (approx(321.428571), 0, 0, approx(1.428571))
    assert api_fit_ehp_uniform.armor is None
    assert api_fit_ehp_uniform.hull == (approx(783.58209), 0, 0, approx(1.492537))
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(ehp=(True, api_profiles)))
    api_ship_ehp_em, api_ship_ehp_uniform = api_ship_stats.ehp
    assert api_ship_ehp_em.shield == (approx(225), 0, 0, approx(1))
    assert api_ship_ehp_em.armor is None
    assert api_ship_ehp_em.hull == (approx(783.58209), 0, 0, approx(1.492537))
    assert api_ship_ehp_uniform.shield == (approx(321.428571), 0, 0, approx(1.428571))
    assert api_ship_ehp_uniform.armor is None
    assert api_ship_ehp_uniform.hull == (approx(783.58209), 0, 0, approx(1.492537))
    # Action
    api_ship.change_ship(type_id=eve_ship2_id)
    # Verification
    api_profiles = [StatsOptionEhp(incoming_dps=(1, 0, 0, 0)), StatsOptionEhp(incoming_dps=(1, 1, 1, 1))]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(ehp=(True, api_profiles)))
    api_fit_ehp_em, api_fit_ehp_uniform = api_fit_stats.ehp
    assert api_fit_ehp_em.shield == (approx(225), 0, 0, approx(1))
    assert api_fit_ehp_em.armor is None
    assert api_fit_ehp_em.hull == (approx(783.58209), 0, 0, approx(1.492537))
    assert api_fit_ehp_uniform.shield == (approx(321.428571), 0, 0, approx(1.428571))
    assert api_fit_ehp_uniform.armor == (approx(1095.238095), 0, 0, approx(1.904762))
    assert api_fit_ehp_uniform.hull == (approx(783.58209), 0, 0, approx(1.492537))
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(ehp=(True, api_profiles)))
    api_ship_ehp_em, api_ship_ehp_uniform = api_ship_stats.ehp
    assert api_ship_ehp_em.shield == (approx(225), 0, 0, approx(1))
    assert api_ship_ehp_em.armor is None
    assert api_ship_ehp_em.hull == (approx(783.58209), 0, 0, approx(1.492537))
    assert api_ship_ehp_uniform.shield == (approx(321.428571), 0, 0, approx(1.428571))
    assert api_ship_ehp_uniform.armor == (approx(1095.238095), 0, 0, approx(1.904762))
    assert api_ship_ehp_uniform.hull == (approx(783.58209), 0, 0, approx(1.492537))


def test_local_asb(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(
        client=client,
        basic_info=eve_basic_info,
        hps=(833, 457, 605),
        resos_shield=(0.25, 0.4, 0.6, 0.5),
        resos_armor=(0.1, 0.325, 0.75, 0.9),
        resos_hull=(0.67, 0.67, 0.67, 0.67))
    eve_rep_item_id = make_eve_local_asb(
        client=client, basic_info=eve_basic_info, rep_amount=146, cycle_time=5000, capacity=14)
    eve_charge_item_id = client.mk_eve_item(attrs={eve_basic_info.volume_attr_id: 1.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(ehp=(True, [
        StatsOptionEhp(incoming_dps=(1, 1, 1, 1)),
        StatsOptionEhp(incoming_dps=(1, 1, 0, 0))])))
    api_fit_ehp_uniform, api_fit_ehp_em_therm = api_fit_stats.ehp
    assert api_fit_ehp_uniform.shield == (approx(1904), approx(3003.428571), 0, approx(2.285714))
    assert api_fit_ehp_uniform.armor == (approx(880.963855), 0, 0, approx(1.927711))
    assert api_fit_ehp_uniform.hull == (approx(902.985075), 0, 0, approx(1.492537))
    assert api_fit_ehp_em_therm.shield == (approx(2563.076923), approx(4043.076923), 0, approx(3.076923))
    assert api_fit_ehp_em_therm.armor == (approx(2150.588235), 0, 0, approx(4.705882))
    assert api_fit_ehp_em_therm.hull == (approx(902.985075), 0, 0, approx(1.492537))
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(ehp=(True, [
        StatsOptionEhp(incoming_dps=(1, 1, 1, 1)),
        StatsOptionEhp(incoming_dps=(1, 1, 0, 0))])))
    api_ship_ehp_uniform, api_ship_ehp_em_therm = api_ship_stats.ehp
    assert api_ship_ehp_uniform.shield == (approx(1904), approx(3003.428571), 0, approx(2.285714))
    assert api_ship_ehp_uniform.armor == (approx(880.963855), 0, 0, approx(1.927711))
    assert api_ship_ehp_uniform.hull == (approx(902.985075), 0, 0, approx(1.492537))
    assert api_ship_ehp_em_therm.shield == (approx(2563.076923), approx(4043.076923), 0, approx(3.076923))
    assert api_ship_ehp_em_therm.armor == (approx(2150.588235), 0, 0, approx(4.705882))
    assert api_ship_ehp_em_therm.hull == (approx(902.985075), 0, 0, approx(1.492537))


def test_local_aar(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(
        client=client,
        basic_info=eve_basic_info,
        hps=(225, 575, 525),
        resos_shield=(1, 0.8, 0.6, 0.4),
        resos_armor=(0.5, 0.65, 0.75, 0.7),
        resos_hull=(0.67, 0.67, 0.67, 0.67))
    eve_rep_item_id = make_eve_local_aar(
        client=client,
        basic_info=eve_basic_info,
        rep_amount=52,
        cycle_time=5000,
        capacity=0.08,
        charge_rate=1)
    eve_charge_item_id = client.mk_eve_item(
        id_=consts.EveItem.nanite_repair_paste,
        attrs={eve_basic_info.volume_attr_id: 0.01})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(ehp=(True, [
        StatsOptionEhp(incoming_dps=(1, 1, 1, 1)),
        StatsOptionEhp(incoming_dps=(1, 1, 0, 0))])))
    api_fit_ehp_uniform, api_fit_ehp_em_therm = api_fit_stats.ehp
    assert api_fit_ehp_uniform.shield == (approx(321.428571), 0, 0, approx(1.428571))
    assert api_fit_ehp_uniform.armor == (approx(884.615385), approx(1920), 0, approx(1.538462))
    assert api_fit_ehp_uniform.hull == (approx(783.58209), 0, 0, approx(1.492537))
    assert api_fit_ehp_em_therm.shield == (approx(250), 0, 0, approx(1.111111))
    assert api_fit_ehp_em_therm.armor == (approx(1000), approx(2170.434782), 0, approx(1.73913))
    assert api_fit_ehp_em_therm.hull == (approx(783.58209), 0, 0, approx(1.492537))
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(ehp=(True, [
        StatsOptionEhp(incoming_dps=(1, 1, 1, 1)),
        StatsOptionEhp(incoming_dps=(1, 1, 0, 0))])))
    api_ship_ehp_uniform, api_ship_ehp_em_therm = api_ship_stats.ehp
    assert api_ship_ehp_uniform.shield == (approx(321.428571), 0, 0, approx(1.428571))
    assert api_ship_ehp_uniform.armor == (approx(884.615385), approx(1920), 0, approx(1.538462))
    assert api_ship_ehp_uniform.hull == (approx(783.58209), 0, 0, approx(1.492537))
    assert api_ship_ehp_em_therm.shield == (approx(250), 0, 0, approx(1.111111))
    assert api_ship_ehp_em_therm.armor == (approx(1000), approx(2170.434782), 0, approx(1.73913))
    assert api_ship_ehp_em_therm.hull == (approx(783.58209), 0, 0, approx(1.492537))


def test_remote_asb(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(
        client=client,
        basic_info=eve_basic_info,
        hps=(833, 457, 605),
        resos_shield=(0.25, 0.4, 0.6, 0.5),
        resos_armor=(0.1, 0.325, 0.75, 0.9),
        resos_hull=(0.67, 0.67, 0.67, 0.67))
    eve_rep_item_id = make_eve_remote_asb(
        client=client, basic_info=eve_basic_info, rep_amount=475, cycle_time=8000, capacity=14)
    eve_charge_item_id = client.mk_eve_item(attrs={eve_basic_info.volume_attr_id: 1.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_rasb = api_src_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_rasb.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(ehp=(True, [
        StatsOptionEhp(incoming_dps=(1, 1, 1, 1)),
        StatsOptionEhp(incoming_dps=(1, 1, 0, 0))])))
    api_tgt_fit_ehp_uniform, api_tgt_fit_ehp_em_therm = api_tgt_fit_stats.ehp
    assert api_tgt_fit_ehp_uniform.shield == (approx(1904), 0, approx(9771.428571), approx(2.285714))
    assert api_tgt_fit_ehp_uniform.armor == (approx(880.963855), 0, 0, approx(1.927711))
    assert api_tgt_fit_ehp_uniform.hull == (approx(902.985075), 0, 0, approx(1.492537))
    assert api_tgt_fit_ehp_em_therm.shield == (approx(2563.076923), 0, approx(13153.846154), approx(3.076923))
    assert api_tgt_fit_ehp_em_therm.armor == (approx(2150.588235), 0, 0, approx(4.705882))
    assert api_tgt_fit_ehp_em_therm.hull == (approx(902.985075), 0, 0, approx(1.492537))
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(ehp=(True, [
        StatsOptionEhp(incoming_dps=(1, 1, 1, 1)),
        StatsOptionEhp(incoming_dps=(1, 1, 0, 0))])))
    api_tgt_ship_ehp_uniform, api_tgt_ship_ehp_em_therm = api_tgt_ship_stats.ehp
    assert api_tgt_ship_ehp_uniform.shield == (approx(1904), 0, approx(9771.428571), approx(2.285714))
    assert api_tgt_ship_ehp_uniform.armor == (approx(880.963855), 0, 0, approx(1.927711))
    assert api_tgt_ship_ehp_uniform.hull == (approx(902.985075), 0, 0, approx(1.492537))
    assert api_tgt_ship_ehp_em_therm.shield == (approx(2563.076923), 0, approx(13153.846154), approx(3.076923))
    assert api_tgt_ship_ehp_em_therm.armor == (approx(2150.588235), 0, 0, approx(4.705882))
    assert api_tgt_ship_ehp_em_therm.hull == (approx(902.985075), 0, 0, approx(1.492537))


def test_remote_aar(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(
        client=client,
        basic_info=eve_basic_info,
        hps=(225, 575, 525),
        resos_shield=(1, 0.8, 0.6, 0.4),
        resos_armor=(0.5, 0.65, 0.75, 0.7),
        resos_hull=(0.67, 0.67, 0.67, 0.67))
    eve_rep_item_id = make_eve_remote_aar(
        client=client,
        basic_info=eve_basic_info,
        rep_amount=37,
        cycle_time=3000,
        capacity=0.08,
        charge_rate=1)
    eve_charge_item_id = client.mk_eve_item(
        id_=consts.EveItem.nanite_repair_paste,
        attrs={eve_basic_info.volume_attr_id: 0.01})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_raar = api_src_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_raar.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(ehp=(True, [
        StatsOptionEhp(incoming_dps=(1, 1, 1, 1)),
        StatsOptionEhp(incoming_dps=(1, 1, 0, 0))])))
    api_tgt_fit_ehp_uniform, api_tgt_fit_ehp_em_therm = api_tgt_fit_stats.ehp
    assert api_tgt_fit_ehp_uniform.shield == (approx(321.428571), 0, 0, approx(1.428571))
    assert api_tgt_fit_ehp_uniform.armor == (approx(884.615385), 0, approx(1366.153846), approx(1.538462))
    assert api_tgt_fit_ehp_uniform.hull == (approx(783.58209), 0, 0, approx(1.492537))
    assert api_tgt_fit_ehp_em_therm.shield == (approx(250), 0, 0, approx(1.111111))
    assert api_tgt_fit_ehp_em_therm.armor == (approx(1000), 0, approx(1544.347826), approx(1.73913))
    assert api_tgt_fit_ehp_em_therm.hull == (approx(783.58209), 0, 0, approx(1.492537))
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(ehp=(True, [
        StatsOptionEhp(incoming_dps=(1, 1, 1, 1)),
        StatsOptionEhp(incoming_dps=(1, 1, 0, 0))])))
    api_tgt_ship_ehp_uniform, api_tgt_ship_ehp_em_therm = api_tgt_ship_stats.ehp
    assert api_tgt_ship_ehp_uniform.shield == (approx(321.428571), 0, 0, approx(1.428571))
    assert api_tgt_ship_ehp_uniform.armor == (approx(884.615385), 0, approx(1366.153846), approx(1.538462))
    assert api_tgt_ship_ehp_uniform.hull == (approx(783.58209), 0, 0, approx(1.492537))
    assert api_tgt_ship_ehp_em_therm.shield == (approx(250), 0, 0, approx(1.111111))
    assert api_tgt_ship_ehp_em_therm.armor == (approx(1000), 0, approx(1544.347826), approx(1.73913))
    assert api_tgt_ship_ehp_em_therm.hull == (approx(783.58209), 0, 0, approx(1.492537))


def test_no_ship(client, consts):
    setup_tank_basics(client=client, consts=consts)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(ehp=(True, [StatsOptionEhp(incoming_dps=(1, 1, 1, 1))])))
    assert api_fit_stats.ehp is None


def test_item_not_loaded(client, consts):
    setup_tank_basics(client=client, consts=consts)
    eve_item_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol(default_incoming_dps=(1, 1, 1, 1))
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_item_id)
    api_drone = api_fit.add_drone(type_id=eve_item_id)
    api_fighter = api_fit.add_fighter(type_id=eve_item_id)
    # Verification
    api_dps_profiles = [StatsOptionEhp(incoming_dps=(1, 1, 1, 1))]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(ehp=(True, api_dps_profiles)))
    assert api_fit_stats.ehp is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(ehp=(True, api_dps_profiles)))
    assert api_ship_stats.ehp is None
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(ehp=(True, api_dps_profiles)))
    assert api_drone_stats.ehp is None
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(ehp=(True, api_dps_profiles)))
    assert api_fighter_stats.ehp is None


def test_not_requested(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(
        client=client,
        basic_info=eve_basic_info,
        hps=(225, 575, 525),
        resos_shield=(1, 0.8, 0.6, 0.4),
        resos_armor=(0.5, 0.65, 0.75, 0.7),
        resos_hull=(0.67, 0.67, 0.67, 0.67))
    client.create_sources()
    api_sol = client.create_sol(default_incoming_dps=(1, 1, 0, 0))
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(ehp=False))
    with check_no_field():
        api_fit_stats.ehp  # noqa: B018
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(ehp=False))
    with check_no_field():
        api_ship_stats.ehp  # noqa: B018
