from tests import approx, check_no_field


def test_from_stage1(client, consts):
    eve_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item()
    eve_mutator1_id = client.alloc_item_id()
    eve_mutator2_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(
        type_id=eve_base_item_id,
        mutation=(eve_mutator1_id, {eve_attr_id: {consts.ApiAttrMutation.roll: 0.3}}))
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item_id
    with check_no_field():
        api_item.mutation  # noqa: B018
    assert api_item.attrs[eve_attr_id].base == approx(100)
    # Action
    api_item.change_module(mutation=eve_mutator2_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.mutation.base_type_id == eve_base_item_id
    assert api_item.mutation.mutator_id == eve_mutator2_id
    assert len(api_item.mutation.attrs) == 1
    assert api_item.mutation.attrs[eve_attr_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr_id].absolute == approx(92)
    assert api_item.attrs[eve_attr_id].base == approx(92)


def test_from_stage2(client, consts):
    eve_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item()
    eve_other_base_item_id = client.mk_eve_item()
    eve_other_mutated_item_id = client.mk_eve_item()
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
        mutation=(eve_mutator1_id, {eve_attr_id: {consts.ApiAttrMutation.roll: 0.3}}))
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item_id
    with check_no_field():
        api_item.mutation  # noqa: B018
    assert api_item.attrs[eve_attr_id].base == approx(100)
    # Action
    api_item.change_module(mutation=eve_mutator2_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.mutation.base_type_id == eve_base_item_id
    assert api_item.mutation.mutator_id == eve_mutator2_id
    assert len(api_item.mutation.attrs) == 1
    assert api_item.mutation.attrs[eve_attr_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr_id].absolute == approx(109)
    assert api_item.attrs[eve_attr_id].base == approx(109)


def test_from_stage3(client, consts):
    eve_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_attr_id: 100})
    eve_mutated_item1_id = client.alloc_item_id()
    eve_mutated_item2_id = client.mk_eve_item()
    eve_mutator1_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item1_id)],
        attrs={eve_attr_id: (0.8, 1.2)})
    eve_mutator2_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item2_id)],
        attrs={eve_attr_id: (1, 1.3)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(
        type_id=eve_base_item_id,
        mutation=(eve_mutator1_id, {eve_attr_id: {consts.ApiAttrMutation.roll: 0.3}}))
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item_id
    with check_no_field():
        api_item.mutation  # noqa: B018
    assert api_item.attrs[eve_attr_id].base == approx(100)
    # Action
    api_item.change_module(mutation=eve_mutator2_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item2_id
    assert api_item.mutation.base_type_id == eve_base_item_id
    assert api_item.mutation.mutator_id == eve_mutator2_id
    assert len(api_item.mutation.attrs) == 1
    assert api_item.mutation.attrs[eve_attr_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr_id].absolute == approx(109)
    assert api_item.attrs[eve_attr_id].base == approx(109)


def test_from_stage4_range_value(client, consts):
    # Base scenario, where only mutation ranges change
    eve_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item()
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
        mutation=(eve_mutator1_id, {eve_attr_id: {consts.ApiAttrMutation.roll: 0.3}}))
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.mutation.base_type_id == eve_base_item_id
    assert api_item.mutation.mutator_id == eve_mutator1_id
    assert len(api_item.mutation.attrs) == 1
    assert api_item.mutation.attrs[eve_attr_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr_id].absolute == approx(92)
    assert api_item.attrs[eve_attr_id].base == approx(92)
    # Action
    api_item.change_module(mutation=eve_mutator2_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.mutation.base_type_id == eve_base_item_id
    assert api_item.mutation.mutator_id == eve_mutator2_id
    assert len(api_item.mutation.attrs) == 1
    assert api_item.mutation.attrs[eve_attr_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr_id].absolute == approx(109)
    assert api_item.attrs[eve_attr_id].base == approx(109)


def test_from_stage4_attr_ranges(client, consts):
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_attr1_id: 100, eve_attr2_id: 100})
    eve_mutated_item_id = client.mk_eve_item()
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
        eve_attr1_id: {consts.ApiAttrMutation.roll: 0.3},
        eve_attr2_id: {consts.ApiAttrMutation.roll: 0.3}}))
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.mutation.base_type_id == eve_base_item_id
    assert api_item.mutation.mutator_id == eve_mutator1_id
    assert len(api_item.mutation.attrs) == 1
    assert api_item.mutation.attrs[eve_attr1_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr1_id].absolute == approx(92)
    assert api_item.attrs[eve_attr1_id].base == approx(92)
    assert api_item.attrs[eve_attr2_id].base == approx(100)
    # Action
    api_item.change_module(mutation=eve_mutator2_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.mutation.base_type_id == eve_base_item_id
    assert api_item.mutation.mutator_id == eve_mutator2_id
    assert len(api_item.mutation.attrs) == 1
    assert api_item.mutation.attrs[eve_attr2_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr2_id].absolute == approx(92)
    assert api_item.attrs[eve_attr1_id].base == approx(100)
    assert api_item.attrs[eve_attr2_id].base == approx(92)


def test_from_stage4_mutated_item(client, consts):
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_attr1_id: 100, eve_attr2_id: 100})
    eve_mutated_item1_id = client.mk_eve_item()
    eve_mutated_item2_id = client.mk_eve_item(attrs={eve_attr2_id: 200})
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
        eve_attr1_id: {consts.ApiAttrMutation.roll: 0.3},
        eve_attr2_id: {consts.ApiAttrMutation.roll: 0.3}}))
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item1_id
    assert api_item.mutation.base_type_id == eve_base_item_id
    assert api_item.mutation.mutator_id == eve_mutator1_id
    assert len(api_item.mutation.attrs) == 2
    assert api_item.mutation.attrs[eve_attr1_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr1_id].absolute == approx(92)
    assert api_item.mutation.attrs[eve_attr2_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr2_id].absolute == approx(92)
    assert api_item.attrs[eve_attr1_id].base == approx(92)
    assert api_item.attrs[eve_attr2_id].base == approx(92)
    # Action
    api_item.change_module(mutation=eve_mutator2_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item2_id
    assert api_item.mutation.base_type_id == eve_base_item_id
    assert api_item.mutation.mutator_id == eve_mutator2_id
    assert len(api_item.mutation.attrs) == 2
    assert api_item.mutation.attrs[eve_attr1_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr1_id].absolute == approx(92)
    assert api_item.mutation.attrs[eve_attr2_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr2_id].absolute == approx(184)
    assert api_item.attrs[eve_attr1_id].base == approx(92)
    assert api_item.attrs[eve_attr2_id].base == approx(184)
