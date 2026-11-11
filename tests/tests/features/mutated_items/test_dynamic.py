from fw import Muta, approx, check_no_field


def test_bare_module(client):
    # Dynamic types are loaded only as mutated items, not on their own
    eve_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item(is_dynamic=True, attrs={eve_attr_id: 150})
    client.mk_eve_mutator(items=[([eve_base_item_id], eve_mutated_item_id)], attrs={eve_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(type_id=eve_mutated_item_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    with check_no_field():
        api_item.mutation  # ruff:ignore[useless-expression]
    with check_no_field():
        api_item.attrs  # ruff:ignore[useless-expression]


def test_bare_drone(client):
    # Dynamic types are loaded only as mutated items, not on their own
    eve_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item(is_dynamic=True, attrs={eve_attr_id: 150})
    client.mk_eve_mutator(items=[([eve_base_item_id], eve_mutated_item_id)], attrs={eve_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_drone(type_id=eve_mutated_item_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    with check_no_field():
        api_item.mutation  # ruff:ignore[useless-expression]
    with check_no_field():
        api_item.attrs  # ruff:ignore[useless-expression]


def test_src_switch(client, consts):
    # Same type is regular on one source and dynamic on another
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_base_item_id = client.mk_eve_item(datas=[eve_d1, eve_d2], attrs={eve_attr_id: 100})
    eve_mutated_item_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_item(datas=[eve_d1], id_=eve_mutated_item_id, attrs={eve_attr_id: 150})
    client.mk_eve_item(datas=[eve_d2], id_=eve_mutated_item_id, is_dynamic=True, attrs={eve_attr_id: 150})
    eve_mutator_id = client.mk_eve_mutator(
        datas=[eve_d1, eve_d2],
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_attr_id: (0.8, 1.2)})
    client.create_sources(cleanup_check=False)
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_bare_item = api_fit.add_module(type_id=eve_mutated_item_id)
    api_mutated_item = api_fit.add_module(
        type_id=eve_base_item_id,
        mutation=(eve_mutator_id, {eve_attr_id: Muta.roll_to_api(val=0.3)}))
    # Verification - the type is regular on the first source: it is loaded on its own, and
    # conversion into it is removed from data
    api_bare_item.update()
    assert api_bare_item.attrs[eve_attr_id].base == approx(150)
    api_mutated_item.update()
    assert api_mutated_item.type_id == eve_base_item_id
    assert api_mutated_item.mutation.type == consts.ApiMutationType.dormant
    assert api_mutated_item.attrs[eve_attr_id].base == approx(100)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    api_bare_item.update()
    with check_no_field():
        api_bare_item.attrs  # ruff:ignore[useless-expression]
    api_mutated_item.update()
    assert api_mutated_item.type_id == eve_mutated_item_id
    assert api_mutated_item.mutation.type == consts.ApiMutationType.effective
    assert api_mutated_item.attrs[eve_attr_id].base == approx(92)
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    api_bare_item.update()
    assert api_bare_item.attrs[eve_attr_id].base == approx(150)
    api_mutated_item.update()
    assert api_mutated_item.type_id == eve_base_item_id
    assert api_mutated_item.mutation.type == consts.ApiMutationType.dormant
    assert api_mutated_item.attrs[eve_attr_id].base == approx(100)
