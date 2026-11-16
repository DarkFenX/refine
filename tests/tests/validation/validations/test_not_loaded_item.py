from fw.api import ValOptions


def test_item_booster(client, consts):
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
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_not_loaded.id: consts.ApiValItemStatus.failed}


def test_item_booster_switch_type_id(client, consts):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_booster = api_fit.add_booster(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed
    # Action
    api_booster.change_booster(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_booster.id: consts.ApiValItemStatus.failed}
    # Action
    api_booster.change_booster(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed


def test_item_character(client, consts):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed
    # Action
    api_char = api_fit.set_character(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_char.id: consts.ApiValItemStatus.failed}


def test_item_character_switch_type_id(client, consts):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_char = api_fit.set_character(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed
    # Action
    api_char.change_character(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_char.id: consts.ApiValItemStatus.failed}
    # Action
    api_char.change_character(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed


def test_item_charge(client, consts):
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
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_not_loaded.charge.id: consts.ApiValItemStatus.failed}


def test_item_charge_switch_type_id(client, consts):
    eve_module_id = client.mk_eve_item()
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed
    # Action
    api_module.charge.change_charge(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_module.charge.id: consts.ApiValItemStatus.failed}
    # Action
    api_module.charge.change_charge(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed


def test_item_drone(client, consts):
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
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_not_loaded.id: consts.ApiValItemStatus.failed}


def test_item_drone_switch_type_id(client, consts):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed
    # Action
    api_drone.change_drone(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_drone.id: consts.ApiValItemStatus.failed}
    # Action
    api_drone.change_drone(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed


def test_item_fighter(client, consts):
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
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_not_loaded.id: consts.ApiValItemStatus.failed}


def test_item_fighter_switch_type_id(client, consts):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed
    # Action
    api_fighter.change_fighter(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_fighter.id: consts.ApiValItemStatus.failed}
    # Action
    api_fighter.change_fighter(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed


def test_item_fw_effect(client, consts):
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
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_not_loaded.id: consts.ApiValItemStatus.failed}


def test_item_fw_effect_switch_type_id(client, consts):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed
    # Action
    api_fw_effect.change_fw_effect(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_fw_effect.id: consts.ApiValItemStatus.failed}
    # Action
    api_fw_effect.change_fw_effect(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed


def test_item_implant(client, consts):
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
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_not_loaded.id: consts.ApiValItemStatus.failed}


def test_item_implant_switch_type_id(client, consts):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_implant = api_fit.add_implant(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed
    # Action
    api_implant.change_implant(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_implant.id: consts.ApiValItemStatus.failed}
    # Action
    api_implant.change_implant(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed


def test_item_module_high(client, consts):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_loaded_id, rack=consts.Rack.high)
    api_not_loaded = api_fit.add_module(type_id=eve_not_loaded_id, rack=consts.Rack.high)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_not_loaded.id: consts.ApiValItemStatus.failed}


def test_item_module_high_switch_type_id(client, consts):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_loaded_id, rack=consts.Rack.high)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed
    # Action
    api_module.change_module(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_module.id: consts.ApiValItemStatus.failed}
    # Action
    api_module.change_module(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed


def test_item_module_low(client, consts):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_loaded_id, rack=consts.Rack.low)
    api_not_loaded = api_fit.add_module(type_id=eve_not_loaded_id, rack=consts.Rack.low)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_not_loaded.id: consts.ApiValItemStatus.failed}


def test_item_module_low_switch_type_id(client, consts):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_loaded_id, rack=consts.Rack.low)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed
    # Action
    api_module.change_module(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_module.id: consts.ApiValItemStatus.failed}
    # Action
    api_module.change_module(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed


def test_item_module_mid(client, consts):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_loaded_id, rack=consts.Rack.mid)
    api_not_loaded = api_fit.add_module(type_id=eve_not_loaded_id, rack=consts.Rack.mid)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_not_loaded.id: consts.ApiValItemStatus.failed}


def test_item_module_mid_switch_type_id(client, consts):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_loaded_id, rack=consts.Rack.mid)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed
    # Action
    api_module.change_module(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_module.id: consts.ApiValItemStatus.failed}
    # Action
    api_module.change_module(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed


def test_item_proj_effect(client, consts):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.add_proj_effect(type_id=eve_loaded_id)
    api_not_loaded = api_sol.add_proj_effect(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_sol.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_not_loaded.id: consts.ApiValItemStatus.failed}


def test_item_proj_effect_switch_type_id(client, consts):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_loaded_id)
    # Verification
    api_val = api_sol.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed
    # Action
    api_proj_effect.change_proj_effect(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_sol.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_proj_effect.id: consts.ApiValItemStatus.failed}
    # Action
    api_proj_effect.change_proj_effect(type_id=eve_loaded_id)
    # Verification
    api_val = api_sol.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed


def test_item_rig(client, consts):
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
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_not_loaded.id: consts.ApiValItemStatus.failed}


