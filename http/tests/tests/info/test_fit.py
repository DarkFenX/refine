"""
Here we check availability of info of various items via fit info endpoint.
"""

from tests import check_no_field
from tests.fw.util import Absent


def test_fleet(client):
    client.create_sources()
    api_sol = client.create_sol()
    api_fleet = api_sol.create_fleet()
    api_fit = api_sol.create_fit()
    api_fit.change(fleet_id=api_fleet.id)
    # Verification
    assert api_fit.update().fleet == api_fleet.id
    # Action
    api_fit.change(fleet_id=None)
    # Verification
    api_fit.update()
    with check_no_field():
        api_fit.fleet  # noqa: B018


def test_char(client):
    eve_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.set_character(type_id=eve_item_id)
    # Verification
    assert api_fit.update().character.id == api_item.id
    # Action
    api_item.remove()
    # Verification
    api_fit.update()
    with check_no_field():
        api_fit.character  # noqa: B018


def test_skill(client):
    eve_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_skill(type_id=eve_item_id, level=1)
    # Verification
    api_fit.update()
    assert len(api_fit.skills) == 1
    assert api_fit.skills[0].id == api_item.id
    # Action
    api_item.remove()
    # Verification
    api_fit.update()
    with check_no_field():
        api_fit.skills  # noqa: B018


def test_implant(client):
    eve_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_implant(type_id=eve_item_id)
    # Verification
    api_fit.update()
    assert len(api_fit.implants) == 1
    assert api_fit.implants[0].id == api_item.id
    # Action
    api_item.remove()
    # Verification
    api_fit.update()
    with check_no_field():
        api_fit.implants  # noqa: B018


def test_booster(client):
    eve_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_booster(type_id=eve_item_id)
    # Verification
    api_fit.update()
    assert len(api_fit.boosters) == 1
    assert api_fit.boosters[0].id == api_item.id
    api_item.remove()
    # Verification
    api_fit.update()
    with check_no_field():
        api_fit.boosters  # noqa: B018


def test_ship(client):
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    assert api_fit.update().ship.id == api_item.id
    # Action
    api_item.remove()
    # Verification
    api_fit.update()
    with check_no_field():
        api_fit.ship  # noqa: B018


def test_stance(client):
    eve_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.set_stance(type_id=eve_item_id)
    # Verification
    assert api_fit.update().stance.id == api_item.id
    # Action
    api_item.remove()
    # Verification
    api_fit.update()
    with check_no_field():
        api_fit.stance  # noqa: B018


def test_subsystem(client):
    eve_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_subsystem(type_id=eve_item_id)
    # Verification
    api_fit.update()
    assert len(api_fit.subsystems) == 1
    assert api_fit.subsystems[0].id == api_item.id
    # Action
    api_item.remove()
    # Verification
    api_fit.update()
    with check_no_field():
        api_fit.subsystems  # noqa: B018


def test_mod_high(client, consts):
    eve_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(type_id=eve_item_id, rack=consts.ApiRack.high)
    # Verification
    api_fit.update()
    assert len(api_fit.modules) == 1
    assert len(api_fit.modules.high) == 1
    assert api_fit.modules.high[0].id == api_item.id
    # Action
    api_item.remove()
    # Verification
    api_fit.update()
    with check_no_field():
        api_fit.modules  # noqa: B018


def test_mod_mid(client, consts):
    eve_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(type_id=eve_item_id, rack=consts.ApiRack.mid)
    # Verification
    api_fit.update()
    assert len(api_fit.modules) == 1
    assert len(api_fit.modules.mid) == 1
    assert api_fit.modules.mid[0].id == api_item.id
    # Action
    api_item.remove()
    # Verification
    api_fit.update()
    with check_no_field():
        api_fit.modules  # noqa: B018


def test_mod_low(client, consts):
    eve_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(type_id=eve_item_id, rack=consts.ApiRack.low)
    # Verification
    api_fit.update()
    assert len(api_fit.modules) == 1
    assert len(api_fit.modules.low) == 1
    assert api_fit.modules.low[0].id == api_item.id
    # Action
    api_item.remove()
    # Verification
    api_fit.update()
    with check_no_field():
        api_fit.modules  # noqa: B018


def test_rig(client):
    eve_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_rig(type_id=eve_item_id)
    # Verification
    api_fit.update()
    assert len(api_fit.rigs) == 1
    assert api_fit.rigs[0].id == api_item.id
    # Action
    api_item.remove()
    # Verification
    api_fit.update()
    with check_no_field():
        api_fit.rigs  # noqa: B018


def test_drone(client):
    eve_drone_id = client.mk_eve_drone()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    api_fit.update()
    assert len(api_fit.drones) == 1
    assert api_fit.drones[0].id == api_item.id
    # Action
    api_item.remove()
    # Verification
    api_fit.update()
    with check_no_field():
        api_fit.drones  # noqa: B018


def test_fighter(client):
    eve_fighter_id = client.mk_eve_fighter()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    api_fit.update()
    assert len(api_fit.fighters) == 1
    assert api_fit.fighters[0].id == api_item.id
    # Action
    api_item.remove()
    # Verification
    api_fit.update()
    with check_no_field():
        api_fit.fighters  # noqa: B018


def test_fw_effect(client):
    eve_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_fw_effect(type_id=eve_item_id)
    # Verification
    api_fit.update()
    assert len(api_fit.fw_effects) == 1
    assert api_fit.fw_effects[0].id == api_item.id
    # Action
    api_item.remove()
    # Verification
    api_fit.update()
    with check_no_field():
        api_fit.fw_effects  # noqa: B018


def test_error_no_fit_full(client, consts):
    # Send ID in correct format, but there is no fit with such ID
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.get_fit(
        fit_id='1',
        fit_info_mode=consts.ApiFitInfoMode.full,
        item_info_mode=Absent,
        status_code=404,
        json_predicate={'code': 'EXC-002', 'message': 'fit 1 not found'})


def test_error_no_fit_id(client, consts):
    # Send ID in correct format, but there is no fit with such ID
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.get_fit(
        fit_id='1',
        fit_info_mode=consts.ApiFitInfoMode.id,
        item_info_mode=Absent,
        status_code=404,
        json_predicate={'code': 'EXC-002', 'message': 'fit 1 not found'})


def test_error_no_fit_malformed(client):
    # Send ID in incorrect format
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.get_fit(
        fit_id='abc',
        fit_info_mode=Absent,
        item_info_mode=Absent,
        status_code=404,
        json_predicate={'code': 'IDC-001', 'message': 'unable to cast string "abc" to id'})
