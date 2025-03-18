from tests import check_no_field
from tests.fw.api import ValOptions


def test_booster(client):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_booster(type_id=eve_loaded_id)
    api_not_loaded = api_fit.add_booster(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_not_loaded.id]


def test_character(client):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_char(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_char = api_fit.set_char(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_char.id]


def test_charge(client):
    eve_module_id = client.mk_eve_item()
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_loaded_id)
    api_not_loaded = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_not_loaded.charge.id]


def test_drone(client):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_drone(type_id=eve_loaded_id)
    api_not_loaded = api_fit.add_drone(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_not_loaded.id]


def test_fighter(client):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_fighter(type_id=eve_loaded_id)
    api_not_loaded = api_fit.add_fighter(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_not_loaded.id]


def test_fw_effect(client):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_fw_effect(type_id=eve_loaded_id)
    api_not_loaded = api_fit.add_fw_effect(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_not_loaded.id]


def test_implant(client):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_implant(type_id=eve_loaded_id)
    api_not_loaded = api_fit.add_implant(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_not_loaded.id]


def test_module_high(client, consts):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_loaded_id, rack=consts.ApiRack.high)
    api_not_loaded = api_fit.add_module(type_id=eve_not_loaded_id, rack=consts.ApiRack.high)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_not_loaded.id]


def test_module_low(client, consts):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_loaded_id, rack=consts.ApiRack.low)
    api_not_loaded = api_fit.add_module(type_id=eve_not_loaded_id, rack=consts.ApiRack.low)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_not_loaded.id]


def test_module_mid(client, consts):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_loaded_id, rack=consts.ApiRack.mid)
    api_not_loaded = api_fit.add_module(type_id=eve_not_loaded_id, rack=consts.ApiRack.mid)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_not_loaded.id]


def test_rig(client):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_rig(type_id=eve_loaded_id)
    api_not_loaded = api_fit.add_rig(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_not_loaded.id]


def test_service(client):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_service(type_id=eve_loaded_id)
    api_not_loaded = api_fit.add_service(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_not_loaded.id]


def test_ship(client):
    eve_loaded_id = client.mk_eve_ship()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_ship = api_fit.set_ship(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_ship.id]


def test_skill(client):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_skill(type_id=eve_loaded_id, level=1)
    api_not_loaded = api_fit.add_skill(type_id=eve_not_loaded_id, level=1)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_not_loaded.id]


def test_stance(client):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_stance(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_stance = api_fit.set_stance(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_stance.id]


def test_subsystem(client):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_subsystem(type_id=eve_loaded_id)
    api_not_loaded = api_fit.add_subsystem(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_not_loaded.id]


def test_known_failures(client):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    eve_other_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_other = api_fit.add_implant(type_id=eve_other_id)
    api_fit.add_module(type_id=eve_loaded_id)
    api_not_loaded1 = api_fit.add_module(type_id=eve_not_loaded_id)
    api_not_loaded2 = api_fit.add_module(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=(True, [api_not_loaded1.id])))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_not_loaded2.id]
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=(True, [api_not_loaded2.id])))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_not_loaded1.id]
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=(True, [api_not_loaded1.id, api_not_loaded2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(options=ValOptions(
        not_loaded_item=(True, [api_not_loaded1.id, api_other.id, api_not_loaded2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_state(client, consts):
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_implant = api_fit.add_implant(type_id=eve_not_loaded_id, state=False)
    api_ship = api_fit.set_ship(type_id=eve_not_loaded_id, state=False)
    api_module = api_fit.add_module(
        type_id=eve_not_loaded_id,
        state=consts.ApiModuleState.ghost,
        charge_type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == sorted([api_implant.id, api_ship.id, api_module.id, api_module.charge.id])
