from tests import approx, check_no_field


def test_from_stage4(client, consts):
    eve_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item()
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(
        type_id=eve_base_item_id,
        mutation=(eve_mutator_id, {eve_attr_id: {consts.ApiAttrMutation.roll: 0.6}}))
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.mutation.base_type_id == eve_base_item_id
    assert len(api_item.mutation.attrs) == 1
    assert api_item.mutation.attrs[eve_attr_id].roll == approx(0.6)
    assert api_item.mutation.attrs[eve_attr_id].absolute == approx(104)
    assert api_item.attrs[eve_attr_id].base == approx(104)
    # Action
    api_item.change_module(mutation=None)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item_id
    with check_no_field():
        api_item.mutation  # noqa: B018
    # Action
    api_item.change_module(mutation=eve_mutator_id)
    # Verification - after mutating item again, all the old mutations should be gone
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.mutation.base_type_id == eve_base_item_id
    assert len(api_item.mutation.attrs) == 1
    assert api_item.mutation.attrs[eve_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_attr_id].absolute == approx(100)
    assert api_item.attrs[eve_attr_id].base == approx(100)


def test_from_stage3(client, consts):
    eve_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_attr_id: 100})
    eve_mutated_item_id = client.alloc_item_id()
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(
        type_id=eve_base_item_id,
        mutation=(eve_mutator_id, {eve_attr_id: {consts.ApiAttrMutation.roll: 0.6}}))
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item_id
    with check_no_field():
        api_item.mutation  # noqa: B018
    # Action
    api_item.change_module(mutation=None)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item_id
    with check_no_field():
        api_item.mutation  # noqa: B018
    # Action
    api_item.change_module(mutation=eve_mutator_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item_id
    with check_no_field():
        api_item.mutation  # noqa: B018


def test_from_stage2(client, consts):
    eve_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item()
    eve_mutator_id = client.mk_eve_mutator(
        items=[([client.mk_eve_item()], eve_mutated_item_id)],
        attrs={eve_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(
        type_id=eve_base_item_id,
        mutation=(eve_mutator_id, {eve_attr_id: {consts.ApiAttrMutation.roll: 0.6}}))
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item_id
    with check_no_field():
        api_item.mutation  # noqa: B018
    # Action
    api_item.change_module(mutation=None)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item_id
    with check_no_field():
        api_item.mutation  # noqa: B018
    # Action
    api_item.change_module(mutation=eve_mutator_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item_id
    with check_no_field():
        api_item.mutation  # noqa: B018


def test_from_stage1(client, consts):
    eve_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_attr_id: 100})
    eve_mutator_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(
        type_id=eve_base_item_id,
        mutation=(eve_mutator_id, {eve_attr_id: {consts.ApiAttrMutation.roll: 0.6}}))
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item_id
    with check_no_field():
        api_item.mutation  # noqa: B018
    # Action
    api_item.change_module(mutation=None)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item_id
    with check_no_field():
        api_item.mutation  # noqa: B018
    # Action
    api_item.change_module(mutation=eve_mutator_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item_id
    with check_no_field():
        api_item.mutation  # noqa: B018


def test_from_unmutated(client):
    eve_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item()
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(type_id=eve_base_item_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item_id
    with check_no_field():
        api_item.mutation  # noqa: B018
    # Action
    api_item.change_module(mutation=None)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item_id
    with check_no_field():
        api_item.mutation  # noqa: B018
    # Action
    api_item.change_module(mutation=eve_mutator_id)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.mutation.base_type_id == eve_base_item_id
    assert len(api_item.mutation.attrs) == 1
    assert api_item.mutation.attrs[eve_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_attr_id].absolute == approx(100)
    assert api_item.attrs[eve_attr_id].base == approx(100)


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
    eve_base_item_id = client.mk_eve_item(
        attrs={eve_affector1_attr_id: 20, eve_affector2_attr_id: 30},
        eff_ids=[eve_effect1_id],
        defeff_id=eve_effect1_id)
    eve_mutated_item_id = client.mk_eve_item(eff_ids=[eve_effect2_id], defeff_id=eve_effect2_id)
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_item_id], eve_mutated_item_id)])
    eve_affectee_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_base_item_id,
        state=consts.ApiModuleState.active,
        mutation=eve_mutator_id)
    api_affector_drone = api_affector_fit.add_drone(
        type_id=eve_base_item_id,
        state=consts.ApiMinionState.engaging,
        mutation=eve_mutator_id)
    api_affectee1_fit = api_sol.create_fit()
    api_affectee1_ship = api_affectee1_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affectee2_fit = api_sol.create_fit()
    api_affectee2_ship = api_affectee2_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affector_module.change_module(add_projs=[api_affectee1_ship.id])
    api_affector_drone.change_drone(add_projs=[api_affectee2_ship.id])
    # Verification
    assert api_affectee1_ship.update().attrs[eve_affectee_attr_id].dogma == approx(130)
    assert api_affectee2_ship.update().attrs[eve_affectee_attr_id].dogma == approx(130)
    # Action
    api_affector_module.change_module(mutation=None)
    api_affector_drone.change_drone(mutation=None)
    # Verification
    assert api_affectee1_ship.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    assert api_affectee2_ship.update().attrs[eve_affectee_attr_id].dogma == approx(120)


def test_drone(client, consts):
    eve_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item()
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_attr_id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_drone(
        type_id=eve_base_item_id,
        mutation=(eve_mutator_id, {eve_attr_id: {consts.ApiAttrMutation.roll: 0.6}}))
    # Verification
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.mutation.base_type_id == eve_base_item_id
    assert len(api_item.mutation.attrs) == 1
    assert api_item.mutation.attrs[eve_attr_id].roll == approx(0.6)
    assert api_item.mutation.attrs[eve_attr_id].absolute == approx(104)
    assert api_item.attrs[eve_attr_id].base == approx(104)
    # Action
    api_item.change_drone(mutation=None)
    # Verification
    api_item.update()
    assert api_item.type_id == eve_base_item_id
    with check_no_field():
        api_item.mutation  # noqa: B018
    # Action
    api_item.change_drone(mutation=eve_mutator_id)
    # Verification - after mutating item again, all the old mutations should be gone
    api_item.update()
    assert api_item.type_id == eve_mutated_item_id
    assert api_item.mutation.base_type_id == eve_base_item_id
    assert len(api_item.mutation.attrs) == 1
    assert api_item.mutation.attrs[eve_attr_id].roll == approx(0.5)
    assert api_item.mutation.attrs[eve_attr_id].absolute == approx(100)
    assert api_item.attrs[eve_attr_id].base == approx(100)
