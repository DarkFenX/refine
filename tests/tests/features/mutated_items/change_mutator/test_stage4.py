from fw import Muta, approx, check_no_field


def test_from_stage1_base_loaded(client, consts):
    eve_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item(is_dynamic=True)
    eve_mutator1_id = client.alloc_item_id()
    eve_mutator2_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(
        type_id=eve_base_item_id,
        mutation=(eve_mutator1_id, {eve_attr_id: Muta.roll_to_api(val=0.3)}))
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item_id
    assert api_item.mutation.type == consts.ApiMutationType.dormant
    assert api_item.mutation.mutator_id == eve_mutator1_id
    assert len(api_item.mutation.rolls) == 1
    assert api_item.mutation.rolls[eve_attr_id] == approx(0.3)
    assert api_item.attrs[eve_attr_id].base == approx(100)
    # Action
    api_item.change_module(mutation=eve_mutator2_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.mutation.type == consts.ApiMutationType.effective
    assert api_item.mutation.base_type_id == eve_base_item_id
    assert api_item.mutation.mutator_id == eve_mutator2_id
    assert len(api_item.mutation.attrs) == 1
    assert api_item.mutation.attrs[eve_attr_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr_id].absolute == approx(92)
    assert len(api_item.mutation.rolls) == 1
    assert api_item.mutation.rolls[eve_attr_id] == approx(0.3)
    assert api_item.attrs[eve_attr_id].base == approx(92)


def test_from_stage2(client, consts):
    eve_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item(is_dynamic=True)
    eve_other_base_item_id = client.mk_eve_item()
    eve_other_mutated_item_id = client.mk_eve_item(is_dynamic=True)
    eve_mutator1_id = client.mk_eve_mutator(
        items=[([eve_other_base_item_id], eve_other_mutated_item_id)],
        attrs={eve_attr_id: (0.8, 1.2)})
    eve_mutator2_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_attr_id: (1, 1.3)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(
        type_id=eve_base_item_id,
        mutation=(eve_mutator1_id, {eve_attr_id: Muta.roll_to_api(val=0.3)}))
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item_id
    assert api_item.mutation.type == consts.ApiMutationType.dormant
    assert api_item.mutation.mutator_id == eve_mutator1_id
    assert len(api_item.mutation.rolls) == 1
    assert api_item.mutation.rolls[eve_attr_id] == approx(0.3)
    assert api_item.attrs[eve_attr_id].base == approx(100)
    # Action
    api_item.change_module(mutation=eve_mutator2_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.mutation.type == consts.ApiMutationType.effective
    assert api_item.mutation.base_type_id == eve_base_item_id
    assert api_item.mutation.mutator_id == eve_mutator2_id
    assert len(api_item.mutation.attrs) == 1
    assert api_item.mutation.attrs[eve_attr_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr_id].absolute == approx(109)
    assert len(api_item.mutation.rolls) == 1
    assert api_item.mutation.rolls[eve_attr_id] == approx(0.3)
    assert api_item.attrs[eve_attr_id].base == approx(109)


def test_from_stage3(client, consts):
    eve_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_attr_id: 100})
    eve_mutated_item1_id = client.alloc_item_id()
    eve_mutated_item2_id = client.mk_eve_item(is_dynamic=True)
    eve_mutator1_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item1_id)],
        attrs={eve_attr_id: (0.8, 1.2)})
    eve_mutator2_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item2_id)],
        attrs={eve_attr_id: (1, 1.3)})
    client.create_sources(cleanup_check=False)
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(
        type_id=eve_base_item_id,
        mutation=(eve_mutator1_id, {eve_attr_id: Muta.roll_to_api(val=0.3)}))
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item_id
    assert api_item.mutation.type == consts.ApiMutationType.dormant
    assert api_item.mutation.mutator_id == eve_mutator1_id
    assert len(api_item.mutation.rolls) == 1
    assert api_item.mutation.rolls[eve_attr_id] == approx(0.3)
    assert api_item.attrs[eve_attr_id].base == approx(100)
    # Action
    api_item.change_module(mutation=eve_mutator2_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item2_id
    assert api_item.mutation.type == consts.ApiMutationType.effective
    assert api_item.mutation.base_type_id == eve_base_item_id
    assert api_item.mutation.mutator_id == eve_mutator2_id
    assert len(api_item.mutation.attrs) == 1
    assert api_item.mutation.attrs[eve_attr_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr_id].absolute == approx(109)
    assert len(api_item.mutation.rolls) == 1
    assert api_item.mutation.rolls[eve_attr_id] == approx(0.3)
    assert api_item.attrs[eve_attr_id].base == approx(109)


def test_from_stage4_range_value(client, consts):
    # Base scenario, where only mutation ranges change
    eve_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item(is_dynamic=True)
    eve_mutator1_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_attr_id: (0.8, 1.2)})
    eve_mutator2_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_attr_id: (1, 1.3)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(
        type_id=eve_base_item_id,
        mutation=(eve_mutator1_id, {eve_attr_id: Muta.roll_to_api(val=0.3)}))
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.mutation.type == consts.ApiMutationType.effective
    assert api_item.mutation.base_type_id == eve_base_item_id
    assert api_item.mutation.mutator_id == eve_mutator1_id
    assert len(api_item.mutation.attrs) == 1
    assert api_item.mutation.attrs[eve_attr_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr_id].absolute == approx(92)
    assert len(api_item.mutation.rolls) == 1
    assert api_item.mutation.rolls[eve_attr_id] == approx(0.3)
    assert api_item.attrs[eve_attr_id].base == approx(92)
    # Action
    api_item.change_module(mutation=eve_mutator2_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.mutation.type == consts.ApiMutationType.effective
    assert api_item.mutation.base_type_id == eve_base_item_id
    assert api_item.mutation.mutator_id == eve_mutator2_id
    assert len(api_item.mutation.attrs) == 1
    assert api_item.mutation.attrs[eve_attr_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr_id].absolute == approx(109)
    assert len(api_item.mutation.rolls) == 1
    assert api_item.mutation.rolls[eve_attr_id] == approx(0.3)
    assert api_item.attrs[eve_attr_id].base == approx(109)


def test_from_stage4_attr_ranges(client, consts):
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_attr1_id: 100, eve_attr2_id: 100})
    eve_mutated_item_id = client.mk_eve_item(is_dynamic=True)
    eve_mutator1_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_attr1_id: (0.8, 1.2)})
    eve_mutator2_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_attr2_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(type_id=eve_base_item_id, mutation=(eve_mutator1_id, {
        eve_attr1_id: Muta.roll_to_api(val=0.3),
        eve_attr2_id: Muta.roll_to_api(val=0.3)}))
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.mutation.type == consts.ApiMutationType.effective
    assert api_item.mutation.base_type_id == eve_base_item_id
    assert api_item.mutation.mutator_id == eve_mutator1_id
    assert len(api_item.mutation.attrs) == 1
    assert api_item.mutation.attrs[eve_attr1_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr1_id].absolute == approx(92)
    assert len(api_item.mutation.rolls) == 2
    assert api_item.mutation.rolls[eve_attr1_id] == approx(0.3)
    assert api_item.mutation.rolls[eve_attr2_id] == approx(0.3)
    assert api_item.attrs[eve_attr1_id].base == approx(92)
    assert api_item.attrs[eve_attr2_id].base == approx(100)
    # Action
    api_item.change_module(mutation=eve_mutator2_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.mutation.type == consts.ApiMutationType.effective
    assert api_item.mutation.base_type_id == eve_base_item_id
    assert api_item.mutation.mutator_id == eve_mutator2_id
    assert len(api_item.mutation.attrs) == 1
    assert api_item.mutation.attrs[eve_attr2_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr2_id].absolute == approx(92)
    assert len(api_item.mutation.rolls) == 2
    assert api_item.mutation.rolls[eve_attr1_id] == approx(0.3)
    assert api_item.mutation.rolls[eve_attr2_id] == approx(0.3)
    assert api_item.attrs[eve_attr1_id].base == approx(100)
    assert api_item.attrs[eve_attr2_id].base == approx(92)


def test_from_stage4_mutated_item(client, consts):
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_attr1_id: 100, eve_attr2_id: 100})
    eve_mutated_item1_id = client.mk_eve_item(is_dynamic=True)
    eve_mutated_item2_id = client.mk_eve_item(is_dynamic=True, attrs={eve_attr2_id: 200})
    eve_mutator1_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item1_id)],
        attrs={eve_attr1_id: (0.8, 1.2), eve_attr2_id: (0.8, 1.2)})
    eve_mutator2_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item2_id)],
        attrs={eve_attr1_id: (0.8, 1.2), eve_attr2_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(type_id=eve_base_item_id, mutation=(eve_mutator1_id, {
        eve_attr1_id: Muta.roll_to_api(val=0.3),
        eve_attr2_id: Muta.roll_to_api(val=0.3)}))
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item1_id
    assert api_item.mutation.type == consts.ApiMutationType.effective
    assert api_item.mutation.base_type_id == eve_base_item_id
    assert api_item.mutation.mutator_id == eve_mutator1_id
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
    api_item.change_module(mutation=eve_mutator2_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item2_id
    assert api_item.mutation.type == consts.ApiMutationType.effective
    assert api_item.mutation.base_type_id == eve_base_item_id
    assert api_item.mutation.mutator_id == eve_mutator2_id
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


def test_from_stage4_unmutated_values(client, consts):
    # Unmutated values come from base item, and from mutated item for attributes base item does not
    # have; attributes neither item has are not exposed
    eve_base_attr_id = client.mk_eve_attr()
    eve_mutated_attr_id = client.mk_eve_attr()
    eve_dropped_attr_id = client.mk_eve_attr()
    eve_absent1_attr_id = client.mk_eve_attr()
    eve_absent2_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_base_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item(is_dynamic=True, attrs={
        eve_base_attr_id: 300,
        eve_mutated_attr_id: 200,
        eve_dropped_attr_id: 200})
    eve_mutator1_id = client.mk_eve_mutator(items=[([eve_base_item_id], eve_mutated_item_id)], attrs={
        eve_base_attr_id: (0.8, 1.2),
        eve_mutated_attr_id: (0.8, 1.2),
        eve_dropped_attr_id: (0.8, 1.2),
        eve_absent1_attr_id: (0.8, 1.2)})
    eve_mutator2_id = client.mk_eve_mutator(items=[([eve_base_item_id], eve_mutated_item_id)], attrs={
        eve_base_attr_id: (1, 1.3),
        eve_mutated_attr_id: (1, 1.3),
        eve_absent2_attr_id: (1, 1.3)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(type_id=eve_base_item_id, mutation=(eve_mutator1_id, {
        eve_base_attr_id: Muta.roll_to_api(val=0.3),
        eve_mutated_attr_id: Muta.roll_to_api(val=0.3),
        eve_dropped_attr_id: Muta.roll_to_api(val=0.3),
        eve_absent1_attr_id: Muta.roll_to_api(val=0.3),
        eve_absent2_attr_id: Muta.roll_to_api(val=0.3)}))
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.mutation.type == consts.ApiMutationType.effective
    assert api_item.mutation.mutator_id == eve_mutator1_id
    assert len(api_item.mutation.attrs) == 3
    assert api_item.mutation.attrs[eve_base_attr_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_base_attr_id].absolute == approx(92)
    assert api_item.mutation.attrs[eve_mutated_attr_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_mutated_attr_id].absolute == approx(184)
    assert api_item.mutation.attrs[eve_dropped_attr_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_dropped_attr_id].absolute == approx(184)
    assert len(api_item.mutation.rolls) == 5
    assert api_item.attrs[eve_base_attr_id].base == approx(92)
    assert api_item.attrs[eve_mutated_attr_id].base == approx(184)
    assert api_item.attrs[eve_dropped_attr_id].base == approx(184)
    assert eve_absent1_attr_id not in api_item.attrs
    assert eve_absent2_attr_id not in api_item.attrs
    # Action
    api_item.change_module(mutation=eve_mutator2_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.mutation.type == consts.ApiMutationType.effective
    assert api_item.mutation.mutator_id == eve_mutator2_id
    assert len(api_item.mutation.attrs) == 2
    assert api_item.mutation.attrs[eve_base_attr_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_base_attr_id].absolute == approx(109)
    assert api_item.mutation.attrs[eve_mutated_attr_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_mutated_attr_id].absolute == approx(218)
    assert len(api_item.mutation.rolls) == 5
    assert api_item.attrs[eve_base_attr_id].base == approx(109)
    assert api_item.attrs[eve_mutated_attr_id].base == approx(218)
    assert api_item.attrs[eve_dropped_attr_id].base == approx(200)
    assert eve_absent1_attr_id not in api_item.attrs
    assert eve_absent2_attr_id not in api_item.attrs


def test_from_stage4_no_roll(client, consts):
    # Values of attributes without rolls are limited by ranges of the new mutator
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_attr1_id: 100, eve_attr2_id: 100})
    eve_mutated_item_id = client.mk_eve_item(is_dynamic=True)
    eve_mutator1_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_attr1_id: (0.8, 1.2), eve_attr2_id: (1.1, 1.3)})
    eve_mutator2_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_attr1_id: (0.7, 0.9), eve_attr2_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(type_id=eve_base_item_id, mutation=eve_mutator1_id)
    # Verification
    api_item.update()
    assert api_item.mutation.type == consts.ApiMutationType.effective
    assert api_item.mutation.mutator_id == eve_mutator1_id
    assert len(api_item.mutation.attrs) == 2
    assert api_item.mutation.attrs[eve_attr1_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_attr1_id].absolute == approx(100)
    assert api_item.mutation.attrs[eve_attr2_id].roll == approx(0)
    assert api_item.mutation.attrs[eve_attr2_id].absolute == approx(110)
    with check_no_field():
        api_item.mutation.rolls  # ruff:ignore[useless-expression]
    assert api_item.attrs[eve_attr1_id].base == approx(100)
    assert api_item.attrs[eve_attr2_id].base == approx(110)
    # Action
    api_item.change_module(mutation=eve_mutator2_id)
    # Verification
    api_item.update()
    assert api_item.mutation.type == consts.ApiMutationType.effective
    assert api_item.mutation.mutator_id == eve_mutator2_id
    assert len(api_item.mutation.attrs) == 2
    assert api_item.mutation.attrs[eve_attr1_id].roll == approx(1)
    assert api_item.mutation.attrs[eve_attr1_id].absolute == approx(90)
    assert api_item.mutation.attrs[eve_attr2_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_attr2_id].absolute == approx(100)
    with check_no_field():
        api_item.mutation.rolls  # ruff:ignore[useless-expression]
    assert api_item.attrs[eve_attr1_id].base == approx(90)
    assert api_item.attrs[eve_attr2_id].base == approx(100)


def test_from_stage4_attr_change(client, consts):
    # Rolls and absolute values set after mutator change use ranges of the new mutator
    eve_roll_attr_id = client.mk_eve_attr()
    eve_absolute_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_roll_attr_id: 100, eve_absolute_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item(is_dynamic=True)
    eve_mutator1_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_roll_attr_id: (0.8, 1.2), eve_absolute_attr_id: (0.8, 1.2)})
    eve_mutator2_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_roll_attr_id: (1, 1.3), eve_absolute_attr_id: (1, 1.3)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(type_id=eve_base_item_id, mutation=(eve_mutator1_id, {
        eve_roll_attr_id: Muta.roll_to_api(val=0.3),
        eve_absolute_attr_id: Muta.roll_to_api(val=0.3)}))
    api_item.change_module(mutation=eve_mutator2_id)
    # Verification
    api_item.update()
    assert api_item.mutation.mutator_id == eve_mutator2_id
    assert api_item.attrs[eve_roll_attr_id].base == approx(109)
    assert api_item.attrs[eve_absolute_attr_id].base == approx(109)
    # Action
    api_item.change_module(mutation={
        eve_roll_attr_id: Muta.roll_to_api(val=0.5),
        eve_absolute_attr_id: Muta.abs_to_api(val=120)})
    # Verification
    api_item.update()
    assert api_item.mutation.type == consts.ApiMutationType.effective
    assert api_item.mutation.mutator_id == eve_mutator2_id
    assert len(api_item.mutation.attrs) == 2
    assert api_item.mutation.attrs[eve_roll_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_roll_attr_id].absolute == approx(115)
    assert api_item.mutation.attrs[eve_absolute_attr_id].roll == approx(0.6666667)
    assert api_item.mutation.attrs[eve_absolute_attr_id].absolute == approx(120)
    assert len(api_item.mutation.rolls) == 2
    assert api_item.mutation.rolls[eve_roll_attr_id] == approx(0.5)
    assert api_item.mutation.rolls[eve_absolute_attr_id] == approx(0.6666667)
    assert api_item.attrs[eve_roll_attr_id].base == approx(115)
    assert api_item.attrs[eve_absolute_attr_id].base == approx(120)


