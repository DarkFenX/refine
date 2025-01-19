from tests import check_no_field
from tests.support.util import Absent


def flatten(*, rack) -> list:
    return [None if i is None else i.id for i in rack]


def test_add_append(client, consts):
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode=consts.ApiModAddMode.append)
    # Verification
    assert flatten(rack=api_fit.update().modules.high) == [api_module1.id]
    assert api_module1.update().pos == 0
    # Action
    api_module2 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode=consts.ApiModAddMode.append)
    # Verification
    assert flatten(rack=api_fit.update().modules.high) == [api_module1.id, api_module2.id]
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 1
    # Action
    api_module3 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode={consts.ApiModAddMode.insert: 3})
    # Verification
    assert flatten(rack=api_fit.update().modules.high) == [api_module1.id, api_module2.id, None, api_module3.id]
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 1
    assert api_module3.update().pos == 3
    # Action
    api_module4 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode=consts.ApiModAddMode.append)
    # Verification
    assert flatten(rack=api_fit.update().modules.high) == [
        api_module1.id, api_module2.id, None, api_module3.id, api_module4.id]
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 1
    assert api_module3.update().pos == 3
    assert api_module4.update().pos == 4


def test_add_equip(client, consts):
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode=consts.ApiModAddMode.equip)
    # Verification
    assert flatten(rack=api_fit.update().modules.high) == [api_module1.id]
    assert api_module1.update().pos == 0
    # Action
    api_module2 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode=consts.ApiModAddMode.equip)
    # Verification
    assert flatten(rack=api_fit.update().modules.high) == [api_module1.id, api_module2.id]
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 1
    # Action
    api_module3 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode={consts.ApiModAddMode.insert: 4})
    # Verification
    assert flatten(rack=api_fit.update().modules.high) == [api_module1.id, api_module2.id, None, None, api_module3.id]
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 1
    assert api_module3.update().pos == 4
    # Action
    api_module4 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode=consts.ApiModAddMode.equip)
    # Verification
    assert flatten(rack=api_fit.update().modules.high) == [
        api_module1.id, api_module2.id, api_module4.id, None, api_module3.id]
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 1
    assert api_module3.update().pos == 4
    assert api_module4.update().pos == 2


def test_add_insert(client, consts):
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode={consts.ApiModAddMode.insert: 0})
    # Verification
    assert flatten(rack=api_fit.update().modules.high) == [api_module1.id]
    assert api_module1.update().pos == 0
    # Action
    api_module2 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode={consts.ApiModAddMode.insert: 2})
    # Verification
    assert flatten(rack=api_fit.update().modules.high) == [api_module1.id, None, api_module2.id]
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 2
    # Action
    api_module3 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode={consts.ApiModAddMode.insert: 5})
    # Verification
    assert flatten(rack=api_fit.update().modules.high) == [
        api_module1.id, None, api_module2.id, None, None, api_module3.id]
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 2
    assert api_module3.update().pos == 5
    # Action
    api_module4 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode={consts.ApiModAddMode.insert: 2})
    # Verification
    assert flatten(rack=api_fit.update().modules.high) == [
        api_module1.id, None, api_module4.id, api_module2.id, None, None, api_module3.id]
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 3
    assert api_module3.update().pos == 6
    assert api_module4.update().pos == 2


def test_add_replace(client, consts):
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode={consts.ApiModAddMode.replace: 0})
    # Verification
    assert flatten(rack=api_fit.update().modules.high) == [api_module1.id]
    assert api_module1.update().pos == 0
    # Action
    api_module2 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode={consts.ApiModAddMode.replace: 2})
    # Verification
    assert flatten(rack=api_fit.update().modules.high) == [api_module1.id, None, api_module2.id]
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 2
    # Action
    api_module3 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode={consts.ApiModAddMode.replace: 5})
    # Verification
    assert flatten(rack=api_fit.update().modules.high) == [
        api_module1.id, None, api_module2.id, None, None, api_module3.id]
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 2
    assert api_module3.update().pos == 5
    # Action
    api_module4 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode={consts.ApiModAddMode.replace: 3})
    # Verification
    assert flatten(rack=api_fit.update().modules.high) == [
        api_module1.id, None, api_module2.id, api_module4.id, None, api_module3.id]
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 2
    assert api_module3.update().pos == 5
    assert api_module4.update().pos == 3
    # Action
    api_module5 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode={consts.ApiModAddMode.replace: 2})
    # Verification
    assert flatten(rack=api_fit.update().modules.high) == [
        api_module1.id, None, api_module5.id, api_module4.id, None, api_module3.id]
    assert api_module1.update().pos == 0
    assert api_module3.update().pos == 5
    assert api_module4.update().pos == 3
    assert api_module5.update().pos == 2


