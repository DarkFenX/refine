from tests import check_no_field
from tests.fw.api import ValOptions


def test_ship_kind_switching(client, consts):
    eve_ship_item_id = client.mk_eve_item(cat_id=consts.EveItemCat.module)
    eve_struct_item_id = client.mk_eve_item(cat_id=consts.EveItemCat.structure_module)
    eve_other_item_id = client.mk_eve_item(cat_id=consts.EveItemCat.subsystem)
    eve_ship_id = client.mk_eve_ship()
    eve_struct_id = client.mk_eve_struct()
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


def test_item_add_remove_ship(client, consts):
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


def test_item_add_remove_struct(client, consts):
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


def test_item_add_remove_unknown(client, consts):
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
