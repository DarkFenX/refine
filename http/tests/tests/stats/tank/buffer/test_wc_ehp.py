from tests import approx
from tests.fw.api import FitStatsOptions, ItemStatsOptions
from tests.tests.stats.tank import (
    make_eve_local_aar,
    make_eve_local_asb,
    make_eve_remote_aar,
    make_eve_remote_asb,
    make_eve_tankable,
    setup_tank_basics,
)


def test_buffer_ship(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(
        client=client,
        basic_info=eve_basic_info,
        hps=(225, 575, 525),
        resos_shield=(1, 0.8, 0.6, 0.4),
        resos_armor=(0.5, 0.65, 0.75, 0.7),
        resos_hull=(0.67, 0.67, 0.67, 0.67))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(wc_ehp=True))
    assert api_fit_stats.wc_ehp.shield == (approx(225), 0, 0, approx(1))
    assert api_fit_stats.wc_ehp.armor == (approx(766.666667), 0, 0, approx(1.333333))
    assert api_fit_stats.wc_ehp.hull == (approx(783.58209), 0, 0, approx(1.492537))
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(wc_ehp=True))
    assert api_ship_stats.wc_ehp.shield == (approx(225), 0, 0, approx(1))
    assert api_ship_stats.wc_ehp.armor == (approx(766.666667), 0, 0, approx(1.333333))
    assert api_ship_stats.wc_ehp.hull == (approx(783.58209), 0, 0, approx(1.492537))


def test_buffer_drone(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_drone_id = make_eve_tankable(
        client=client,
        basic_info=eve_basic_info,
        hps=(1728, 672, 600),
        resos_shield=(1, 0.8, 0.6, 0.5),
        resos_armor=(0.5, 0.55, 0.75, 0.9),
        resos_hull=(1, 1, 1, 1))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(wc_ehp=True))
    assert api_drone_stats.wc_ehp.shield == (approx(1728), 0, 0, approx(1))
    assert api_drone_stats.wc_ehp.armor == (approx(746.666667), 0, 0, approx(1.111111))
    assert api_drone_stats.wc_ehp.hull == (approx(600), 0, 0, approx(1))


def test_buffer_fighter(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_fighter_id = make_eve_tankable(
        client=client,
        basic_info=eve_basic_info,
        hps=(2190, None, 100),
        resos_shield=(0.7, 0.85, 1, 1))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(wc_ehp=True))
    assert api_fighter_stats.wc_ehp.shield == (approx(2190), 0, 0, approx(1))
    assert api_fighter_stats.wc_ehp.armor == (0, 0, 0, approx(1))
    assert api_fighter_stats.wc_ehp.hull == (approx(100), 0, 0, approx(1))


def test_immunity(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(
        client=client,
        basic_info=eve_basic_info,
        hps=(225, 575, 525),
        resos_shield=(1, 0.8, 0.6, 0.4),
        resos_armor=(0, 0, 0, 0),
        resos_hull=(0.67, 0.67, 0.67, 0.67))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(wc_ehp=True))
    assert api_fit_stats.wc_ehp.shield == (approx(225), 0, 0, approx(1))
    assert api_fit_stats.wc_ehp.armor is None
    assert api_fit_stats.wc_ehp.hull == (approx(783.58209), 0, 0, approx(1.492537))
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(wc_ehp=True))
    assert api_ship_stats.wc_ehp.shield == (approx(225), 0, 0, approx(1))
    assert api_ship_stats.wc_ehp.armor is None
    assert api_ship_stats.wc_ehp.hull == (approx(783.58209), 0, 0, approx(1.492537))


