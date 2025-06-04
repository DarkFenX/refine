from tests import check_no_field
from tests.fw.api import FitValOptions


def test_booster(client):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_booster(type_id=eve_loaded_id)
    api_not_loaded = api_fit.add_booster(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_not_loaded.id]


def test_booster_switch_type_id(client):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_booster = api_fit.add_booster(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_booster.change_booster(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_booster.id]
    # Action
    api_booster.change_booster(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_character(client):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_char = api_fit.set_character(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_char.id]


def test_character_switch_type_id(client):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_char = api_fit.set_character(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_char.change_character(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_char.id]
    # Action
    api_char.change_character(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


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
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_not_loaded.charge.id]


def test_charge_switch_type_id(client):
    eve_module_id = client.mk_eve_item()
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.charge.change_charge(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_module.charge.id]
    # Action
    api_module.charge.change_charge(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_drone(client):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_drone(type_id=eve_loaded_id)
    api_not_loaded = api_fit.add_drone(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_not_loaded.id]


def test_drone_switch_type_id(client):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_drone.change_drone(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_drone.id]
    # Action
    api_drone.change_drone(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_fighter(client):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_fighter(type_id=eve_loaded_id)
    api_not_loaded = api_fit.add_fighter(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_not_loaded.id]


def test_fighter_switch_type_id(client):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fighter.change_fighter(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_fighter.id]
    # Action
    api_fighter.change_fighter(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_fw_effect(client):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_fw_effect(type_id=eve_loaded_id)
    api_not_loaded = api_fit.add_fw_effect(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_not_loaded.id]


def test_fw_effect_switch_type_id(client):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fw_effect.change_fw_effect(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_fw_effect.id]
    # Action
    api_fw_effect.change_fw_effect(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_implant(client):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_implant(type_id=eve_loaded_id)
    api_not_loaded = api_fit.add_implant(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_not_loaded.id]


def test_implant_switch_type_id(client):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_implant = api_fit.add_implant(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_implant.change_implant(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_implant.id]
    # Action
    api_implant.change_implant(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_module_high(client, consts):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_loaded_id, rack=consts.ApiRack.high)
    api_not_loaded = api_fit.add_module(type_id=eve_not_loaded_id, rack=consts.ApiRack.high)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_not_loaded.id]


def test_module_high_switch_type_id(client, consts):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_loaded_id, rack=consts.ApiRack.high)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_module(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_module.id]
    # Action
    api_module.change_module(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_module_low(client, consts):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_loaded_id, rack=consts.ApiRack.low)
    api_not_loaded = api_fit.add_module(type_id=eve_not_loaded_id, rack=consts.ApiRack.low)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_not_loaded.id]


def test_module_low_switch_type_id(client, consts):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_loaded_id, rack=consts.ApiRack.low)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_module(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_module.id]
    # Action
    api_module.change_module(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_module_mid(client, consts):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_loaded_id, rack=consts.ApiRack.mid)
    api_not_loaded = api_fit.add_module(type_id=eve_not_loaded_id, rack=consts.ApiRack.mid)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_not_loaded.id]


def test_module_mid_switch_type_id(client, consts):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_loaded_id, rack=consts.ApiRack.mid)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_module(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_module.id]
    # Action
    api_module.change_module(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_rig(client):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_rig(type_id=eve_loaded_id)
    api_not_loaded = api_fit.add_rig(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_not_loaded.id]


def test_rig_switch_type_id(client):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_rig = api_fit.add_rig(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_rig.change_rig(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_rig.id]
    # Action
    api_rig.change_rig(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_service(client):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_service(type_id=eve_loaded_id)
    api_not_loaded = api_fit.add_service(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_not_loaded.id]


def test_service_switch_type_id(client):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_service = api_fit.add_service(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_service.change_service(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_service.id]
    # Action
    api_service.change_service(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_ship(client):
    eve_loaded_id = client.mk_eve_ship()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_ship = api_fit.set_ship(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_ship.id]


def test_ship_switch_type_id(client):
    eve_loaded_id = client.mk_eve_ship()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_ship.change_ship(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_ship.id]
    # Action
    api_ship.change_ship(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_skill(client):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_skill(type_id=eve_loaded_id, level=1)
    api_not_loaded = api_fit.add_skill(type_id=eve_not_loaded_id, level=1)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_not_loaded.id]


def test_skill_switch_type_id(client):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_skill = api_fit.add_skill(type_id=eve_loaded_id, level=1)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_skill.change_skill(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_skill.id]
    # Action
    api_skill.change_skill(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_stance(client):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_stance(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_stance = api_fit.set_stance(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_stance.id]


def test_stance_switch_type_id(client):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_stance = api_fit.set_stance(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_stance.change_stance(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_stance.id]
    # Action
    api_stance.change_stance(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_subsystem(client):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_subsystem(type_id=eve_loaded_id)
    api_not_loaded = api_fit.add_subsystem(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_not_loaded.id]


def test_subsystem_switch_type_id(client):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_subsystem = api_fit.add_subsystem(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_subsystem.change_subsystem(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_subsystem.id]
    # Action
    api_subsystem.change_subsystem(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


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
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=(True, [api_not_loaded1.id])))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_not_loaded2.id]
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=(True, [api_not_loaded2.id])))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_not_loaded1.id]
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=(True, [api_not_loaded1.id, api_not_loaded2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(options=FitValOptions(
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
    api_val = api_fit.validate(options=FitValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == sorted([api_implant.id, api_ship.id, api_module.id, api_module.charge.id])
