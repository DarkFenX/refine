from fw import Muta, approx, check_no_field


def test_from_stage2(client, consts):
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_base_item1_id = client.mk_eve_item(attrs={eve_attr1_id: 100, eve_attr2_id: 100})
    eve_base_item2_id = client.mk_eve_item(attrs={eve_attr1_id: 200, eve_attr2_id: 300})
    eve_mutated_item2_id = client.mk_eve_item(attrs={eve_attr2_id: 200})
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_item2_id], eve_mutated_item2_id)],
        attrs={eve_attr1_id: (0.8, 1.2), eve_attr2_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(type_id=eve_base_item1_id, mutation=(eve_mutator_id, {
        eve_attr1_id: Muta.roll_to_api(val=0.3),
        eve_attr2_id: Muta.roll_to_api(val=0.3)}))
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item1_id
    assert api_item.mutation.type == consts.ApiMutationType.dormant
    assert len(api_item.mutation.rolls) == 2
    assert api_item.mutation.rolls[eve_attr1_id] == approx(0.3)
    assert api_item.mutation.rolls[eve_attr2_id] == approx(0.3)
    assert api_item.attrs[eve_attr1_id].base == approx(100)
    assert api_item.attrs[eve_attr2_id].base == approx(100)
    # Action
    api_item.change_module(type_id=eve_base_item2_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item2_id
    assert api_item.mutation.type == consts.ApiMutationType.effective
    assert api_item.mutation.base_type_id == eve_base_item2_id
    assert len(api_item.mutation.attrs) == 2
    assert api_item.mutation.attrs[eve_attr1_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr1_id].absolute == approx(184)
    assert api_item.mutation.attrs[eve_attr2_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr2_id].absolute == approx(276)
    assert len(api_item.mutation.rolls) == 2
    assert api_item.mutation.rolls[eve_attr1_id] == approx(0.3)
    assert api_item.mutation.rolls[eve_attr2_id] == approx(0.3)
    assert api_item.attrs[eve_attr1_id].base == approx(184)
    assert api_item.attrs[eve_attr2_id].base == approx(276)


def test_from_stage3(client, consts):
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_base_item1_id = client.mk_eve_item(attrs={eve_attr1_id: 100, eve_attr2_id: 100})
    eve_base_item2_id = client.mk_eve_item(attrs={eve_attr1_id: 200, eve_attr2_id: 300})
    eve_mutated_item1_id = client.alloc_item_id()
    eve_mutated_item2_id = client.mk_eve_item(attrs={eve_attr2_id: 200})
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_item1_id], eve_mutated_item1_id), ([eve_base_item2_id], eve_mutated_item2_id)],
        attrs={eve_attr1_id: (0.8, 1.2), eve_attr2_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(type_id=eve_base_item1_id, mutation=(eve_mutator_id, {
        eve_attr1_id: Muta.roll_to_api(val=0.3),
        eve_attr2_id: Muta.roll_to_api(val=0.3)}))
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item1_id
    assert api_item.mutation.type == consts.ApiMutationType.dormant
    assert len(api_item.mutation.rolls) == 2
    assert api_item.mutation.rolls[eve_attr1_id] == approx(0.3)
    assert api_item.mutation.rolls[eve_attr2_id] == approx(0.3)
    assert api_item.attrs[eve_attr1_id].base == approx(100)
    assert api_item.attrs[eve_attr2_id].base == approx(100)
    # Action
    api_item.change_module(type_id=eve_base_item2_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item2_id
    assert api_item.mutation.type == consts.ApiMutationType.effective
    assert api_item.mutation.base_type_id == eve_base_item2_id
    assert len(api_item.mutation.attrs) == 2
    assert api_item.mutation.attrs[eve_attr1_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr1_id].absolute == approx(184)
    assert api_item.mutation.attrs[eve_attr2_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr2_id].absolute == approx(276)
    assert len(api_item.mutation.rolls) == 2
    assert api_item.mutation.rolls[eve_attr1_id] == approx(0.3)
    assert api_item.mutation.rolls[eve_attr2_id] == approx(0.3)
    assert api_item.attrs[eve_attr1_id].base == approx(184)
    assert api_item.attrs[eve_attr2_id].base == approx(276)


def test_from_stage4_mutated_item_same(client, consts):
    eve_attr_id = client.mk_eve_attr()
    eve_base_item1_id = client.mk_eve_item(attrs={eve_attr_id: 100})
    eve_base_item2_id = client.mk_eve_item(attrs={eve_attr_id: 200})
    eve_mutated_item_id = client.mk_eve_item()
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_item1_id, eve_base_item2_id], eve_mutated_item_id)],
        attrs={eve_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(
        type_id=eve_base_item1_id,
        mutation=(eve_mutator_id, {eve_attr_id: Muta.roll_to_api(val=0.3)}))
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.mutation.type == consts.ApiMutationType.effective
    assert api_item.mutation.base_type_id == eve_base_item1_id
    assert len(api_item.mutation.attrs) == 1
    assert api_item.mutation.attrs[eve_attr_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr_id].absolute == approx(92)
    assert len(api_item.mutation.rolls) == 1
    assert api_item.mutation.rolls[eve_attr_id] == approx(0.3)
    assert api_item.attrs[eve_attr_id].base == approx(92)
    # Action
    api_item.change_module(type_id=eve_base_item2_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.mutation.type == consts.ApiMutationType.effective
    assert api_item.mutation.base_type_id == eve_base_item2_id
    assert len(api_item.mutation.attrs) == 1
    assert api_item.mutation.attrs[eve_attr_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr_id].absolute == approx(184)
    assert len(api_item.mutation.rolls) == 1
    assert api_item.mutation.rolls[eve_attr_id] == approx(0.3)
    assert api_item.attrs[eve_attr_id].base == approx(184)


def test_from_stage4_mutated_item_same_base_not_loaded(client, consts):
    eve_attr_id = client.mk_eve_attr()
    eve_base_item1_id = client.mk_eve_item(attrs={eve_attr_id: 100})
    eve_base_item2_id = client.alloc_item_id()
    eve_mutated_item_id = client.mk_eve_item()
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_item1_id, eve_base_item2_id], eve_mutated_item_id)],
        attrs={eve_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(
        type_id=eve_base_item1_id,
        mutation=(eve_mutator_id, {eve_attr_id: Muta.roll_to_api(val=0.3)}))
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.mutation.type == consts.ApiMutationType.effective
    assert api_item.mutation.base_type_id == eve_base_item1_id
    assert len(api_item.mutation.attrs) == 1
    assert api_item.mutation.attrs[eve_attr_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr_id].absolute == approx(92)
    assert len(api_item.mutation.rolls) == 1
    assert api_item.mutation.rolls[eve_attr_id] == approx(0.3)
    assert api_item.attrs[eve_attr_id].base == approx(92)
    # Action
    api_item.change_module(type_id=eve_base_item2_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.mutation.type == consts.ApiMutationType.effective
    assert api_item.mutation.base_type_id == eve_base_item2_id
    with check_no_field():
        api_item.mutation.attrs  # ruff:ignore[useless-expression]
    assert len(api_item.mutation.rolls) == 1
    assert api_item.mutation.rolls[eve_attr_id] == approx(0.3)
    # Action
    api_item.change_module(type_id=eve_base_item1_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.mutation.type == consts.ApiMutationType.effective
    assert api_item.mutation.base_type_id == eve_base_item1_id
    assert len(api_item.mutation.attrs) == 1
    assert api_item.mutation.attrs[eve_attr_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr_id].absolute == approx(92)
    assert len(api_item.mutation.rolls) == 1
    assert api_item.mutation.rolls[eve_attr_id] == approx(0.3)
    assert api_item.attrs[eve_attr_id].base == approx(92)


def test_from_stage4_mutated_item_same_drone(client, consts):
    eve_mutated_attr_id = client.mk_eve_attr()
    eve_old_attr_id = client.mk_eve_attr()
    eve_new_attr_id = client.mk_eve_attr()
    eve_base_item1_id = client.mk_eve_drone(attrs={eve_mutated_attr_id: 100, eve_old_attr_id: 50})
    eve_base_item2_id = client.mk_eve_drone(attrs={eve_mutated_attr_id: 200, eve_new_attr_id: 70})
    eve_mutated_item_id = client.mk_eve_drone(attrs={eve_new_attr_id: 10})
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_item1_id, eve_base_item2_id], eve_mutated_item_id)],
        attrs={eve_mutated_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(
        type_id=eve_base_item1_id,
        mutation=(eve_mutator_id, {eve_mutated_attr_id: Muta.roll_to_api(val=0.3)}))
    # Verification
    api_drone.update()
    assert api_drone.type_id == eve_mutated_item_id
    assert api_drone.mutation.type == consts.ApiMutationType.effective
    assert api_drone.mutation.base_type_id == eve_base_item1_id
    assert len(api_drone.mutation.attrs) == 1
    assert api_drone.mutation.attrs[eve_mutated_attr_id].roll == approx(0.3)
    assert api_drone.mutation.attrs[eve_mutated_attr_id].absolute == approx(92)
    assert api_drone.attrs[eve_mutated_attr_id].base == approx(92)
    assert api_drone.attrs[eve_old_attr_id].base == approx(50)
    assert api_drone.attrs[eve_new_attr_id].base == approx(10)
    # Action
    api_drone.change_drone(type_id=eve_base_item2_id)
    # Verification
    api_drone.update()
    assert api_drone.type_id == eve_mutated_item_id
    assert api_drone.mutation.type == consts.ApiMutationType.effective
    assert api_drone.mutation.base_type_id == eve_base_item2_id
    assert len(api_drone.mutation.attrs) == 1
    assert api_drone.mutation.attrs[eve_mutated_attr_id].roll == approx(0.3)
    assert api_drone.mutation.attrs[eve_mutated_attr_id].absolute == approx(184)
    assert len(api_drone.mutation.rolls) == 1
    assert api_drone.mutation.rolls[eve_mutated_attr_id] == approx(0.3)
    assert api_drone.attrs[eve_mutated_attr_id].base == approx(184)
    assert eve_old_attr_id not in api_drone.attrs
    assert api_drone.attrs[eve_new_attr_id].base == approx(70)


def test_from_stage4_mutated_item_different(client, consts):
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_base_item1_id = client.mk_eve_item(attrs={eve_attr1_id: 100, eve_attr2_id: 100})
    eve_base_item2_id = client.mk_eve_item(attrs={eve_attr1_id: 200, eve_attr2_id: 300})
    eve_mutated_item1_id = client.mk_eve_item()
    eve_mutated_item2_id = client.mk_eve_item(attrs={eve_attr2_id: 200})
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_item1_id], eve_mutated_item1_id), ([eve_base_item2_id], eve_mutated_item2_id)],
        attrs={eve_attr1_id: (0.8, 1.2), eve_attr2_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(type_id=eve_base_item1_id, mutation=(eve_mutator_id, {
        eve_attr1_id: Muta.roll_to_api(val=0.3),
        eve_attr2_id: Muta.roll_to_api(val=0.3)}))
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item1_id
    assert api_item.mutation.type == consts.ApiMutationType.effective
    assert api_item.mutation.base_type_id == eve_base_item1_id
    assert len(api_item.mutation.attrs) == 2
    assert api_item.mutation.attrs[eve_attr1_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr1_id].absolute == approx(92)
    assert api_item.mutation.attrs[eve_attr2_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr2_id].absolute == approx(92)
    assert len(api_item.mutation.rolls) == 2
    assert api_item.mutation.rolls[eve_attr1_id] == approx(0.3)
    assert api_item.mutation.rolls[eve_attr2_id] == approx(0.3)
    assert api_item.attrs[eve_attr1_id].base == approx(92)
    assert api_item.attrs[eve_attr2_id].base == approx(92)
    # Action
    api_item.change_module(type_id=eve_base_item2_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item2_id
    assert api_item.mutation.type == consts.ApiMutationType.effective
    assert api_item.mutation.base_type_id == eve_base_item2_id
    assert len(api_item.mutation.attrs) == 2
    assert api_item.mutation.attrs[eve_attr1_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr1_id].absolute == approx(184)
    assert api_item.mutation.attrs[eve_attr2_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr2_id].absolute == approx(276)
    assert len(api_item.mutation.rolls) == 2
    assert api_item.mutation.rolls[eve_attr1_id] == approx(0.3)
    assert api_item.mutation.rolls[eve_attr2_id] == approx(0.3)
    assert api_item.attrs[eve_attr1_id].base == approx(184)
    assert api_item.attrs[eve_attr2_id].base == approx(276)


def test_from_stage4_mutated_item_different_base_not_loaded(client, consts):
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_base_item1_id = client.mk_eve_item(attrs={eve_attr1_id: 100, eve_attr2_id: 100})
    eve_base_item2_id = client.alloc_item_id()
    eve_mutated_item1_id = client.mk_eve_item()
    eve_mutated_item2_id = client.mk_eve_item(attrs={eve_attr2_id: 300})
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_item1_id], eve_mutated_item1_id), ([eve_base_item2_id], eve_mutated_item2_id)],
        attrs={eve_attr1_id: (0.8, 1.2), eve_attr2_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(type_id=eve_base_item1_id, mutation=(eve_mutator_id, {
        eve_attr1_id: Muta.roll_to_api(val=0.3),
        eve_attr2_id: Muta.roll_to_api(val=0.3)}))
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item1_id
    assert api_item.mutation.type == consts.ApiMutationType.effective
    assert api_item.mutation.base_type_id == eve_base_item1_id
    assert len(api_item.mutation.attrs) == 2
    assert api_item.mutation.attrs[eve_attr1_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr1_id].absolute == approx(92)
    assert api_item.mutation.attrs[eve_attr2_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr2_id].absolute == approx(92)
    assert len(api_item.mutation.rolls) == 2
    assert api_item.mutation.rolls[eve_attr1_id] == approx(0.3)
    assert api_item.mutation.rolls[eve_attr2_id] == approx(0.3)
    assert api_item.attrs[eve_attr1_id].base == approx(92)
    assert api_item.attrs[eve_attr2_id].base == approx(92)
    # Action
    api_item.change_module(type_id=eve_base_item2_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item2_id
    assert api_item.mutation.type == consts.ApiMutationType.effective
    assert api_item.mutation.base_type_id == eve_base_item2_id
    assert len(api_item.mutation.attrs) == 1
    assert api_item.mutation.attrs[eve_attr2_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr2_id].absolute == approx(276)
    assert len(api_item.mutation.rolls) == 2
    assert api_item.mutation.rolls[eve_attr1_id] == approx(0.3)
    assert api_item.mutation.rolls[eve_attr2_id] == approx(0.3)
    assert api_item.attrs[eve_attr2_id].base == approx(276)
    # Action
    api_item.change_module(type_id=eve_base_item1_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item1_id
    assert api_item.mutation.type == consts.ApiMutationType.effective
    assert api_item.mutation.base_type_id == eve_base_item1_id
    assert len(api_item.mutation.attrs) == 2
    assert api_item.mutation.attrs[eve_attr1_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr1_id].absolute == approx(92)
    assert api_item.mutation.attrs[eve_attr2_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr2_id].absolute == approx(92)
    assert len(api_item.mutation.rolls) == 2
    assert api_item.mutation.rolls[eve_attr1_id] == approx(0.3)
    assert api_item.mutation.rolls[eve_attr2_id] == approx(0.3)
    assert api_item.attrs[eve_attr1_id].base == approx(92)
    assert api_item.attrs[eve_attr2_id].base == approx(92)


def test_from_stage4_to_mutated_item_module(client, consts):
    # Type ID which effectively mutated item exposes is the mutated one, but type ID change sets
    # base type ID, so changing it to the exposed one has to switch base item
    eve_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item(attrs={eve_attr_id: 150})
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(
        type_id=eve_base_item_id,
        mutation=(eve_mutator_id, {eve_attr_id: Muta.roll_to_api(val=0.3)}))
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.mutation.type == consts.ApiMutationType.effective
    assert api_item.mutation.base_type_id == eve_base_item_id
    assert api_item.attrs[eve_attr_id].base == approx(92)
    # Action
    api_item.change_module(type_id=eve_mutated_item_id)
    # Verification - mutator has no mutation for the new base item
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.mutation.type == consts.ApiMutationType.dormant
    assert len(api_item.mutation.rolls) == 1
    assert api_item.mutation.rolls[eve_attr_id] == approx(0.3)
    assert api_item.attrs[eve_attr_id].base == approx(150)
    # Action
    api_item.change_module(type_id=eve_base_item_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.mutation.type == consts.ApiMutationType.effective
    assert api_item.mutation.base_type_id == eve_base_item_id
    assert len(api_item.mutation.attrs) == 1
    assert api_item.mutation.attrs[eve_attr_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr_id].absolute == approx(92)
    assert len(api_item.mutation.rolls) == 1
    assert api_item.mutation.rolls[eve_attr_id] == approx(0.3)
    assert api_item.attrs[eve_attr_id].base == approx(92)


def test_from_stage4_to_mutated_item_drone(client, consts):
    # Type ID which effectively mutated item exposes is the mutated one, but type ID change sets
    # base type ID, so changing it to the exposed one has to switch base item
    eve_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item(attrs={eve_attr_id: 150})
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_drone(
        type_id=eve_base_item_id,
        mutation=(eve_mutator_id, {eve_attr_id: Muta.roll_to_api(val=0.3)}))
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.mutation.type == consts.ApiMutationType.effective
    assert api_item.mutation.base_type_id == eve_base_item_id
    assert api_item.attrs[eve_attr_id].base == approx(92)
    # Action
    api_item.change_drone(type_id=eve_mutated_item_id)
    # Verification - mutator has no mutation for the new base item
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.mutation.type == consts.ApiMutationType.dormant
    assert len(api_item.mutation.rolls) == 1
    assert api_item.mutation.rolls[eve_attr_id] == approx(0.3)
    assert api_item.attrs[eve_attr_id].base == approx(150)
    # Action
    api_item.change_drone(type_id=eve_base_item_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.mutation.type == consts.ApiMutationType.effective
    assert api_item.mutation.base_type_id == eve_base_item_id
    assert len(api_item.mutation.attrs) == 1
    assert api_item.mutation.attrs[eve_attr_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr_id].absolute == approx(92)
    assert len(api_item.mutation.rolls) == 1
    assert api_item.mutation.rolls[eve_attr_id] == approx(0.3)
    assert api_item.attrs[eve_attr_id].base == approx(92)


def test_modification_incoming(client, consts):
    # Check that base type change correctly triggers recalculation of mutated attributes
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_base_item1_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20, eve_affectee_attr_id: 200})
    eve_base_item2_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20, eve_affectee_attr_id: 300})
    eve_mutated_item_id = client.mk_eve_item(eff_ids=[eve_effect_id])
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_item1_id, eve_base_item2_id], eve_mutated_item_id)],
        attrs={eve_affectee_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(
        type_id=eve_base_item1_id,
        mutation=(eve_mutator_id, {eve_affectee_attr_id: Muta.roll_to_api(val=0.3)}))
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].base == approx(184)
    assert api_item.attrs[eve_affectee_attr_id].modified == approx(220.8)
    # Action
    api_item.change_module(type_id=eve_base_item2_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.attrs[eve_affectee_attr_id].base == approx(276)
    assert api_item.attrs[eve_affectee_attr_id].modified == approx(331.2)


def test_modification_outgoing(client, consts):
    # Check that base type change correctly triggers recalculation of attributes modified by
    # attributes it changes, on the item itself and on other items
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_item_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_ship_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_charge_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_item_mod, eve_ship_mod, eve_charge_mod])
    eve_base_item1_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20, eve_affectee_attr_id: 200})
    eve_base_item2_id = client.mk_eve_item(attrs={eve_affector_attr_id: 30, eve_affectee_attr_id: 200})
    eve_mutated_item_id = client.mk_eve_item(eff_ids=[eve_effect_id])
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_item1_id, eve_base_item2_id], eve_mutated_item_id)],
        attrs={eve_affector_attr_id: (0.8, 1.2)})
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 200})
    eve_charge_id = client.mk_eve_charge(attrs={eve_affectee_attr_id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_item = api_fit.add_module(
        type_id=eve_base_item1_id,
        mutation=(eve_mutator_id, {eve_affector_attr_id: Muta.roll_to_api(val=0.3)}),
        charge_type_id=eve_charge_id)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affector_attr_id].base == approx(18.4)
    assert api_item.attrs[eve_affectee_attr_id].modified == approx(236.8)
    assert api_ship.update().attrs[eve_affectee_attr_id].modified == approx(236.8)
    assert api_item.charge.update().attrs[eve_affectee_attr_id].modified == approx(236.8)
    # Action
    api_item.change_module(type_id=eve_base_item2_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.attrs[eve_affector_attr_id].base == approx(27.6)
    assert api_item.attrs[eve_affectee_attr_id].modified == approx(255.2)
    assert api_ship.update().attrs[eve_affectee_attr_id].modified == approx(255.2)
    assert api_item.charge.update().attrs[eve_affectee_attr_id].modified == approx(255.2)


def test_modification_attr_added_removed(client, consts):
    # Affector attribute is defined only on one of base items. Before it appears, calculation uses
    # its default value, so base type change has to trigger recalculation of everything it modifies
    # both when it appears and when it disappears
    eve_affector_attr_id = client.mk_eve_attr(def_val=10)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_item_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_ship_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_item_mod, eve_ship_mod])
    eve_base_item1_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 200})
    eve_base_item2_id = client.mk_eve_item(attrs={eve_affector_attr_id: 30, eve_affectee_attr_id: 200})
    eve_mutated_item_id = client.mk_eve_item(eff_ids=[eve_effect_id])
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_item1_id, eve_base_item2_id], eve_mutated_item_id)])
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_item = api_fit.add_module(type_id=eve_base_item1_id, mutation=eve_mutator_id)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].modified == approx(220)
    assert api_ship.update().attrs[eve_affectee_attr_id].modified == approx(220)
    # Action
    api_item.change_module(type_id=eve_base_item2_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.attrs[eve_affector_attr_id].base == approx(30)
    assert api_item.attrs[eve_affectee_attr_id].modified == approx(260)
    assert api_ship.update().attrs[eve_affectee_attr_id].modified == approx(260)
    # Action
    api_item.change_module(type_id=eve_base_item1_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.attrs[eve_affectee_attr_id].modified == approx(220)
    assert api_ship.update().attrs[eve_affectee_attr_id].modified == approx(220)


def test_projection(client, consts):
    # Check that projection is properly reapplied if effects change
    eve_affector1_attr_id = client.mk_eve_attr()
    eve_affector2_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_modifier1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector1_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect1_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_modifier1])
    eve_modifier2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector2_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect2_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_modifier2])
    eve_base_item1_id = client.mk_eve_item(attrs={eve_affector1_attr_id: 20, eve_affector2_attr_id: 30})
    eve_base_item2_id = client.mk_eve_item(attrs={eve_affector1_attr_id: 40, eve_affector2_attr_id: 50})
    eve_mutated_item1_id = client.mk_eve_item(eff_ids=[eve_effect1_id], defeff_id=eve_effect1_id)
    eve_mutated_item2_id = client.mk_eve_item(eff_ids=[eve_effect2_id], defeff_id=eve_effect2_id)
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_item1_id], eve_mutated_item1_id), ([eve_base_item2_id], eve_mutated_item2_id)])
    eve_affectee_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_base_item1_id,
        state=consts.ApiModuleState.active,
        mutation=eve_mutator_id)
    api_affector_drone = api_affector_fit.add_drone(
        type_id=eve_base_item1_id,
        state=consts.ApiMinionState.engaging,
        mutation=eve_mutator_id)
    api_affectee1_fit = api_sol.create_fit()
    api_affectee1_ship = api_affectee1_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affectee2_fit = api_sol.create_fit()
    api_affectee2_ship = api_affectee2_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affector_module.change_module(add_proj_item_ids=[api_affectee1_ship.id])
    api_affector_drone.change_drone(add_proj_item_ids=[api_affectee2_ship.id])
    # Verification
    assert api_affectee1_ship.update().attrs[eve_affectee_attr_id].modified == approx(120)
    assert api_affectee2_ship.update().attrs[eve_affectee_attr_id].modified == approx(120)
    # Action
    api_affector_module.change_module(type_id=eve_base_item2_id)
    api_affector_drone.change_drone(type_id=eve_base_item2_id)
    # Verification
    assert api_affectee1_ship.update().attrs[eve_affectee_attr_id].modified == approx(150)
    assert api_affectee2_ship.update().attrs[eve_affectee_attr_id].modified == approx(150)
