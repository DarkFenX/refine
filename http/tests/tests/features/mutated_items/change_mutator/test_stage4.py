from fw import Muta, approx, check_no_field


def test_from_stage1(client):
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
        mutation=(eve_mutator1_id, {eve_attr_id: Muta.roll_to_api(val=0.3)}))
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


def test_from_stage2(client):
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
        mutation=(eve_mutator1_id, {eve_attr_id: Muta.roll_to_api(val=0.3)}))
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


def test_from_stage3(client):
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
        mutation=(eve_mutator1_id, {eve_attr_id: Muta.roll_to_api(val=0.3)}))
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


def test_from_stage4_range_value(client):
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
        mutation=(eve_mutator1_id, {eve_attr_id: Muta.roll_to_api(val=0.3)}))
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


def test_from_stage4_attr_ranges(client):
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
        eve_attr1_id: Muta.roll_to_api(val=0.3),
        eve_attr2_id: Muta.roll_to_api(val=0.3)}))
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


def test_from_stage4_mutated_item(client):
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
        eve_attr1_id: Muta.roll_to_api(val=0.3),
        eve_attr2_id: Muta.roll_to_api(val=0.3)}))
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
    eve_base_item_id = client.mk_eve_item(attrs={eve_affector1_attr_id: 20, eve_affector2_attr_id: 30})
    eve_mutated_item1_id = client.mk_eve_item(eff_ids=[eve_effect1_id], defeff_id=eve_effect1_id)
    eve_mutated_item2_id = client.mk_eve_item(eff_ids=[eve_effect2_id], defeff_id=eve_effect2_id)
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
    api_affector_module.change_module(add_projs=[api_affectee1_ship.id])
    api_affector_drone.change_drone(add_projs=[api_affectee2_ship.id])
    # Verification
    assert api_affectee1_ship.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    assert api_affectee2_ship.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_affector_module.change_module(mutation=eve_mutator2_id)
    api_affector_drone.change_drone(mutation=eve_mutator2_id)
    # Verification
    assert api_affectee1_ship.update().attrs[eve_affectee_attr_id].dogma == approx(130)
    assert api_affectee2_ship.update().attrs[eve_affectee_attr_id].dogma == approx(130)
