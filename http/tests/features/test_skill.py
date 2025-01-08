from tests import approx, check_no_field


def test_level_switch_loaded(client, consts):
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.skill_level)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_skill(type_id=eve_item_id, level=5)
    # Verification
    api_item.update()
    assert api_item.level == 5
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(500)
    api_mod = api_item.mods.find_by_affector_item(
        affectee_attr_id=eve_affectee_attr_id,
        affector_item_id=api_item.id).one()
    assert api_mod.op == consts.ApiModOp.post_mul
    assert api_mod.initial_val == approx(5)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(5)
    # Action
    api_item.change_skill(level=3)
    # Verification
    api_item.update()
    assert api_item.level == 3
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(300)
    api_mod = api_item.mods.find_by_affector_item(
        affectee_attr_id=eve_affectee_attr_id,
        affector_item_id=api_item.id).one()
    assert api_mod.op == consts.ApiModOp.post_mul
    assert api_mod.initial_val == approx(3)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(3)


def test_level_switch_unloaded(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_affectee_attr_id = client.mk_eve_attr(datas=[eve_d2])
    eve_affector_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2], id_=consts.EveAttr.skill_level)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(datas=[eve_d2], mod_info=[eve_mod])
    eve_item_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_item(datas=[eve_d2], id_=eve_item_id, attrs={eve_affectee_attr_id: 100}, eff_ids=[eve_effect_id])
    # Create another item just to prevent level attribute from getting cleaned up
    eve_another_item_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_item(datas=[eve_d1], id_=eve_another_item_id, attrs={eve_affector_attr_id: 4})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_skill(type_id=eve_item_id, level=5)
    # Verification
    api_item.update()
    assert api_item.level == 5
    with check_no_field():
        api_item.attrs  # pylint: disable=W0104
    with check_no_field():
        api_item.mods  # pylint: disable=W0104
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    api_item.update()
    assert api_item.level == 5
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(500)
    api_mod = api_item.mods.find_by_affector_item(
        affectee_attr_id=eve_affectee_attr_id,
        affector_item_id=api_item.id).one()
    assert api_mod.op == consts.ApiModOp.post_mul
    assert api_mod.initial_val == approx(5)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(5)
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    api_item.update()
    assert api_item.level == 5
    with check_no_field():
        api_item.attrs  # pylint: disable=W0104
    with check_no_field():
        api_item.mods  # pylint: disable=W0104
    # Action
    api_item.change_skill(level=3)
    # Verification
    api_item.update()
    assert api_item.level == 3
    with check_no_field():
        api_item.attrs  # pylint: disable=W0104
    with check_no_field():
        api_item.mods  # pylint: disable=W0104
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    api_item.update()
    assert api_item.level == 3
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(300)
    api_mod = api_item.mods.find_by_affector_item(
        affectee_attr_id=eve_affectee_attr_id,
        affector_item_id=api_item.id).one()
    assert api_mod.op == consts.ApiModOp.post_mul
    assert api_mod.initial_val == approx(3)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(3)


def test_duplicate_loaded(client, consts):
    eve_level_attr_id = client.mk_eve_attr(id_=consts.EveAttr.skill_level)
    eve_item_id = client.mk_eve_item(attrs={eve_level_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_skill(type_id=eve_item_id, level=3)
    # Verification
    api_item.update()
    assert api_item.level == 3
    assert api_item.attrs[eve_level_attr_id].dogma == 3
    api_fit.update()
    assert len(api_fit.skills) == 1
    assert api_fit.skills[0].id == api_item.id
    # Action
    api_fit.add_skill(type_id=eve_item_id, level=4, status_code=409, json_predicate={
        'code': 'EXC-015.1',
        'message': f'skill {eve_item_id} already exists on fit {api_fit.id}, item {api_item.id} has the same type ID'})
    # Verification - nothing should've changed
    api_item.update()
    assert api_item.level == 3
    assert api_item.attrs[eve_level_attr_id].dogma == 3
    api_fit.update()
    assert len(api_fit.skills) == 1
    assert api_fit.skills[0].id == api_item.id


def test_duplicate_unloaded(client, consts):
    eve_level_attr_id = client.mk_eve_attr(id_=consts.EveAttr.skill_level)
    # Create another item just to make sure level attribute doesn't get cleaned up
    client.mk_eve_item(attrs={eve_level_attr_id: 0})
    eve_item_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_skill(type_id=eve_item_id, level=3)
    # Verification
    api_item.update()
    assert api_item.level == 3
    with check_no_field():
        api_item.attrs  # pylint: disable=W0104
    api_fit.update()
    assert len(api_fit.skills) == 1
    assert api_fit.skills[0].id == api_item.id
    # Action
    api_fit.add_skill(type_id=eve_item_id, level=4, status_code=409, json_predicate={
        'code': 'EXC-015.1',
        'message': f'skill {eve_item_id} already exists on fit {api_fit.id}, item {api_item.id} has the same type ID'})
    # Verification - nothing should've changed
    api_item.update()
    assert api_item.level == 3
    with check_no_field():
        api_item.attrs  # pylint: disable=W0104
    api_fit.update()
    assert len(api_fit.skills) == 1
    assert api_fit.skills[0].id == api_item.id


def test_level_modification(client, consts):
    # Check that modifications of skill attribute are completely ignored
    eve_affectee_attr_id = client.mk_eve_attr(id_=consts.EveAttr.skill_level)
    eve_affector_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 4, eve_affectee_attr_id: 0}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_skill(type_id=eve_item_id, level=5)
    # Verification
    api_item.update()
    assert api_item.level == 5
    assert api_item.attrs[eve_affectee_attr_id].base == approx(0)
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(5)
    assert api_item.attrs[eve_affectee_attr_id].extra == approx(5)
    with check_no_field():
        api_item.mods  # pylint: disable=W0104
    # Action
    api_item.change_skill(level=3)
    # Verification
    api_item.update()
    assert api_item.level == 3
    assert api_item.attrs[eve_affectee_attr_id].base == approx(0)
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(3)
    assert api_item.attrs[eve_affectee_attr_id].extra == approx(3)
    with check_no_field():
        api_item.mods  # pylint: disable=W0104
