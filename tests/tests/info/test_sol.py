"""
Here we check availability of info of various items via solar system info endpoint.
"""
from fw import check_no_field
from fw.util import Absent


def test_error_params_malformed(client):
    client.create_sources()
    api_sol = client.create_sol()
    # Verification
    api_sol.update(
        sol_info_mode='random',
        status_code=400,
        json_predicate={'code': 'PRM-001', 'message': 're:.+'})
    api_sol.update(
        fleet_info_mode='random',
        status_code=400,
        json_predicate={'code': 'PRM-001', 'message': 're:.+'})
    api_sol.update(
        fit_info_mode='random',
        status_code=400,
        json_predicate={'code': 'PRM-001', 'message': 're:.+'})
    api_sol.update(
        item_info_mode='random',
        status_code=400,
        json_predicate={'code': 'PRM-001', 'message': 're:.+'})


def test_src_alias(client, consts):
    eve_data = client.mk_eve_data()
    client.create_sources()
    api_sol = client.create_sol(data=eve_data)
    # Verification
    assert api_sol.src_alias == eve_data.alias
    assert api_sol.update(sol_info_mode=consts.ApiSolInfoMode.id).src_alias == eve_data.alias
    assert api_sol.update(sol_info_mode=consts.ApiSolInfoMode.full).src_alias == eve_data.alias


def test_fleet(client, consts):
    eve_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fleet = api_sol.create_fleet()
    api_fit = api_sol.create_fit(fleet_id=api_fleet.id)
    api_item = api_fit.set_character(type_id=eve_item_id)
    api_sol.update(
        sol_info_mode=consts.ApiSolInfoMode.full,
        fleet_info_mode=consts.ApiFleetInfoMode.full,
        fit_info_mode=consts.ApiFitInfoMode.full,
        item_info_mode=consts.ApiItemInfoMode.id)
    # Fleet info contains fit info, but fit info has just IDs in the fleet container
    assert len(api_sol.fleets) == 1
    assert api_fleet.id in api_sol.fleets
    assert api_sol.fleets[api_fleet.id].id == api_fleet.id
    api_fit_via_fleet = api_sol.fleets[api_fleet.id].fits[api_fit.id]
    with check_no_field():
        api_fit_via_fleet.character  # ruff:ignore[useless-expression]
    # But fit has full info as requested in the fit container
    assert api_sol.fits[api_fit.id].character.id == api_item.id
    # Remove fleet and check that fleets container is empty
    api_fleet.remove()
    api_sol.update()
    with check_no_field():
        api_sol.fleets  # ruff:ignore[useless-expression]


def test_fit(client):
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_sol.update()
    assert len(api_sol.fits) == 1
    assert api_fit.id in api_sol.fits
    assert api_sol.fits[api_fit.id].id == api_fit.id
    api_fit.remove()
    api_sol.update()
    with check_no_field():
        api_sol.fits  # ruff:ignore[useless-expression]


def test_fit_item(client):
    eve_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.set_character(type_id=eve_item_id)
    api_sol.update()
    assert len(api_sol.fits) == 1
    assert api_fit.id in api_sol.fits
    assert api_sol.fits[api_fit.id].character.id == api_item.id
    api_item.remove()
    api_fit = api_sol.update().fits[api_fit.id]
    with check_no_field():
        api_fit.character  # ruff:ignore[useless-expression]


def test_sw_effect(client):
    eve_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_item = api_sol.add_sw_effect(type_id=eve_item_id)
    api_sol.update()
    assert len(api_sol.sw_effects) == 1
    assert api_sol.sw_effects[0].id == api_item.id
    api_item.remove()
    api_sol.update()
    with check_no_field():
        api_sol.sw_effects  # ruff:ignore[useless-expression]


def test_proj_effect(client):
    eve_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_item = api_sol.add_proj_effect(type_id=eve_item_id)
    api_sol.update()
    assert len(api_sol.proj_effects) == 1
    assert api_sol.proj_effects[0].id == api_item.id
    api_item.remove()
    api_sol.update()
    with check_no_field():
        api_sol.proj_effects  # ruff:ignore[useless-expression]


def test_error_no_sol_full(client, consts):
    # Check case when there is no solar system with such ID
    client.create_sources()
    client.get_sol(
        sol_id='1',
        sol_info_mode=consts.ApiSolInfoMode.full,
        fleet_info_mode=Absent,
        fit_info_mode=Absent,
        item_info_mode=Absent,
        status_code=404,
        json_predicate={'code': 'SOL-002', 'message': 'failed to parse a UUID'})


def test_error_no_sol_id(client, consts):
    # Check case when there is no solar system with such ID
    client.create_sources()
    client.get_sol(
        sol_id='1',
        sol_info_mode=consts.ApiSolInfoMode.id,
        fleet_info_mode=Absent,
        fit_info_mode=Absent,
        item_info_mode=Absent,
        status_code=404,
        json_predicate={'code': 'SOL-002', 'message': 'failed to parse a UUID'})
