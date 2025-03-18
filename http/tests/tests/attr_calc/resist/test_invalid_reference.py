from tests import approx


def test_on_effect(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_resist_attr_id = client.alloc_attr_id()
    eve_module_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_module_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        resist_attr_id=eve_resist_attr_id,
        mod_info=[eve_module_mod])
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -60},
        eff_ids=[eve_module_effect_id],
        defeff_id=eve_module_effect_id)
    eve_affectee_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 500, eve_resist_attr_id: 0.4})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_affector_module.change_module(add_projs=[api_affectee_ship.id])
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr_id].dogma == approx(200)
    api_mod = api_affectee_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_val == approx(-60)
    assert api_mod.resist_mult is None
    assert api_mod.applied_val == approx(-60)
    assert api_mod.affectors.one().item_id == api_affector_module.id
    assert api_mod.affectors.one().attr_id == eve_affector_attr_id


def test_on_affector_item(client, consts):
    # We have to do 2 different items here to avoid on-item reference transfer to on-effect
    # reference during adapted data generation
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr(stackable=True)
    eve_resist_attr1_id = client.mk_eve_attr()
    eve_resist_attr2_id = client.alloc_attr_id()
    eve_resist_def_attr_id = client.mk_eve_attr(id_=consts.EveAttr.remote_resistance_id)
    eve_module_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_module_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        mod_info=[eve_module_mod])
    eve_affector_module1_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -60, eve_resist_def_attr_id: eve_resist_attr1_id},
        eff_ids=[eve_module_effect_id],
        defeff_id=eve_module_effect_id)
    eve_affector_module2_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -55, eve_resist_def_attr_id: eve_resist_attr2_id},
        eff_ids=[eve_module_effect_id],
        defeff_id=eve_module_effect_id)
    eve_affectee_ship_id = client.mk_eve_ship(
        attrs={eve_affectee_attr_id: 500, eve_resist_attr1_id: 0.4, eve_resist_attr2_id: 0.3})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affector_module1 = api_affector_fit.add_module(
        type_id=eve_affector_module1_id,
        state=consts.ApiModuleState.active)
    api_affector_module1.change_module(add_projs=[api_affectee_ship.id])
    api_affector_module2 = api_affector_fit.add_module(
        type_id=eve_affector_module2_id,
        state=consts.ApiModuleState.active)
    api_affector_module2.change_module(add_projs=[api_affectee_ship.id])
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr_id].dogma == approx(171)
    api_mods = api_affectee_ship.mods[eve_affectee_attr_id]
    assert len(api_mods) == 2
    api_module1_mod = api_mods.find_by_affector_item(affector_item_id=api_affector_module1.id).one()
    assert api_module1_mod.op == consts.ApiModOp.post_percent
    assert api_module1_mod.initial_val == approx(-60)
    assert api_module1_mod.resist_mult == approx(0.4)
    assert api_module1_mod.applied_val == approx(-24)
    assert api_module1_mod.affectors.one().attr_id == eve_affector_attr_id
    api_module2_mod = api_mods.find_by_affector_item(affector_item_id=api_affector_module2.id).one()
    assert api_module2_mod.op == consts.ApiModOp.post_percent
    assert api_module2_mod.initial_val == approx(-55)
    assert api_module2_mod.resist_mult is None
    assert api_module2_mod.applied_val == approx(-55)
    assert api_module2_mod.affectors.one().attr_id == eve_affector_attr_id