def test_modification_incoming(client, consts):
    # Check that mutator change correctly triggers recalculation of mutated attributes
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_base_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20, eve_affectee_attr_id: 200})
    eve_mutated_item_id = client.mk_eve_item(is_dynamic=True, eff_ids=[eve_effect_id])
    eve_mutator1_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_affectee_attr_id: (0.8, 1.2)})
    eve_mutator2_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_affectee_attr_id: (1, 1.3)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(
        type_id=eve_base_item_id,
        mutation=(eve_mutator1_id, {eve_affectee_attr_id: Muta.roll_to_api(val=0.3)}))
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].base == approx(184)
    assert api_item.attrs[eve_affectee_attr_id].modified == approx(220.8)
    # Action
    api_item.change_module(mutation=eve_mutator2_id)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].base == approx(218)
    assert api_item.attrs[eve_affectee_attr_id].modified == approx(261.6)


def test_modification_outgoing(client, consts):
    # Check that mutator change correctly triggers recalculation of attributes modified by mutated
    # attributes, on the item itself and on other items
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
    eve_base_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20, eve_affectee_attr_id: 200})
    eve_mutated_item_id = client.mk_eve_item(is_dynamic=True, eff_ids=[eve_effect_id])
    eve_mutator1_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_affector_attr_id: (0.8, 1.2)})
    eve_mutator2_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_affector_attr_id: (1, 1.3)})
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 200})
    eve_charge_id = client.mk_eve_charge(attrs={eve_affectee_attr_id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_item = api_fit.add_module(
        type_id=eve_base_item_id,
        mutation=(eve_mutator1_id, {eve_affector_attr_id: Muta.roll_to_api(val=0.3)}),
        charge_type_id=eve_charge_id)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affector_attr_id].base == approx(18.4)
    assert api_item.attrs[eve_affectee_attr_id].modified == approx(236.8)
    assert api_ship.update().attrs[eve_affectee_attr_id].modified == approx(236.8)
    assert api_item.charge.update().attrs[eve_affectee_attr_id].modified == approx(236.8)
    # Action
    api_item.change_module(mutation=eve_mutator2_id)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affector_attr_id].base == approx(21.8)
    assert api_item.attrs[eve_affectee_attr_id].modified == approx(243.6)
    assert api_ship.update().attrs[eve_affectee_attr_id].modified == approx(243.6)
    assert api_item.charge.update().attrs[eve_affectee_attr_id].modified == approx(243.6)


