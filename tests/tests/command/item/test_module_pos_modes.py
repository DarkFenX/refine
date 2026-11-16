from fw import Absent, check_no_field
from fw.api import ModAddMode, ModMoveMode


def test_add_append(client, consts):
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.append())
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [api_module1.id]
    assert api_module1.update().pos == 0
    # Action
    api_module2 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.append())
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [api_module1.id, api_module2.id]
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 1
    # Action
    api_module3 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.insert(pos=3))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [api_module1.id, api_module2.id, None, api_module3.id]
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 1
    assert api_module3.update().pos == 3
    # Action
    api_module4 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.append())
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [
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
    api_module1 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.equip())
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [api_module1.id]
    assert api_module1.update().pos == 0
    # Action
    api_module2 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.equip())
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [api_module1.id, api_module2.id]
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 1
    # Action
    api_module3 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.insert(pos=4))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [
        api_module1.id, api_module2.id, None, None, api_module3.id]
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 1
    assert api_module3.update().pos == 4
    # Action
    api_module4 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.equip())
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [
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
    api_module1 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.insert(pos=0))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [api_module1.id]
    assert api_module1.update().pos == 0
    # Action
    api_module2 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.insert(pos=2))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [api_module1.id, None, api_module2.id]
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 2
    # Action
    api_module3 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.insert(pos=5))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [
        api_module1.id, None, api_module2.id, None, None, api_module3.id]
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 2
    assert api_module3.update().pos == 5
    # Action
    api_module4 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.insert(pos=2))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [
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
    api_module1 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.replace(pos=0))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [api_module1.id]
    assert api_module1.update().pos == 0
    # Action
    api_module2 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.replace(pos=2))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [api_module1.id, None, api_module2.id]
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 2
    # Action
    api_module3 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.replace(pos=5))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [
        api_module1.id, None, api_module2.id, None, None, api_module3.id]
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 2
    assert api_module3.update().pos == 5
    # Action
    api_module4 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.replace(pos=3))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [
        api_module1.id, None, api_module2.id, api_module4.id, None, api_module3.id]
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 2
    assert api_module3.update().pos == 5
    assert api_module4.update().pos == 3
    # Action
    api_module5 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.replace(pos=2))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [
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
    # Action & verification
    api_fit.add_module(
        type_id=eve_module_id,
        rack=consts.Rack.high,
        add_mode=Absent,
        status_code=400,
        json_predicate={
            'code': 'JSN-001',
            'message': 'Failed to deserialize the JSON body into the target type: missing field `add_mode`'})


def test_add_pos_absent(client, consts):
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Action & verification
    api_fit.add_module(
        type_id=eve_module_id,
        rack=consts.Rack.high,
        add_mode=ModAddMode(mode='insert'),
        status_code=400,
        json_predicate={
            'code': 'JSN-001',
            'message': 'Failed to deserialize the JSON body into the target type: missing field `pos`'})


def test_add_append_limit(client, consts):
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.replace(pos=998))
    # Action
    api_module2 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.append())
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [None] * 998 + [api_module1.id, api_module2.id]
    assert api_module1.update().pos == 998
    assert api_module2.update().pos == 999
    # Action & verification
    api_fit.add_module(
        type_id=eve_module_id,
        rack=consts.Rack.high,
        add_mode=ModAddMode.append(),
        status_code=400,
        json_predicate={
            'code': 'MOD-006',
            'message': f"failed to add module: high rack of fit {api_fit.id} can't have modules past position 999"})
    assert api_fit.update().modules.high.map(lambda m: m.id) == [None] * 998 + [api_module1.id, api_module2.id]
    assert api_module1.update().pos == 998
    assert api_module2.update().pos == 999


def test_add_equip_limit(client, consts):
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_fit.batch() as api_fit_batch:
        api_modules = [
            api_fit_batch.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.equip())
            for _ in range(1000)]
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [m.id for m in api_modules]
    # Action & verification
    api_fit.add_module(
        type_id=eve_module_id,
        rack=consts.Rack.high,
        add_mode=ModAddMode.equip(),
        status_code=400,
        json_predicate={
            'code': 'MOD-006',
            'message': f"failed to add module: high rack of fit {api_fit.id} can't have modules past position 999"})
    assert api_fit.update().modules.high.map(lambda m: m.id) == [m.id for m in api_modules]
    # Action
    api_modules[500].remove(rm_mode=consts.ApiModRmMode.free)
    api_module = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.equip())
    # Verification
    assert api_module.update().pos == 500


