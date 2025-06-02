from tests import approx, check_no_field


def test_from_stage2(client, consts):
    eve_attr_id = client.mk_eve_attr()
    eve_base_item1_id = client.mk_eve_item(attrs={eve_attr_id: 100})
    eve_base_item2_id = client.mk_eve_item(attrs={eve_attr_id: 200})
    eve_mutated_item2_id = client.alloc_item_id()
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_item2_id], eve_mutated_item2_id)],
        attrs={eve_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(
        type_id=eve_base_item1_id,
        mutation=(eve_mutator_id, {eve_attr_id: {consts.ApiAttrMutation.roll: 0.3}}))
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item1_id
    with check_no_field():
        api_item.mutation  # noqa: B018
    assert api_item.attrs[eve_attr_id].base == approx(100)
    # Action
    api_item.change_module(type_id=eve_base_item2_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item2_id
    with check_no_field():
        api_item.mutation  # noqa: B018
    assert api_item.attrs[eve_attr_id].base == approx(200)
    # Action
    api_item.change_module(type_id=eve_base_item1_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item1_id
    with check_no_field():
        api_item.mutation  # noqa: B018
    assert api_item.attrs[eve_attr_id].base == approx(100)


def test_from_stage3(client, consts):
    eve_attr_id = client.mk_eve_attr()
    eve_base_item1_id = client.mk_eve_item(attrs={eve_attr_id: 100})
    eve_base_item2_id = client.mk_eve_item(attrs={eve_attr_id: 200})
    eve_mutated_item1_id = client.alloc_item_id()
    eve_mutated_item2_id = client.alloc_item_id()
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_item1_id], eve_mutated_item1_id), ([eve_base_item2_id], eve_mutated_item2_id)],
        attrs={eve_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(
        type_id=eve_base_item1_id,
        mutation=(eve_mutator_id, {eve_attr_id: {consts.ApiAttrMutation.roll: 0.3}}))
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item1_id
    with check_no_field():
        api_item.mutation  # noqa: B018
    assert api_item.attrs[eve_attr_id].base == approx(100)
    # Action
    api_item.change_module(type_id=eve_base_item2_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item2_id
    with check_no_field():
        api_item.mutation  # noqa: B018
    assert api_item.attrs[eve_attr_id].base == approx(200)
    # Action
    api_item.change_module(type_id=eve_base_item1_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item1_id
    with check_no_field():
        api_item.mutation  # noqa: B018
    assert api_item.attrs[eve_attr_id].base == approx(100)


def test_from_stage4(client, consts):
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_base_item1_id = client.mk_eve_item(attrs={eve_attr1_id: 100, eve_attr2_id: 100})
    eve_base_item2_id = client.mk_eve_item(attrs={eve_attr1_id: 200, eve_attr2_id: 200})
    eve_mutated_item1_id = client.mk_eve_item(attrs={eve_attr2_id: 50})
    eve_mutated_item2_id = client.alloc_item_id()
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_item1_id], eve_mutated_item1_id), ([eve_base_item2_id], eve_mutated_item2_id)],
        attrs={eve_attr1_id: (0.8, 1.2), eve_attr2_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(type_id=eve_base_item1_id, mutation=(eve_mutator_id, {
        eve_attr1_id: {consts.ApiAttrMutation.roll: 0.3},
        eve_attr2_id: {consts.ApiAttrMutation.roll: 0.3}}))
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item1_id
    assert api_item.mutation.base_type_id == eve_base_item1_id
    assert len(api_item.mutation.attrs) == 2
    assert api_item.mutation.attrs[eve_attr1_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr1_id].absolute == approx(92)
    assert api_item.mutation.attrs[eve_attr2_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr2_id].absolute == approx(46)
    assert api_item.attrs[eve_attr1_id].base == approx(92)
    assert api_item.attrs[eve_attr2_id].base == approx(46)
    # Action
    api_item.change_module(type_id=eve_base_item2_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item2_id
    with check_no_field():
        api_item.mutation  # noqa: B018
    assert api_item.attrs[eve_attr1_id].base == approx(200)
    assert api_item.attrs[eve_attr2_id].base == approx(200)
    # Action
    api_item.change_module(type_id=eve_base_item1_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item1_id
    assert api_item.mutation.base_type_id == eve_base_item1_id
    assert len(api_item.mutation.attrs) == 2
    assert api_item.mutation.attrs[eve_attr1_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr1_id].absolute == approx(92)
    assert api_item.mutation.attrs[eve_attr2_id].roll == approx(0.3)
    assert api_item.mutation.attrs[eve_attr2_id].absolute == approx(46)
    assert api_item.attrs[eve_attr1_id].base == approx(92)
    assert api_item.attrs[eve_attr2_id].base == approx(46)