def test_add_absent(client, consts):
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode=Absent,
        status_code=422,
        text_predicate='Failed to deserialize the JSON body into the target type: missing field `add_mode`')


def test_remove_remove(client, consts):
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode={consts.ApiModAddMode.replace: 1})
    api_module2 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode={consts.ApiModAddMode.replace: 3})
    api_module3 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode={consts.ApiModAddMode.replace: 5})
    # Verification
    assert flatten(rack=api_fit.update().modules.high) == [
        None, api_module1.id, None, api_module2.id, None, api_module3.id]
    assert api_module1.update().pos == 1
    assert api_module2.update().pos == 3
    assert api_module3.update().pos == 5
    # Action
    api_module2.remove(mode=consts.ApiModRmMode.remove)
    # Verification
    assert flatten(rack=api_fit.update().modules.high) == [None, api_module1.id, None, None, api_module3.id]
    assert api_module1.update().pos == 1
    assert api_module3.update().pos == 4
    # Action
    api_module3.remove(mode=consts.ApiModRmMode.remove)
    # Verification
    assert flatten(rack=api_fit.update().modules.high) == [None, api_module1.id]
    assert api_module1.update().pos == 1
    # Action
    api_module1.remove(mode=consts.ApiModRmMode.remove)
    # Verification
    with check_no_field():
        api_fit.update().modules  # pylint: disable=W0104


def test_remove_free(client, consts):
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode={consts.ApiModAddMode.replace: 1})
    api_module2 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode={consts.ApiModAddMode.replace: 3})
    api_module3 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode={consts.ApiModAddMode.replace: 5})
    # Verification
    assert flatten(rack=api_fit.update().modules.high) == [
        None, api_module1.id, None, api_module2.id, None, api_module3.id]
    assert api_module1.update().pos == 1
    assert api_module2.update().pos == 3
    assert api_module3.update().pos == 5
    # Action
    api_module2.remove(mode=consts.ApiModRmMode.free)
    # Verification
    assert flatten(rack=api_fit.update().modules.high) == [None, api_module1.id, None, None, None, api_module3.id]
    assert api_module1.update().pos == 1
    assert api_module3.update().pos == 5
    # Action
    api_module3.remove(mode=consts.ApiModRmMode.free)
    # Verification
    assert flatten(rack=api_fit.update().modules.high) == [None, api_module1.id]
    assert api_module1.update().pos == 1
    # Action
    api_module1.remove(mode=consts.ApiModRmMode.free)
    # Verification
    with check_no_field():
        api_fit.update().modules  # pylint: disable=W0104


def test_remove_absent(client, consts):
    # Equivalent of free
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode={consts.ApiModAddMode.replace: 1})
    api_module2 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode={consts.ApiModAddMode.replace: 3})
    api_module3 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode={consts.ApiModAddMode.replace: 5})
    # Verification
    assert flatten(rack=api_fit.update().modules.high) == [
        None, api_module1.id, None, api_module2.id, None, api_module3.id]
    assert api_module1.update().pos == 1
    assert api_module2.update().pos == 3
    assert api_module3.update().pos == 5
    # Action
    api_module2.remove()
    # Verification
    assert flatten(rack=api_fit.update().modules.high) == [None, api_module1.id, None, None, None, api_module3.id]
    assert api_module1.update().pos == 1
    assert api_module3.update().pos == 5
    # Action
    api_module3.remove()
    # Verification
    assert flatten(rack=api_fit.update().modules.high) == [None, api_module1.id]
    assert api_module1.update().pos == 1
    # Action
    api_module1.remove()
    # Verification
    with check_no_field():
        api_fit.update().modules  # pylint: disable=W0104