def test_add_insert_limit(client, consts):
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.replace(pos=0))
    api_module2 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.replace(pos=998))
    # Action
    api_module3 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.insert(pos=0))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == (
        [api_module3.id, api_module1.id] + [None] * 997 + [api_module2.id])
    assert api_module1.update().pos == 1
    assert api_module2.update().pos == 999
    assert api_module3.update().pos == 0
    # Action & verification
    api_fit.add_module(
        type_id=eve_module_id,
        rack=consts.Rack.high,
        add_mode=ModAddMode.insert(pos=2),
        status_code=400,
        json_predicate={
            'code': 'MOD-006',
            'message': f"failed to add module: high rack of fit {api_fit.id} can't have modules past position 999"})
    assert api_fit.update().modules.high.map(lambda m: m.id) == (
        [api_module3.id, api_module1.id] + [None] * 997 + [api_module2.id])
    assert api_module1.update().pos == 1
    assert api_module2.update().pos == 999
    assert api_module3.update().pos == 0


def test_add_limit_batch(client, consts):
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.replace(pos=999))
    # Action & verification
    with api_fit.batch(status_code=400, json_predicate={
            'code': 'MOD-006',
            'message': f"failed to add module: high rack of fit {api_fit.id} can't have modules past position 999",
            'cmd_index': 1,
    }) as api_fit_batch:
        api_fit_batch.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.replace(pos=0))
        api_fit_batch.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.append())
    assert api_fit.update().modules.high.map(lambda m: m.id) == [None] * 999 + [api_module.id]
    # Action & verification
    with api_sol.batch(status_code=400, json_predicate={
            'code': 'MOD-006',
            'message': f"failed to add module: high rack of fit {api_fit.id} can't have modules past position 999",
            'cmd_index': 1,
    }) as api_sol_batch:
        api_sol_batch.add_module(
            fit_id=api_fit.id,
            type_id=eve_module_id,
            rack=consts.Rack.high,
            add_mode=ModAddMode.replace(pos=0))
        api_sol_batch.add_module(
            fit_id=api_fit.id,
            type_id=eve_module_id,
            rack=consts.Rack.high,
            add_mode=ModAddMode.insert(pos=0))
    assert api_fit.update().modules.high.map(lambda m: m.id) == [None] * 999 + [api_module.id]


def test_add_out_of_range(client, consts):
    # Positions past the last one are clamped to it, negative positions to 0
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Action
    api_module1 = api_fit.add_module(
        type_id=eve_module_id,
        rack=consts.Rack.high,
        add_mode=ModAddMode.insert(pos=2 ** 64 - 1))
    # Verification
    assert api_module1.update().pos == 999
    # Action
    api_module2 = api_fit.add_module(
        type_id=eve_module_id,
        rack=consts.Rack.high,
        add_mode=ModAddMode.replace(pos=1000))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [None] * 999 + [api_module2.id]
    assert api_module2.update().pos == 999
    # Action
    api_module3 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.replace(pos=-5))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [api_module3.id] + [None] * 998 + [api_module2.id]
    assert api_module2.update().pos == 999
    assert api_module3.update().pos == 0


def test_add_float(client, consts):
    # Floats are rounded, then clamped
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Action
    api_module1 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.insert(pos=2.4))
    # Verification
    assert api_module1.update().pos == 2
    # Action
    api_module2 = api_fit.add_module(
        type_id=eve_module_id,
        rack=consts.Rack.high,
        add_mode=ModAddMode.replace(pos=2 ** 64))
    # Verification
    assert api_module2.update().pos == 999
    # Action
    with api_fit.batch() as api_fit_batch:
        api_module3 = api_fit_batch.add_module(
            type_id=eve_module_id,
            rack=consts.Rack.high,
            add_mode=ModAddMode.replace(pos=4.6))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == (
        [None, None, api_module1.id, None, None, api_module3.id] + [None] * 993 + [api_module2.id])
    assert api_module1.update().pos == 2
    assert api_module2.update().pos == 999
    assert api_module3.update().pos == 5