def test_local_asb(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(
        client=client,
        basic_info=eve_basic_info,
        hps=(833, 457, 605),
        resos_shield=(0.25, 0.4, 0.6, 0.5),
        resos_armor=(0.1, 0.325, 0.75, 0.9),
        resos_hull=(0.67, 0.67, 0.67, 0.67))
    eve_rep_item_id = make_eve_local_asb(client=client, basic_info=eve_basic_info, rep_amount=146, capacity=14)
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
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(wc_ehp=True))
    assert api_fit_stats.wc_ehp.shield == (approx(1388.333333), approx(2190), 0, approx(1.666667))
    assert api_fit_stats.wc_ehp.armor == (approx(507.777778), 0, 0, approx(1.111111))
    assert api_fit_stats.wc_ehp.hull == (approx(902.985075), 0, 0, approx(1.492537))
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(wc_ehp=True))
    assert api_ship_stats.wc_ehp.shield == (approx(1388.333333), approx(2190), 0, approx(1.666667))
    assert api_ship_stats.wc_ehp.armor == (approx(507.777778), 0, 0, approx(1.111111))
    assert api_ship_stats.wc_ehp.hull == (approx(902.985075), 0, 0, approx(1.492537))


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
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(wc_ehp=True))
    assert api_fit_stats.wc_ehp.shield == (approx(225), 0, 0, approx(1))
    assert api_fit_stats.wc_ehp.armor == (approx(766.666667), approx(1664), 0, approx(1.333333))
    assert api_fit_stats.wc_ehp.hull == (approx(783.58209), 0, 0, approx(1.492537))
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(wc_ehp=True))
    assert api_ship_stats.wc_ehp.shield == (approx(225), 0, 0, approx(1))
    assert api_ship_stats.wc_ehp.armor == (approx(766.666667), approx(1664), 0, approx(1.333333))
    assert api_ship_stats.wc_ehp.hull == (approx(783.58209), 0, 0, approx(1.492537))


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
    api_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_rasb.change_module(add_projs=[api_ship.id])
    # Verification
    api_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(wc_ehp=True))
    assert api_fit_stats.wc_ehp.shield == (approx(1388.333333), 0, approx(7125), approx(1.666667))
    assert api_fit_stats.wc_ehp.armor == (approx(507.777778), 0, 0, approx(1.111111))
    assert api_fit_stats.wc_ehp.hull == (approx(902.985075), 0, 0, approx(1.492537))
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(wc_ehp=True))
    assert api_ship_stats.wc_ehp.shield == (approx(1388.333333), 0, approx(7125), approx(1.666667))
    assert api_ship_stats.wc_ehp.armor == (approx(507.777778), 0, 0, approx(1.111111))
    assert api_ship_stats.wc_ehp.hull == (approx(902.985075), 0, 0, approx(1.492537))


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
    api_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_raar.change_module(add_projs=[api_ship.id])
    # Verification
    api_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(wc_ehp=True))
    assert api_fit_stats.wc_ehp.shield == (approx(225), 0, 0, approx(1))
    assert api_fit_stats.wc_ehp.armor == (approx(766.666667), 0, approx(1184), approx(1.333333))
    assert api_fit_stats.wc_ehp.hull == (approx(783.58209), 0, 0, approx(1.492537))
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(wc_ehp=True))
    assert api_ship_stats.wc_ehp.shield == (approx(225), 0, 0, approx(1))
    assert api_ship_stats.wc_ehp.armor == (approx(766.666667), 0, approx(1184), approx(1.333333))
    assert api_ship_stats.wc_ehp.hull == (approx(783.58209), 0, 0, approx(1.492537))


def test_no_ship(client, consts):
    setup_tank_basics(client=client, consts=consts)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(wc_ehp=True))
    assert api_fit_stats.wc_ehp is None


def test_item_not_loaded(client, consts):
    setup_tank_basics(client=client, consts=consts)
    eve_item_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_item_id)
    api_drone = api_fit.add_drone(type_id=eve_item_id)
    api_fighter = api_fit.add_fighter(type_id=eve_item_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(wc_ehp=True))
    assert api_fit_stats.wc_ehp is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(wc_ehp=True))
    assert api_ship_stats.wc_ehp is None
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(wc_ehp=True))
    assert api_drone_stats.wc_ehp is None
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(wc_ehp=True))
    assert api_fighter_stats.wc_ehp is None