def test_projection_effect(client, consts):
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
    eve_base_item_id = client.mk_eve_item(attrs={eve_affector1_attr_id: 20, eve_affector2_attr_id: 30})
    eve_mutated_item1_id = client.mk_eve_item(is_dynamic=True, eff_ids=[eve_effect1_id], defeff_id=eve_effect1_id)
    eve_mutated_item2_id = client.mk_eve_item(is_dynamic=True, eff_ids=[eve_effect2_id], defeff_id=eve_effect2_id)
    eve_mutator1_id = client.mk_eve_mutator(items=[([eve_base_item_id], eve_mutated_item1_id)])
    eve_mutator2_id = client.mk_eve_mutator(items=[([eve_base_item_id], eve_mutated_item2_id)])
    eve_affectee_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_base_item_id,
        state=consts.ApiModuleState.active,
        mutation=eve_mutator1_id)
    api_affector_drone = api_affector_fit.add_drone(
        type_id=eve_base_item_id,
        state=consts.ApiMinionState.engaging,
        mutation=eve_mutator1_id)
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
    api_affector_module.change_module(mutation=eve_mutator2_id)
    api_affector_drone.change_drone(mutation=eve_mutator2_id)
    # Verification
    assert api_affectee1_ship.update().attrs[eve_affectee_attr_id].modified == approx(130)
    assert api_affectee2_ship.update().attrs[eve_affectee_attr_id].modified == approx(130)