def test_move_swap(client, consts):
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.replace(pos=1))
    api_module2 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.replace(pos=3))
    api_module3 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.replace(pos=5))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [
        None, api_module1.id, None, api_module2.id, None, api_module3.id]
    assert api_module1.update().pos == 1
    assert api_module2.update().pos == 3
    assert api_module3.update().pos == 5
    # Action
    api_module1.change_module(move=ModMoveMode.swap(pos=5))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [
        None, api_module3.id, None, api_module2.id, None, api_module1.id]
    assert api_module1.update().pos == 5
    assert api_module2.update().pos == 3
    assert api_module3.update().pos == 1
    # Action
    api_module1.change_module(move=ModMoveMode.swap(pos=0))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [
        api_module1.id, api_module3.id, None, api_module2.id]
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 3
    assert api_module3.update().pos == 1
    # Action
    api_module1.change_module(move=ModMoveMode.swap(pos=2))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [
        None, api_module3.id, api_module1.id, api_module2.id]
    assert api_module1.update().pos == 2
    assert api_module2.update().pos == 3
    assert api_module3.update().pos == 1
    # Action
    api_module1.change_module(move=ModMoveMode.swap(pos=6))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [
        None, api_module3.id, None, api_module2.id, None, None, api_module1.id]
    assert api_module1.update().pos == 6
    assert api_module2.update().pos == 3
    assert api_module3.update().pos == 1
    # Action
    api_module1.change_module(move=ModMoveMode.swap(pos=6))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [
        None, api_module3.id, None, api_module2.id, None, None, api_module1.id]
    assert api_module1.update().pos == 6
    assert api_module2.update().pos == 3
    assert api_module3.update().pos == 1


def test_move_shift(client, consts):
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.replace(pos=0))
    api_module2 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.replace(pos=1))
    api_module3 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.replace(pos=2))
    api_module4 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.replace(pos=3))
    api_module5 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.replace(pos=4))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [
        api_module1.id, api_module2.id, api_module3.id, api_module4.id, api_module5.id]
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 1
    assert api_module3.update().pos == 2
    assert api_module4.update().pos == 3
    assert api_module5.update().pos == 4
    # Action
    api_module4.change_module(move=ModMoveMode.shift(pos=1))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [
        api_module1.id, api_module4.id, api_module2.id, api_module3.id, api_module5.id]
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 2
    assert api_module3.update().pos == 3
    assert api_module4.update().pos == 1
    assert api_module5.update().pos == 4
    # Action
    api_module4.change_module(move=ModMoveMode.shift(pos=3))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [
        api_module1.id, api_module2.id, api_module3.id, api_module4.id, api_module5.id]
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 1
    assert api_module3.update().pos == 2
    assert api_module4.update().pos == 3
    assert api_module5.update().pos == 4
    # Action
    api_module4.change_module(move=ModMoveMode.shift(pos=0))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [
        api_module4.id, api_module1.id, api_module2.id, api_module3.id, api_module5.id]
    assert api_module1.update().pos == 1
    assert api_module2.update().pos == 2
    assert api_module3.update().pos == 3
    assert api_module4.update().pos == 0
    assert api_module5.update().pos == 4
    # Action
    api_module4.change_module(move=ModMoveMode.shift(pos=6))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [
        api_module1.id, api_module2.id, api_module3.id, api_module5.id, None, None, api_module4.id]
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 1
    assert api_module3.update().pos == 2
    assert api_module4.update().pos == 6
    assert api_module5.update().pos == 3
    # Action
    api_module4.change_module(move=ModMoveMode.shift(pos=5))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [
        api_module1.id, api_module2.id, api_module3.id, api_module5.id, None, api_module4.id]
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 1
    assert api_module3.update().pos == 2
    assert api_module4.update().pos == 5
    assert api_module5.update().pos == 3
    # Action
    api_module4.change_module(move=ModMoveMode.shift(pos=3))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [
        api_module1.id, api_module2.id, api_module3.id, api_module4.id, api_module5.id]
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 1
    assert api_module3.update().pos == 2
    assert api_module4.update().pos == 3
    assert api_module5.update().pos == 4
    # Action
    api_module4.change_module(move=ModMoveMode.shift(pos=3))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [
        api_module1.id, api_module2.id, api_module3.id, api_module4.id, api_module5.id]
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 1
    assert api_module3.update().pos == 2
    assert api_module4.update().pos == 3
    assert api_module5.update().pos == 4


def test_move_out_of_range(client, consts):
    # Positions past the last one are clamped to it
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.replace(pos=0))
    api_module2 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.replace(pos=1))
    # Action
    api_module1.change_module(move=ModMoveMode.shift(pos=5000))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [api_module2.id] + [None] * 998 + [api_module1.id]
    assert api_module1.update().pos == 999
    assert api_module2.update().pos == 0
    # Action
    api_module2.change_module(move=ModMoveMode.swap(pos=5000))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [api_module1.id] + [None] * 998 + [api_module2.id]
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 999


