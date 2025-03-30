from tests import check_no_field
from tests.fw.api import ValOptions


def test_ship_kind_switching(client, consts):
    eve_ship_item_id = client.mk_eve_item(cat_id=consts.EveItemCat.module)
    eve_struct_item_id = client.mk_eve_item(cat_id=consts.EveItemCat.structure_module)
    eve_other_item_id = client.mk_eve_item(cat_id=consts.EveItemCat.subsystem)
    eve_ship_id = client.mk_eve_ship()
    eve_struct_id = client.mk_eve_struct()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship_module_high = api_fit.add_module(type_id=eve_ship_item_id, rack=consts.ApiRack.high)
    api_ship_module_mid = api_fit.add_module(type_id=eve_ship_item_id, rack=consts.ApiRack.mid)
    api_ship_module_low = api_fit.add_module(type_id=eve_ship_item_id, rack=consts.ApiRack.low)
    api_ship_rig = api_fit.add_rig(type_id=eve_ship_item_id)
    api_ship_service = api_fit.add_service(type_id=eve_ship_item_id)
    api_struct_module_high = api_fit.add_module(type_id=eve_struct_item_id, rack=consts.ApiRack.high)
    api_struct_module_mid = api_fit.add_module(type_id=eve_struct_item_id, rack=consts.ApiRack.mid)
    api_struct_module_low = api_fit.add_module(type_id=eve_struct_item_id, rack=consts.ApiRack.low)
    api_struct_rig = api_fit.add_rig(type_id=eve_struct_item_id)
    api_struct_service = api_fit.add_service(type_id=eve_struct_item_id)
    api_fit.add_module(type_id=eve_other_item_id, rack=consts.ApiRack.high)
    api_fit.add_module(type_id=eve_other_item_id, rack=consts.ApiRack.mid)
    api_fit.add_module(type_id=eve_other_item_id, rack=consts.ApiRack.low)
    api_fit.add_rig(type_id=eve_other_item_id)
    api_fit.add_service(type_id=eve_other_item_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(item_vs_ship_kind=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(item_vs_ship_kind=True))
    assert api_val.passed is False
    assert api_val.details.item_vs_ship_kind.ship_kind == consts.ApiValShipType.ship
    assert api_val.details.item_vs_ship_kind.items == {
        api_struct_module_high.id: consts.ApiValShipType.structure,
        api_struct_module_mid.id: consts.ApiValShipType.structure,
        api_struct_module_low.id: consts.ApiValShipType.structure,
        api_struct_rig.id: consts.ApiValShipType.structure,
        api_struct_service.id: consts.ApiValShipType.structure}
    # Action
    api_ship.remove()
    # Verification
    api_val = api_fit.validate(options=ValOptions(item_vs_ship_kind=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_ship(type_id=eve_struct_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(item_vs_ship_kind=True))
    assert api_val.passed is False
    assert api_val.details.item_vs_ship_kind.ship_kind == consts.ApiValShipType.structure
    assert api_val.details.item_vs_ship_kind.items == {
        api_ship_module_high.id: consts.ApiValShipType.ship,
        api_ship_module_mid.id: consts.ApiValShipType.ship,
        api_ship_module_low.id: consts.ApiValShipType.ship,
        api_ship_rig.id: consts.ApiValShipType.ship,
        api_ship_service.id: consts.ApiValShipType.ship}
    # Action
    api_fit.set_ship(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(item_vs_ship_kind=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_ship(type_id=eve_other_item_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(item_vs_ship_kind=True))
    assert api_val.passed is False
    assert api_val.details.item_vs_ship_kind.ship_kind == consts.ApiValShipType.unknown
    assert api_val.details.item_vs_ship_kind.items == {
        api_ship_module_high.id: consts.ApiValShipType.ship,
        api_ship_module_mid.id: consts.ApiValShipType.ship,
        api_ship_module_low.id: consts.ApiValShipType.ship,
        api_ship_rig.id: consts.ApiValShipType.ship,
        api_ship_service.id: consts.ApiValShipType.ship,
        api_struct_module_high.id: consts.ApiValShipType.structure,
        api_struct_module_mid.id: consts.ApiValShipType.structure,
        api_struct_module_low.id: consts.ApiValShipType.structure,
        api_struct_rig.id: consts.ApiValShipType.structure,
        api_struct_service.id: consts.ApiValShipType.structure}


def test_item_add_remove_ship_ship(client, consts):
    eve_ship_item_id = client.mk_eve_item(cat_id=consts.EveItemCat.module)
    eve_struct_item_id = client.mk_eve_item(cat_id=consts.EveItemCat.structure_module)
    eve_other_item_id = client.mk_eve_item(cat_id=consts.EveItemCat.subsystem)
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_ship_module_high = api_fit.add_module(type_id=eve_ship_item_id, rack=consts.ApiRack.high)
    api_ship_module_mid = api_fit.add_module(type_id=eve_ship_item_id, rack=consts.ApiRack.mid)
    api_ship_module_low = api_fit.add_module(type_id=eve_ship_item_id, rack=consts.ApiRack.low)
    api_ship_rig = api_fit.add_rig(type_id=eve_ship_item_id)
    api_ship_service = api_fit.add_service(type_id=eve_ship_item_id)
    api_struct_module_high = api_fit.add_module(type_id=eve_struct_item_id, rack=consts.ApiRack.high)
    api_struct_module_mid = api_fit.add_module(type_id=eve_struct_item_id, rack=consts.ApiRack.mid)
    api_struct_module_low = api_fit.add_module(type_id=eve_struct_item_id, rack=consts.ApiRack.low)
    api_struct_rig = api_fit.add_rig(type_id=eve_struct_item_id)
    api_struct_service = api_fit.add_service(type_id=eve_struct_item_id)
    api_other_module_high = api_fit.add_module(type_id=eve_other_item_id, rack=consts.ApiRack.high)
    api_other_module_mid = api_fit.add_module(type_id=eve_other_item_id, rack=consts.ApiRack.mid)
    api_other_module_low = api_fit.add_module(type_id=eve_other_item_id, rack=consts.ApiRack.low)
    api_other_rig = api_fit.add_rig(type_id=eve_other_item_id)
    api_other_service = api_fit.add_service(type_id=eve_other_item_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(item_vs_ship_kind=True))
    assert api_val.passed is False
    assert api_val.details.item_vs_ship_kind.ship_kind == consts.ApiValShipType.ship
    assert api_val.details.item_vs_ship_kind.items == {
        api_struct_module_high.id: consts.ApiValShipType.structure,
        api_struct_module_mid.id: consts.ApiValShipType.structure,
        api_struct_module_low.id: consts.ApiValShipType.structure,
        api_struct_rig.id: consts.ApiValShipType.structure,
        api_struct_service.id: consts.ApiValShipType.structure}
    # Action
    api_ship_module_high.remove()
    api_ship_module_mid.remove()
    api_ship_module_low.remove()
    api_ship_rig.remove()
    api_ship_service.remove()
    api_struct_module_high.remove()
    api_struct_module_mid.remove()
    api_struct_module_low.remove()
    api_struct_rig.remove()
    api_struct_service.remove()
    api_other_module_high.remove()
    api_other_module_mid.remove()
    api_other_module_low.remove()
    api_other_rig.remove()
    api_other_service.remove()
    # Verification
    api_val = api_fit.validate(options=ValOptions(item_vs_ship_kind=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_item_add_remove_ship_struct(client, consts):
    eve_ship_item_id = client.mk_eve_item(cat_id=consts.EveItemCat.module)
    eve_struct_item_id = client.mk_eve_item(cat_id=consts.EveItemCat.structure_module)
    eve_other_item_id = client.mk_eve_item(cat_id=consts.EveItemCat.subsystem)
    eve_struct_id = client.mk_eve_struct()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct_id)
    api_ship_module_high = api_fit.add_module(type_id=eve_ship_item_id, rack=consts.ApiRack.high)
    api_ship_module_mid = api_fit.add_module(type_id=eve_ship_item_id, rack=consts.ApiRack.mid)
    api_ship_module_low = api_fit.add_module(type_id=eve_ship_item_id, rack=consts.ApiRack.low)
    api_ship_rig = api_fit.add_rig(type_id=eve_ship_item_id)
    api_ship_service = api_fit.add_service(type_id=eve_ship_item_id)
    api_struct_module_high = api_fit.add_module(type_id=eve_struct_item_id, rack=consts.ApiRack.high)
    api_struct_module_mid = api_fit.add_module(type_id=eve_struct_item_id, rack=consts.ApiRack.mid)
    api_struct_module_low = api_fit.add_module(type_id=eve_struct_item_id, rack=consts.ApiRack.low)
    api_struct_rig = api_fit.add_rig(type_id=eve_struct_item_id)
    api_struct_service = api_fit.add_service(type_id=eve_struct_item_id)
    api_other_module_high = api_fit.add_module(type_id=eve_other_item_id, rack=consts.ApiRack.high)
    api_other_module_mid = api_fit.add_module(type_id=eve_other_item_id, rack=consts.ApiRack.mid)
    api_other_module_low = api_fit.add_module(type_id=eve_other_item_id, rack=consts.ApiRack.low)
    api_other_rig = api_fit.add_rig(type_id=eve_other_item_id)
    api_other_service = api_fit.add_service(type_id=eve_other_item_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(item_vs_ship_kind=True))
    assert api_val.passed is False
    assert api_val.details.item_vs_ship_kind.ship_kind == consts.ApiValShipType.structure
    assert api_val.details.item_vs_ship_kind.items == {
        api_ship_module_high.id: consts.ApiValShipType.ship,
        api_ship_module_mid.id: consts.ApiValShipType.ship,
        api_ship_module_low.id: consts.ApiValShipType.ship,
        api_ship_rig.id: consts.ApiValShipType.ship,
        api_ship_service.id: consts.ApiValShipType.ship}
    # Action
    api_ship_module_high.remove()
    api_ship_module_mid.remove()
    api_ship_module_low.remove()
    api_ship_rig.remove()
    api_ship_service.remove()
    api_struct_module_high.remove()
    api_struct_module_mid.remove()
    api_struct_module_low.remove()
    api_struct_rig.remove()
    api_struct_service.remove()
    api_other_module_high.remove()
    api_other_module_mid.remove()
    api_other_module_low.remove()
    api_other_rig.remove()
    api_other_service.remove()
    # Verification
    api_val = api_fit.validate(options=ValOptions(item_vs_ship_kind=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_item_add_remove_ship_unknown(client, consts):
    eve_ship_item_id = client.mk_eve_item(cat_id=consts.EveItemCat.module)
    eve_struct_item_id = client.mk_eve_item(cat_id=consts.EveItemCat.structure_module)
    eve_other_item_id = client.mk_eve_item(cat_id=consts.EveItemCat.subsystem)
    eve_other_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_other_id)
    api_ship_module_high = api_fit.add_module(type_id=eve_ship_item_id, rack=consts.ApiRack.high)
    api_ship_module_mid = api_fit.add_module(type_id=eve_ship_item_id, rack=consts.ApiRack.mid)
    api_ship_module_low = api_fit.add_module(type_id=eve_ship_item_id, rack=consts.ApiRack.low)
    api_ship_rig = api_fit.add_rig(type_id=eve_ship_item_id)
    api_ship_service = api_fit.add_service(type_id=eve_ship_item_id)
    api_struct_module_high = api_fit.add_module(type_id=eve_struct_item_id, rack=consts.ApiRack.high)
    api_struct_module_mid = api_fit.add_module(type_id=eve_struct_item_id, rack=consts.ApiRack.mid)
    api_struct_module_low = api_fit.add_module(type_id=eve_struct_item_id, rack=consts.ApiRack.low)
    api_struct_rig = api_fit.add_rig(type_id=eve_struct_item_id)
    api_struct_service = api_fit.add_service(type_id=eve_struct_item_id)
    api_other_module_high = api_fit.add_module(type_id=eve_other_item_id, rack=consts.ApiRack.high)
    api_other_module_mid = api_fit.add_module(type_id=eve_other_item_id, rack=consts.ApiRack.mid)
    api_other_module_low = api_fit.add_module(type_id=eve_other_item_id, rack=consts.ApiRack.low)
    api_other_rig = api_fit.add_rig(type_id=eve_other_item_id)
    api_other_service = api_fit.add_service(type_id=eve_other_item_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(item_vs_ship_kind=True))
    assert api_val.passed is False
    assert api_val.details.item_vs_ship_kind.ship_kind == consts.ApiValShipType.unknown
    assert api_val.details.item_vs_ship_kind.items == {
        api_ship_module_high.id: consts.ApiValShipType.ship,
        api_ship_module_mid.id: consts.ApiValShipType.ship,
        api_ship_module_low.id: consts.ApiValShipType.ship,
        api_ship_rig.id: consts.ApiValShipType.ship,
        api_ship_service.id: consts.ApiValShipType.ship,
        api_struct_module_high.id: consts.ApiValShipType.structure,
        api_struct_module_mid.id: consts.ApiValShipType.structure,
        api_struct_module_low.id: consts.ApiValShipType.structure,
        api_struct_rig.id: consts.ApiValShipType.structure,
        api_struct_service.id: consts.ApiValShipType.structure}
    # Action
    api_ship_module_high.remove()
    api_ship_module_mid.remove()
    api_ship_module_low.remove()
    api_ship_rig.remove()
    api_ship_service.remove()
    api_struct_module_high.remove()
    api_struct_module_mid.remove()
    api_struct_module_low.remove()
    api_struct_rig.remove()
    api_struct_service.remove()
    api_other_module_high.remove()
    api_other_module_mid.remove()
    api_other_module_low.remove()
    api_other_rig.remove()
    api_other_service.remove()
    # Verification
    api_val = api_fit.validate(options=ValOptions(item_vs_ship_kind=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_item_add_remove_ship_none(client, consts):
    eve_ship_item_id = client.mk_eve_item(cat_id=consts.EveItemCat.module)
    eve_struct_item_id = client.mk_eve_item(cat_id=consts.EveItemCat.structure_module)
    eve_other_item_id = client.mk_eve_item(cat_id=consts.EveItemCat.subsystem)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship_module_high = api_fit.add_module(type_id=eve_ship_item_id, rack=consts.ApiRack.high)
    api_ship_module_mid = api_fit.add_module(type_id=eve_ship_item_id, rack=consts.ApiRack.mid)
    api_ship_module_low = api_fit.add_module(type_id=eve_ship_item_id, rack=consts.ApiRack.low)
    api_ship_rig = api_fit.add_rig(type_id=eve_ship_item_id)
    api_ship_service = api_fit.add_service(type_id=eve_ship_item_id)
    api_struct_module_high = api_fit.add_module(type_id=eve_struct_item_id, rack=consts.ApiRack.high)
    api_struct_module_mid = api_fit.add_module(type_id=eve_struct_item_id, rack=consts.ApiRack.mid)
    api_struct_module_low = api_fit.add_module(type_id=eve_struct_item_id, rack=consts.ApiRack.low)
    api_struct_rig = api_fit.add_rig(type_id=eve_struct_item_id)
    api_struct_service = api_fit.add_service(type_id=eve_struct_item_id)
    api_other_module_high = api_fit.add_module(type_id=eve_other_item_id, rack=consts.ApiRack.high)
    api_other_module_mid = api_fit.add_module(type_id=eve_other_item_id, rack=consts.ApiRack.mid)
    api_other_module_low = api_fit.add_module(type_id=eve_other_item_id, rack=consts.ApiRack.low)
    api_other_rig = api_fit.add_rig(type_id=eve_other_item_id)
    api_other_service = api_fit.add_service(type_id=eve_other_item_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(item_vs_ship_kind=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_ship_module_high.remove()
    api_ship_module_mid.remove()
    api_ship_module_low.remove()
    api_ship_rig.remove()
    api_ship_service.remove()
    api_struct_module_high.remove()
    api_struct_module_mid.remove()
    api_struct_module_low.remove()
    api_struct_rig.remove()
    api_struct_service.remove()
    api_other_module_high.remove()
    api_other_module_mid.remove()
    api_other_module_low.remove()
    api_other_rig.remove()
    api_other_service.remove()
    # Verification
    api_val = api_fit.validate(options=ValOptions(item_vs_ship_kind=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_item_add_remove_ship_not_loaded(client, consts):
    eve_ship_item_id = client.mk_eve_item(cat_id=consts.EveItemCat.module)
    eve_struct_item_id = client.mk_eve_item(cat_id=consts.EveItemCat.structure_module)
    eve_other_item_id = client.mk_eve_item(cat_id=consts.EveItemCat.subsystem)
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_ship_module_high = api_fit.add_module(type_id=eve_ship_item_id, rack=consts.ApiRack.high)
    api_ship_module_mid = api_fit.add_module(type_id=eve_ship_item_id, rack=consts.ApiRack.mid)
    api_ship_module_low = api_fit.add_module(type_id=eve_ship_item_id, rack=consts.ApiRack.low)
    api_ship_rig = api_fit.add_rig(type_id=eve_ship_item_id)
    api_ship_service = api_fit.add_service(type_id=eve_ship_item_id)
    api_struct_module_high = api_fit.add_module(type_id=eve_struct_item_id, rack=consts.ApiRack.high)
    api_struct_module_mid = api_fit.add_module(type_id=eve_struct_item_id, rack=consts.ApiRack.mid)
    api_struct_module_low = api_fit.add_module(type_id=eve_struct_item_id, rack=consts.ApiRack.low)
    api_struct_rig = api_fit.add_rig(type_id=eve_struct_item_id)
    api_struct_service = api_fit.add_service(type_id=eve_struct_item_id)
    api_other_module_high = api_fit.add_module(type_id=eve_other_item_id, rack=consts.ApiRack.high)
    api_other_module_mid = api_fit.add_module(type_id=eve_other_item_id, rack=consts.ApiRack.mid)
    api_other_module_low = api_fit.add_module(type_id=eve_other_item_id, rack=consts.ApiRack.low)
    api_other_rig = api_fit.add_rig(type_id=eve_other_item_id)
    api_other_service = api_fit.add_service(type_id=eve_other_item_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(item_vs_ship_kind=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_ship_module_high.remove()
    api_ship_module_mid.remove()
    api_ship_module_low.remove()
    api_ship_rig.remove()
    api_ship_service.remove()
    api_struct_module_high.remove()
    api_struct_module_mid.remove()
    api_struct_module_low.remove()
    api_struct_rig.remove()
    api_struct_service.remove()
    api_other_module_high.remove()
    api_other_module_mid.remove()
    api_other_module_low.remove()
    api_other_rig.remove()
    api_other_service.remove()
    # Verification
    api_val = api_fit.validate(options=ValOptions(item_vs_ship_kind=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_known_failures(client, consts):
    eve_ship_item_id = client.mk_eve_item(cat_id=consts.EveItemCat.module)
    eve_struct_id = client.mk_eve_struct()
    eve_other_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_other = api_fit.add_implant(type_id=eve_other_id)
    api_fit.set_ship(type_id=eve_struct_id)
    api_ship_module = api_fit.add_module(type_id=eve_ship_item_id)
    api_ship_rig = api_fit.add_rig(type_id=eve_ship_item_id)
    api_ship_service = api_fit.add_service(type_id=eve_ship_item_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(item_vs_ship_kind=(True, [api_ship_module.id])))
    assert api_val.passed is False
    assert api_val.details.item_vs_ship_kind.ship_kind == consts.ApiValShipType.structure
    assert api_val.details.item_vs_ship_kind.items == {
        api_ship_rig.id: consts.ApiValShipType.ship,
        api_ship_service.id: consts.ApiValShipType.ship}
    api_val = api_fit.validate(options=ValOptions(item_vs_ship_kind=(True, [api_ship_rig.id])))
    assert api_val.passed is False
    assert api_val.details.item_vs_ship_kind.ship_kind == consts.ApiValShipType.structure
    assert api_val.details.item_vs_ship_kind.items == {
        api_ship_module.id: consts.ApiValShipType.ship,
        api_ship_service.id: consts.ApiValShipType.ship}
    api_val = api_fit.validate(options=ValOptions(item_vs_ship_kind=(True, [api_ship_module.id, api_ship_service.id])))
    assert api_val.passed is False
    assert api_val.details.item_vs_ship_kind.ship_kind == consts.ApiValShipType.structure
    assert api_val.details.item_vs_ship_kind.items == {api_ship_rig.id: consts.ApiValShipType.ship}
    api_val = api_fit.validate(options=ValOptions(
        item_vs_ship_kind=(True, [api_ship_module.id, api_other.id, api_ship_service.id])))
    assert api_val.passed is False
    assert api_val.details.item_vs_ship_kind.ship_kind == consts.ApiValShipType.structure
    assert api_val.details.item_vs_ship_kind.items == {api_ship_rig.id: consts.ApiValShipType.ship}
    api_val = api_fit.validate(options=ValOptions(
        item_vs_ship_kind=(True, [api_ship_module.id, api_ship_rig.id, api_ship_service.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(options=ValOptions(
        item_vs_ship_kind=(True, [api_ship_module.id, api_ship_rig.id, api_other.id, api_ship_service.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_mutation(client, consts):
    eve_base_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module)
    eve_mutated_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.structure_module)
    eve_ship_id = client.mk_eve_ship()
    eve_struct_id = client.mk_eve_struct()
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_module_id], eve_mutated_module_id)])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct_id)
    api_module = api_fit.add_module(type_id=eve_base_module_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(item_vs_ship_kind=True))
    assert api_val.passed is False
    assert api_val.details.item_vs_ship_kind.ship_kind == consts.ApiValShipType.structure
    assert api_val.details.item_vs_ship_kind.items == {api_module.id: consts.ApiValShipType.ship}
    # Action
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(item_vs_ship_kind=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_module(mutation=eve_mutator_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(item_vs_ship_kind=True))
    assert api_val.passed is False
    assert api_val.details.item_vs_ship_kind.ship_kind == consts.ApiValShipType.ship
    assert api_val.details.item_vs_ship_kind.items == {api_module.id: consts.ApiValShipType.structure}
    # Action
    api_fit.set_ship(type_id=eve_struct_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(item_vs_ship_kind=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_module(mutation=None)
    # Verification
    api_val = api_fit.validate(options=ValOptions(item_vs_ship_kind=True))
    assert api_val.passed is False
    assert api_val.details.item_vs_ship_kind.ship_kind == consts.ApiValShipType.structure
    assert api_val.details.item_vs_ship_kind.items == {api_module.id: consts.ApiValShipType.ship}


def test_not_loaded_item(client):
    eve_item_id = client.alloc_item_id()
    eve_ship_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_item_id)
    api_rig = api_fit.add_rig(type_id=eve_item_id)
    api_service = api_fit.add_service(type_id=eve_item_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(item_vs_ship_kind=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.remove()
    api_rig.remove()
    api_service.remove()
    # Verification
    api_val = api_fit.validate(options=ValOptions(item_vs_ship_kind=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_criterion_item_category(client, consts):
    eve_item_id = client.mk_eve_item(cat_id=consts.EveItemCat.subsystem)
    eve_ship_id = client.mk_eve_ship()
    eve_struct_id = client.mk_eve_struct()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_item_id)
    api_fit.add_rig(type_id=eve_item_id)
    api_fit.add_service(type_id=eve_item_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(item_vs_ship_kind=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(item_vs_ship_kind=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_ship(type_id=eve_struct_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(item_vs_ship_kind=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_criterion_item_state(client, consts):
    eve_ship_item_id = client.mk_eve_item(cat_id=consts.EveItemCat.module)
    eve_struct_id = client.mk_eve_struct()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct_id)
    api_ship_module = api_fit.add_module(type_id=eve_ship_item_id, state=consts.ApiModuleState.ghost)
    api_ship_rig = api_fit.add_rig(type_id=eve_ship_item_id, state=False)
    api_ship_service = api_fit.add_service(type_id=eve_ship_item_id, state=consts.ApiServiceState.ghost)
    # Verification
    api_val = api_fit.validate(options=ValOptions(item_vs_ship_kind=True))
    assert api_val.passed is False
    assert api_val.details.item_vs_ship_kind.ship_kind == consts.ApiValShipType.structure
    assert api_val.details.item_vs_ship_kind.items == {
        api_ship_module.id: consts.ApiValShipType.ship,
        api_ship_rig.id: consts.ApiValShipType.ship,
        api_ship_service.id: consts.ApiValShipType.ship}


def test_criterion_item_kind(client, consts):
    eve_item_id = client.mk_eve_item(cat_id=consts.EveItemCat.module)
    eve_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.skill)
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_autocharge_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.fighter_ability_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_fighter1_id = client.mk_eve_item(
        cat_id=consts.EveItemCat.module,
        attrs={eve_autocharge_attr_id: eve_item_id},
        eff_ids=[eve_autocharge_effect_id])
    eve_fighter2_id = client.mk_eve_item(
        grp_id=consts.EveItemGrp.light_fighter,
        cat_id=consts.EveItemCat.fighter,
        attrs={eve_autocharge_attr_id: eve_item_id},
        eff_ids=[eve_autocharge_effect_id])
    eve_struct_id = client.mk_eve_struct()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_item_id)
    api_fit.add_drone(type_id=eve_item_id, state=consts.ApiMinionState.engaging)
    api_fighter1 = api_fit.add_fighter(type_id=eve_fighter1_id, state=consts.ApiMinionState.engaging)
    api_fighter2 = api_fit.add_fighter(type_id=eve_fighter2_id, state=consts.ApiMinionState.engaging)
    api_fit.add_fw_effect(type_id=eve_item_id)
    api_fit.add_implant(type_id=eve_item_id)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.overload, charge_type_id=eve_item_id)
    api_fit.set_ship(type_id=eve_struct_id)
    api_fit.add_skill(type_id=eve_item_id, level=5)
    api_fit.set_stance(type_id=eve_item_id)
    api_fit.add_subsystem(type_id=eve_item_id)
    # Verification
    assert len(api_fighter1.autocharges) == 1
    assert len(api_fighter2.autocharges) == 1
    api_val = api_fit.validate(options=ValOptions(item_vs_ship_kind=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