def test_projection_value(client, consts):
    # Check that projected modifications are recalculated if only mutated values change
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_base_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20})
    eve_mutated_item_id = client.mk_eve_item(is_dynamic=True, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_mutator1_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_affector_attr_id: (0.8, 1.2)})
    eve_mutator2_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_affector_attr_id: (1, 1.3)})
    eve_affectee_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_base_item_id,
        state=consts.ApiModuleState.active,
        mutation=(eve_mutator1_id, {eve_affector_attr_id: Muta.roll_to_api(val=0.3)}))
    api_affector_drone = api_affector_fit.add_drone(
        type_id=eve_base_item_id,
        state=consts.ApiMinionState.engaging,
        mutation=(eve_mutator1_id, {eve_affector_attr_id: Muta.roll_to_api(val=0.3)}))
    api_affectee1_fit = api_sol.create_fit()
    api_affectee1_ship = api_affectee1_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affectee2_fit = api_sol.create_fit()
    api_affectee2_ship = api_affectee2_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affector_module.change_module(add_proj_item_ids=[api_affectee1_ship.id])
    api_affector_drone.change_drone(add_proj_item_ids=[api_affectee2_ship.id])
    # Verification
    assert api_affectee1_ship.update().attrs[eve_affectee_attr_id].modified == approx(118.4)
    assert api_affectee2_ship.update().attrs[eve_affectee_attr_id].modified == approx(118.4)
    # Action
    api_affector_module.change_module(mutation=eve_mutator2_id)
    api_affector_drone.change_drone(mutation=eve_mutator2_id)
    # Verification
    assert api_affectee1_ship.update().attrs[eve_affectee_attr_id].modified == approx(121.8)
    assert api_affectee2_ship.update().attrs[eve_affectee_attr_id].modified == approx(121.8)