def test_move_float(client, consts):
    # Floats are rounded, then clamped
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.replace(pos=0))
    api_module2 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.replace(pos=1))
    # Action
    api_module1.change_module(move=ModMoveMode.shift(pos=3.4))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [api_module2.id, None, None, api_module1.id]
    assert api_module1.update().pos == 3
    assert api_module2.update().pos == 0
    # Action
    api_module2.change_module(move=ModMoveMode.swap(pos=2.6))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [api_module1.id, None, None, api_module2.id]
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 3
    # Action
    with api_fit.batch() as api_fit_batch:
        api_fit_batch.change_module(item_id=api_module2.id, move=ModMoveMode.swap(pos=1e30))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [api_module1.id] + [None] * 998 + [api_module2.id]
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 999


def test_move_pos_absent(client, consts):
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high)
    # Action & verification
    api_module.change_module(
        move=ModMoveMode(mode='shift'),
        status_code=400,
        json_predicate={
            'code': 'JSN-001',
            'message': 'Failed to deserialize the JSON body into the target type: missing field `pos`'})
    assert api_module.update().pos == 0


def test_remove_remove(client, consts):
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.replace(pos=1))
    api_module2 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.replace(pos=3))
    api_module3 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.replace(pos=5))
    api_module4 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.replace(pos=6))
    api_module5 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.replace(pos=7))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [
        None, api_module1.id, None, api_module2.id, None, api_module3.id, api_module4.id, api_module5.id]
    assert api_module1.update().pos == 1
    assert api_module2.update().pos == 3
    assert api_module3.update().pos == 5
    assert api_module4.update().pos == 6
    assert api_module5.update().pos == 7
    # Action
    api_module4.remove(rm_mode=consts.ApiModRmMode.remove)
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [
        None, api_module1.id, None, api_module2.id, None, api_module3.id, api_module5.id]
    assert api_module1.update().pos == 1
    assert api_module2.update().pos == 3
    assert api_module3.update().pos == 5
    assert api_module5.update().pos == 6
    # Action
    api_module5.remove(rm_mode=consts.ApiModRmMode.remove)
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [
        None, api_module1.id, None, api_module2.id, None, api_module3.id]
    assert api_module1.update().pos == 1
    assert api_module2.update().pos == 3
    assert api_module3.update().pos == 5
    # Action
    api_module2.remove(rm_mode=consts.ApiModRmMode.remove)
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [None, api_module1.id, None, None, api_module3.id]
    assert api_module1.update().pos == 1
    assert api_module3.update().pos == 4
    # Action
    api_module3.remove(rm_mode=consts.ApiModRmMode.remove)
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [None, api_module1.id]
    assert api_module1.update().pos == 1
    # Action
    api_module1.remove(rm_mode=consts.ApiModRmMode.remove)
    # Verification
    api_fit.update()
    with check_no_field():
        api_fit.modules  # ruff:ignore[useless-expression]


def test_remove_free(client, consts):
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.replace(pos=1))
    api_module2 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.replace(pos=3))
    api_module3 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.replace(pos=5))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [
        None, api_module1.id, None, api_module2.id, None, api_module3.id]
    assert api_module1.update().pos == 1
    assert api_module2.update().pos == 3
    assert api_module3.update().pos == 5
    # Action
    api_module2.remove(rm_mode=consts.ApiModRmMode.free)
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [None, api_module1.id, None, None, None, api_module3.id]
    assert api_module1.update().pos == 1
    assert api_module3.update().pos == 5
    # Action
    api_module3.remove(rm_mode=consts.ApiModRmMode.free)
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [None, api_module1.id]
    assert api_module1.update().pos == 1
    # Action
    api_module1.remove(rm_mode=consts.ApiModRmMode.free)
    # Verification
    api_fit.update()
    with check_no_field():
        api_fit.modules  # ruff:ignore[useless-expression]


def test_remove_absent(client, consts):
    # Equivalent of free
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.replace(pos=1))
    api_module2 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.replace(pos=3))
    api_module3 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high, add_mode=ModAddMode.replace(pos=5))
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [
        None, api_module1.id, None, api_module2.id, None, api_module3.id]
    assert api_module1.update().pos == 1
    assert api_module2.update().pos == 3
    assert api_module3.update().pos == 5
    # Action
    api_module2.remove()
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [None, api_module1.id, None, None, None, api_module3.id]
    assert api_module1.update().pos == 1
    assert api_module3.update().pos == 5
    # Action
    api_module3.remove()
    # Verification
    assert api_fit.update().modules.high.map(lambda m: m.id) == [None, api_module1.id]
    assert api_module1.update().pos == 1
    # Action
    api_module1.remove()
    # Verification
    api_fit.update()
    with check_no_field():
        api_fit.modules  # ruff:ignore[useless-expression]