def test_item_rig_switch_type_id(client, consts):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_rig = api_fit.add_rig(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed
    # Action
    api_rig.change_rig(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_rig.id: consts.ApiValItemStatus.failed}
    # Action
    api_rig.change_rig(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed


def test_item_service(client, consts):
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
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_not_loaded.id: consts.ApiValItemStatus.failed}


def test_item_service_switch_type_id(client, consts):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_service = api_fit.add_service(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed
    # Action
    api_service.change_service(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_service.id: consts.ApiValItemStatus.failed}
    # Action
    api_service.change_service(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed


def test_item_ship(client, consts):
    eve_loaded_id = client.mk_eve_ship()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed
    # Action
    api_ship = api_fit.set_ship(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_ship.id: consts.ApiValItemStatus.failed}


def test_item_ship_switch_type_id(client, consts):
    eve_loaded_id = client.mk_eve_ship()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed
    # Action
    api_ship.change_ship(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_ship.id: consts.ApiValItemStatus.failed}
    # Action
    api_ship.change_ship(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed


def test_item_skill(client, consts):
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
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_not_loaded.id: consts.ApiValItemStatus.failed}


def test_item_skill_switch_type_id(client, consts):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_skill = api_fit.add_skill(type_id=eve_loaded_id, level=1)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed
    # Action
    api_skill.change_skill(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_skill.id: consts.ApiValItemStatus.failed}
    # Action
    api_skill.change_skill(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed


def test_item_stance(client, consts):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_stance(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed
    # Action
    api_stance = api_fit.set_stance(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_stance.id: consts.ApiValItemStatus.failed}


def test_item_stance_switch_type_id(client, consts):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_stance = api_fit.set_stance(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed
    # Action
    api_stance.change_stance(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_stance.id: consts.ApiValItemStatus.failed}
    # Action
    api_stance.change_stance(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed


def test_item_subsystem(client, consts):
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
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_not_loaded.id: consts.ApiValItemStatus.failed}


def test_item_subsystem_switch_type_id(client, consts):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_subsystem = api_fit.add_subsystem(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed
    # Action
    api_subsystem.change_subsystem(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_subsystem.id: consts.ApiValItemStatus.failed}
    # Action
    api_subsystem.change_subsystem(type_id=eve_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed


def test_item_sw_effect(client, consts):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.add_sw_effect(type_id=eve_loaded_id)
    api_not_loaded = api_sol.add_sw_effect(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_sol.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_not_loaded.id: consts.ApiValItemStatus.failed}


def test_item_sw_effect_switch_type_id(client, consts):
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_sw_effect = api_sol.add_sw_effect(type_id=eve_loaded_id)
    # Verification
    api_val = api_sol.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed
    # Action
    api_sw_effect.change_sw_effect(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_sol.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_sw_effect.id: consts.ApiValItemStatus.failed}
    # Action
    api_sw_effect.change_sw_effect(type_id=eve_loaded_id)
    # Verification
    api_val = api_sol.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed


def test_item_dynamic_bare(client, consts):
    # Dynamic types are loaded only as mutated items, not on their own
    eve_base_item_id = client.mk_eve_item()
    eve_mutated_item_id = client.mk_eve_item(is_dynamic=True)
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_item_id], eve_mutated_item_id)])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_base_item_id, mutation=eve_mutator_id)
    api_not_loaded = api_fit.add_module(type_id=eve_mutated_item_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_not_loaded.id: consts.ApiValItemStatus.failed}


def test_item_dynamic_switch_type_id(client, consts):
    eve_base_item_id = client.mk_eve_item()
    eve_mutated_item_id = client.mk_eve_item(is_dynamic=True)
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_item_id], eve_mutated_item_id)])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_base_item_id, mutation=eve_mutator_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed
    # Action
    api_module.change_module(type_id=eve_mutated_item_id)
    # Verification - mutator has no mutation for a dynamic base type, which is not loaded on its own
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {api_module.id: consts.ApiValItemStatus.failed}
    # Action
    api_module.change_module(type_id=eve_base_item_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed


def test_known_failures(client, consts):
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
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=[api_not_loaded1.id]))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {
        api_not_loaded1.id: consts.ApiValItemStatus.kf,
        api_not_loaded2.id: consts.ApiValItemStatus.failed}
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=[api_not_loaded2.id]))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {
        api_not_loaded1.id: consts.ApiValItemStatus.failed,
        api_not_loaded2.id: consts.ApiValItemStatus.kf}
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=[api_not_loaded1.id, api_not_loaded2.id]))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed_kfs
    assert api_val.details.not_loaded_item.fail == {
        api_not_loaded1.id: consts.ApiValItemStatus.kf,
        api_not_loaded2.id: consts.ApiValItemStatus.kf}
    api_val = api_fit.validate(options=ValOptions(
        not_loaded_item=[api_not_loaded1.id, api_other.id, api_not_loaded2.id]))
    assert api_val.passed is True
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.passed_kfs
    assert api_val.details.not_loaded_item.fail == {
        api_not_loaded1.id: consts.ApiValItemStatus.kf,
        api_not_loaded2.id: consts.ApiValItemStatus.kf}


def test_state(client, consts):
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_implant = api_fit.add_implant(type_id=eve_not_loaded_id, state=False)
    api_ship = api_fit.set_ship(type_id=eve_not_loaded_id, state=False)
    api_module = api_fit.add_module(
        type_id=eve_not_loaded_id,
        state=consts.ApiModuleState.disabled,
        charge_type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item.status == consts.ApiValStatus.failed
    assert api_val.details.not_loaded_item.fail == {
        api_implant.id: consts.ApiValItemStatus.failed,
        api_ship.id: consts.ApiValItemStatus.failed,
        api_module.id: consts.ApiValItemStatus.failed,
        api_module.charge.id: consts.ApiValItemStatus.failed}
