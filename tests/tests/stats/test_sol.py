from fw import Absent, check_no_field
from fw.api import FitStatsOptions, FleetStatsOptions, ItemStatsOptions


def test_error_body_malformed(client):
    client.create_sources()
    api_sol = client.create_sol()
    req = client.get_sol_stats_request(
        sol_id=api_sol.id,
        fleet_options=Absent,
        fit_options=FitStatsOptions(mass=True),
        item_options=Absent)
    req.set_json(data={'fit_options': 'random'})
    # Action & verification
    req.send().check(status_code=400, json_predicate={'code': 'JSN-001', 'message': 're:.+'})


def test_error_no_sol(client):
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.remove()
    # Action & verification
    resp = client.get_sol_stats_request(
        sol_id=api_sol.id,
        fleet_options=Absent,
        fit_options=Absent,
        item_options=Absent).send()
    resp.check(status_code=404, json_predicate={'code': 'SOL-001', 'message': f'solar system {api_sol.id} not found'})


def test_error_sol_id_malformed(client):
    # Action & verification
    resp = client.get_sol_stats_request(
        sol_id='1',
        fleet_options=Absent,
        fit_options=Absent,
        item_options=Absent).send()
    resp.check(status_code=404, json_predicate={'code': 'SOL-002', 'message': 'failed to parse a UUID'})


def test_defaults(client, consts):
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_ship1_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 1000000})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 2000000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fleet1 = api_sol.create_fleet()
    api_fleet2 = api_sol.create_fleet()
    api_fit1 = api_sol.create_fit(fleet_id=api_fleet1.id)
    api_fit2 = api_sol.create_fit(fleet_id=api_fleet2.id)
    api_ship1 = api_fit1.set_ship(type_id=eve_ship1_id)
    api_ship2 = api_fit2.set_ship(type_id=eve_ship2_id)
    # Action
    api_sol_stats = api_sol.get_stats(
        fleet_options=FleetStatsOptions(mass=True),
        fit_options=FitStatsOptions(mass=True),
        item_options=ItemStatsOptions(mass=True))
    # Verification
    assert api_sol_stats.fleets.keys() == {api_fleet1.id, api_fleet2.id}
    assert api_sol_stats.fleets[api_fleet1.id].mass.one() == 1000000
    assert api_sol_stats.fleets[api_fleet2.id].mass.one() == 2000000
    assert api_sol_stats.fits.keys() == {api_fit1.id, api_fit2.id}
    assert api_sol_stats.fits[api_fit1.id].mass.one() == 1000000
    assert api_sol_stats.fits[api_fit2.id].mass.one() == 2000000
    assert api_sol_stats.items.keys() == {api_ship1.id, api_ship2.id}
    assert api_sol_stats.items[api_ship1.id].mass.one() == 1000000
    assert api_sol_stats.items[api_ship2.id].mass.one() == 2000000


def test_overrides_only(client, consts):
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_ship1_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 1000000})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 2000000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fleet1 = api_sol.create_fleet()
    api_fleet2 = api_sol.create_fleet()
    api_fit1 = api_sol.create_fit(fleet_id=api_fleet1.id)
    api_fit2 = api_sol.create_fit(fleet_id=api_fleet2.id)
    api_fit1.set_ship(type_id=eve_ship1_id)
    api_ship2 = api_fit2.set_ship(type_id=eve_ship2_id)
    # Action
    api_sol_stats = api_sol.get_stats(
        fleet_options=(FleetStatsOptions(), [(FleetStatsOptions(mass=True), [api_fleet2.id])]),
        fit_options=(FitStatsOptions(), [(FitStatsOptions(mass=True), [api_fit2.id])]),
        item_options=(ItemStatsOptions(), [(ItemStatsOptions(mass=True), [api_ship2.id])]))
    # Verification - when defaults request no stats, only overridden entities are returned
    assert api_sol_stats.fleets.keys() == {api_fleet2.id}
    assert api_sol_stats.fleets[api_fleet2.id].mass.one() == 2000000
    assert api_sol_stats.fits.keys() == {api_fit2.id}
    assert api_sol_stats.fits[api_fit2.id].mass.one() == 2000000
    assert api_sol_stats.items.keys() == {api_ship2.id}
    assert api_sol_stats.items[api_ship2.id].mass.one() == 2000000


def test_no_options(client, consts):
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_ship_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 1000000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fleet = api_sol.create_fleet()
    api_fit = api_sol.create_fit(fleet_id=api_fleet.id)
    api_fit.set_ship(type_id=eve_ship_id)
    # Action
    api_sol_stats = api_sol.get_stats()
    # Verification - the request is sent without body, and no stats are requested by default
    with check_no_field():
        api_sol_stats.fleets  # ruff:ignore[useless-expression]
    with check_no_field():
        api_sol_stats.fits  # ruff:ignore[useless-expression]
    with check_no_field():
        api_sol_stats.items  # ruff:ignore[useless-expression]
