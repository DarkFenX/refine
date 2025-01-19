
def test_append(client, consts):
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode=consts.ApiModAddMode.append)
    # Verification
    api_fit.update()
    assert len(api_fit.modules.high) == 1
    assert api_fit.modules.high[0].id == api_module1.id
    assert api_module1.update().pos == 0
    # Action
    api_module2 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode=consts.ApiModAddMode.append)
    # Verification
    api_fit.update()
    assert len(api_fit.modules.high) == 2
    assert api_fit.modules.high[0].id == api_module1.id
    assert api_fit.modules.high[1].id == api_module2.id
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 1
    # Action
    api_module3 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode={consts.ApiModAddMode.insert: 3})
    # Verification
    api_fit.update()
    assert len(api_fit.modules.high) == 4
    assert api_fit.modules.high[0].id == api_module1.id
    assert api_fit.modules.high[1].id == api_module2.id
    assert api_fit.modules.high[2] is None
    assert api_fit.modules.high[3].id == api_module3.id
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 1
    assert api_module3.update().pos == 3
    # Action
    api_module4 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode=consts.ApiModAddMode.append)
    # Verification
    api_fit.update()
    assert len(api_fit.modules.high) == 5
    assert api_fit.modules.high[0].id == api_module1.id
    assert api_fit.modules.high[1].id == api_module2.id
    assert api_fit.modules.high[2] is None
    assert api_fit.modules.high[3].id == api_module3.id
    assert api_fit.modules.high[4].id == api_module4.id
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 1
    assert api_module3.update().pos == 3
    assert api_module4.update().pos == 4


def test_equip(client, consts):
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode=consts.ApiModAddMode.equip)
    # Verification
    api_fit.update()
    assert len(api_fit.modules.high) == 1
    assert api_fit.modules.high[0].id == api_module1.id
    assert api_module1.update().pos == 0
    # Action
    api_module2 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode=consts.ApiModAddMode.equip)
    # Verification
    api_fit.update()
    assert len(api_fit.modules.high) == 2
    assert api_fit.modules.high[0].id == api_module1.id
    assert api_fit.modules.high[1].id == api_module2.id
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 1
    # Action
    api_module3 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode={consts.ApiModAddMode.insert: 4})
    # Verification
    api_fit.update()
    assert len(api_fit.modules.high) == 5
    assert api_fit.modules.high[0].id == api_module1.id
    assert api_fit.modules.high[1].id == api_module2.id
    assert api_fit.modules.high[2] is None
    assert api_fit.modules.high[3] is None
    assert api_fit.modules.high[4].id == api_module3.id
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 1
    assert api_module3.update().pos == 4
    # Action
    api_module4 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode=consts.ApiModAddMode.equip)
    # Verification
    api_fit.update()
    assert len(api_fit.modules.high) == 5
    assert api_fit.modules.high[0].id == api_module1.id
    assert api_fit.modules.high[1].id == api_module2.id
    assert api_fit.modules.high[2].id == api_module4.id
    assert api_fit.modules.high[3] is None
    assert api_fit.modules.high[4].id == api_module3.id
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 1
    assert api_module3.update().pos == 4
    assert api_module4.update().pos == 2


def test_insert(client, consts):
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode={consts.ApiModAddMode.insert: 0})
    # Verification
    api_fit.update()
    assert len(api_fit.modules.high) == 1
    assert api_fit.modules.high[0].id == api_module1.id
    assert api_module1.update().pos == 0
    # Action
    api_module2 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode={consts.ApiModAddMode.insert: 2})
    # Verification
    api_fit.update()
    assert len(api_fit.modules.high) == 3
    assert api_fit.modules.high[0].id == api_module1.id
    assert api_fit.modules.high[1] is None
    assert api_fit.modules.high[2].id == api_module2.id
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 2
    # Action
    api_module3 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode={consts.ApiModAddMode.insert: 5})
    # Verification
    api_fit.update()
    assert len(api_fit.modules.high) == 6
    assert api_fit.modules.high[0].id == api_module1.id
    assert api_fit.modules.high[1] is None
    assert api_fit.modules.high[2].id == api_module2.id
    assert api_fit.modules.high[3] is None
    assert api_fit.modules.high[4] is None
    assert api_fit.modules.high[5].id == api_module3.id
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 2
    assert api_module3.update().pos == 5
    # Action
    api_module4 = api_fit.add_mod(
        type_id=eve_module_id,
        rack=consts.ApiRack.high,
        mode={consts.ApiModAddMode.insert: 2})
    # Verification
    api_fit.update()
    assert len(api_fit.modules.high) == 7
    assert api_fit.modules.high[0].id == api_module1.id
    assert api_fit.modules.high[1] is None
    assert api_fit.modules.high[2].id == api_module4.id
    assert api_fit.modules.high[3].id == api_module2.id
    assert api_fit.modules.high[4] is None
    assert api_fit.modules.high[5] is None
    assert api_fit.modules.high[6].id == api_module3.id
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 3
    assert api_module3.update().pos == 6
    assert api_module4.update().pos == 2
