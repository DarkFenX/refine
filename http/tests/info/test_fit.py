# Here we check availability of info of various items via fit info endpoint

from pytest import raises


def test_fleet(client):
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fleet = api_ss.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    assert api_fit.update().fleet == api_fleet.id
    # Action
    api_fleet.change(remove_fits=[api_fit.id])
    # Verification
    api_fit.update()
    with raises(AttributeError):
        api_fit.fleet  # pylint: disable=W0104


def test_char(client):
    eve_item = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.set_char(type_id=eve_item.id)
    # Verification
    assert api_fit.update().character.id == api_item.id
    # Action
    api_item.remove()
    # Verification
    api_fit.update()
    with raises(AttributeError):
        api_fit.character  # pylint: disable=W0104


def test_skill(client):
    eve_item = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_skill(type_id=eve_item.id, level=1)
    # Verification
    api_fit.update()
    assert len(api_fit.skills) == 1
    assert api_fit.skills[0].id == api_item.id
    # Action
    api_item.remove()
    # Verification
    api_fit.update()
    with raises(AttributeError):
        api_fit.skills  # pylint: disable=W0104


def test_implant(client):
    eve_item = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_implant(type_id=eve_item.id)
    # Verification
    api_fit.update()
    assert len(api_fit.implants) == 1
    assert api_fit.implants[0].id == api_item.id
    # Action
    api_item.remove()
    # Verification
    api_fit.update()
    with raises(AttributeError):
        api_fit.implants  # pylint: disable=W0104


def test_booster(client):
    eve_item = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_booster(type_id=eve_item.id)
    # Verification
    api_fit.update()
    assert len(api_fit.boosters) == 1
    assert api_fit.boosters[0].id == api_item.id
    api_item.remove()
    # Verification
    api_fit.update()
    with raises(AttributeError):
        api_fit.boosters  # pylint: disable=W0104


def test_ship(client):
    eve_item = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.set_ship(type_id=eve_item.id)
    # Verification
    assert api_fit.update().ship.id == api_item.id
    # Action
    api_item.remove()
    # Verification
    api_fit.update()
    with raises(AttributeError):
        api_fit.ship  # pylint: disable=W0104


def test_struct(client):
    eve_item = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.set_struct(type_id=eve_item.id)
    # Verification
    assert api_fit.update().structure.id == api_item.id
    # Action
    api_item.remove()
    # Verification
    api_fit.update()
    with raises(AttributeError):
        api_fit.structure  # pylint: disable=W0104


def test_stance(client):
    eve_item = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.set_stance(type_id=eve_item.id)
    # Verification
    assert api_fit.update().stance.id == api_item.id
    # Action
    api_item.remove()
    # Verification
    api_fit.update()
    with raises(AttributeError):
        api_fit.stance  # pylint: disable=W0104


def test_subsystem(client):
    eve_item = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_subsystem(type_id=eve_item.id)
    # Verification
    api_fit.update()
    assert len(api_fit.subsystems) == 1
    assert api_fit.subsystems[0].id == api_item.id
    # Action
    api_item.remove()
    # Verification
    api_fit.update()
    with raises(AttributeError):
        api_fit.subsystems  # pylint: disable=W0104


def test_mod_high(client, consts):
    eve_item = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_mod(type_id=eve_item.id, rack=consts.ApiRack.high)
    # Verification
    api_fit.update()
    assert len(api_fit.modules) == 1
    assert len(api_fit.modules.high) == 1
    assert api_fit.modules.high[0].id == api_item.id
    # Action
    api_item.remove()
    # Verification
    api_fit.update()
    with raises(AttributeError):
        api_fit.modules  # pylint: disable=W0104


def test_mod_mid(client, consts):
    eve_item = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_mod(type_id=eve_item.id, rack=consts.ApiRack.mid)
    # Verification
    api_fit.update()
    assert len(api_fit.modules) == 1
    assert len(api_fit.modules.mid) == 1
    assert api_fit.modules.mid[0].id == api_item.id
    # Action
    api_item.remove()
    # Verification
    api_fit.update()
    with raises(AttributeError):
        api_fit.modules  # pylint: disable=W0104


def test_mod_low(client, consts):
    eve_item = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_mod(type_id=eve_item.id, rack=consts.ApiRack.low)
    # Verification
    api_fit.update()
    assert len(api_fit.modules) == 1
    assert len(api_fit.modules.low) == 1
    assert api_fit.modules.low[0].id == api_item.id
    # Action
    api_item.remove()
    # Verification
    api_fit.update()
    with raises(AttributeError):
        api_fit.modules  # pylint: disable=W0104


def test_rig(client):
    eve_item = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_rig(type_id=eve_item.id)
    # Verification
    api_fit.update()
    assert len(api_fit.rigs) == 1
    assert api_fit.rigs[0].id == api_item.id
    # Action
    api_item.remove()
    # Verification
    api_fit.update()
    with raises(AttributeError):
        api_fit.rigs  # pylint: disable=W0104


def test_drone(client):
    eve_item = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_drone(type_id=eve_item.id)
    # Verification
    api_fit.update()
    assert len(api_fit.drones) == 1
    assert api_fit.drones[0].id == api_item.id
    # Action
    api_item.remove()
    # Verification
    api_fit.update()
    with raises(AttributeError):
        api_fit.drones  # pylint: disable=W0104


def test_fighter(client):
    eve_item = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_fighter(type_id=eve_item.id)
    # Verification
    api_fit.update()
    assert len(api_fit.fighters) == 1
    assert api_fit.fighters[0].id == api_item.id
    # Action
    api_item.remove()
    # Verification
    api_fit.update()
    with raises(AttributeError):
        api_fit.fighters  # pylint: disable=W0104


def test_fw_effect(client):
    eve_item = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_fw_effect(type_id=eve_item.id)
    # Verification
    api_fit.update()
    assert len(api_fit.fw_effects) == 1
    assert api_fit.fw_effects[0].id == api_item.id
    # Action
    api_item.remove()
    # Verification
    api_fit.update()
    with raises(AttributeError):
        api_fit.fw_effects  # pylint: disable=W0104


def test_error_no_fit_full(client, consts):
    # Send ID in correct format, but there is no fit with such ID
    client.create_sources()
    api_ss = client.create_ss()
    resp = api_ss.get_fit_request(fit_id='1', fit_info_mode=consts.ApiFitInfoMode.full).send()
    resp.check(status_code=404, json_predicate={'code': 'COR-003', 'message': 'core library error: fit 1 not found'})


def test_error_no_fit_id(client, consts):
    # Send ID in correct format, but there is no fit with such ID
    client.create_sources()
    api_ss = client.create_ss()
    resp = api_ss.get_fit_request(fit_id='1', fit_info_mode=consts.ApiFitInfoMode.id).send()
    resp.check(status_code=404, json_predicate={'code': 'COR-003', 'message': 'core library error: fit 1 not found'})


def test_error_no_fit_malformed(client):
    # Send ID in incorrect format
    client.create_sources()
    api_ss = client.create_ss()
    resp = api_ss.get_fit_request(fit_id='abc').send()
    resp.check(status_code=404, json_predicate={'code': 'IDC-001', 'message': 'unable to cast string "abc" to id'})