def test_drone(client, consts):
    eve_dropped_attr_id = client.mk_eve_attr()
    eve_changed_attr_id = client.mk_eve_attr()
    eve_added_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_drone(attrs={
        eve_dropped_attr_id: 100,
        eve_changed_attr_id: 100,
        eve_added_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_drone(is_dynamic=True)
    eve_mutator1_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_dropped_attr_id: (0.8, 1.2), eve_changed_attr_id: (0.8, 1.2)})
    eve_mutator2_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_changed_attr_id: (1, 1.3), eve_added_attr_id: (1.1, 1.3)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_base_item_id, mutation=(eve_mutator1_id, {
        eve_dropped_attr_id: Muta.roll_to_api(val=0.3),
        eve_changed_attr_id: Muta.roll_to_api(val=0.3)}))
    # Verification
    api_drone.update()
    assert api_drone.type_id == eve_mutated_item_id
    assert api_drone.mutation.type == consts.ApiMutationType.effective
    assert api_drone.mutation.base_type_id == eve_base_item_id
    assert api_drone.mutation.mutator_id == eve_mutator1_id
    assert len(api_drone.mutation.attrs) == 2
    assert api_drone.mutation.attrs[eve_dropped_attr_id].roll == approx(0.3)
    assert api_drone.mutation.attrs[eve_dropped_attr_id].absolute == approx(92)
    assert api_drone.mutation.attrs[eve_changed_attr_id].roll == approx(0.3)
    assert api_drone.mutation.attrs[eve_changed_attr_id].absolute == approx(92)
    assert len(api_drone.mutation.rolls) == 2
    assert api_drone.mutation.rolls[eve_dropped_attr_id] == approx(0.3)
    assert api_drone.mutation.rolls[eve_changed_attr_id] == approx(0.3)
    assert api_drone.attrs[eve_dropped_attr_id].base == approx(92)
    assert api_drone.attrs[eve_changed_attr_id].base == approx(92)
    assert api_drone.attrs[eve_added_attr_id].base == approx(100)
    # Action
    api_drone.change_drone(mutation=eve_mutator2_id)
    # Verification
    api_drone.update()
    assert api_drone.type_id == eve_mutated_item_id
    assert api_drone.mutation.type == consts.ApiMutationType.effective
    assert api_drone.mutation.base_type_id == eve_base_item_id
    assert api_drone.mutation.mutator_id == eve_mutator2_id
    assert len(api_drone.mutation.attrs) == 2
    assert api_drone.mutation.attrs[eve_changed_attr_id].roll == approx(0.3)
    assert api_drone.mutation.attrs[eve_changed_attr_id].absolute == approx(109)
    assert api_drone.mutation.attrs[eve_added_attr_id].roll == approx(0)
    assert api_drone.mutation.attrs[eve_added_attr_id].absolute == approx(110)
    assert len(api_drone.mutation.rolls) == 2
    assert api_drone.mutation.rolls[eve_dropped_attr_id] == approx(0.3)
    assert api_drone.mutation.rolls[eve_changed_attr_id] == approx(0.3)
    assert api_drone.attrs[eve_dropped_attr_id].base == approx(100)
    assert api_drone.attrs[eve_changed_attr_id].base == approx(109)
    assert api_drone.attrs[eve_added_attr_id].base == approx(110)
