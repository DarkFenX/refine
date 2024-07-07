from pytest import approx


def test_on_effect(client, consts):
    # Check that modification resistance works when resistance attribute ID is defined on effect
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_resist_attr = client.mk_eve_attr()
    eve_module_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_module_effect = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        resist_attr_id=eve_resist_attr.id,
        mod_info=[eve_module_mod])
    eve_affector_module = client.mk_eve_item(
        attrs={eve_affector_attr.id: -60},
        eff_ids=[eve_module_effect.id],
        defeff_id=eve_module_effect.id)
    eve_affectee_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 500, eve_resist_attr.id: 0.4})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship.id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    api_affector_module.change_mod(add_projs=[api_affectee_ship.id])
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(380)


def test_on_affector_item(client, consts):
    # Check that modification resistance works when resistance attribute ID is defined on affector.
    # We have to do 2 different items here to avoid on-item reference transfer to on-effect
    # reference during adapted data generation
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr(stackable=True)
    eve_resist_attr1 = client.mk_eve_attr()
    eve_resist_attr2 = client.mk_eve_attr()
    eve_resist_def_attr = client.mk_eve_attr(id_=consts.EveAttr.remote_resistance_id)
    eve_module_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_module_effect = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        mod_info=[eve_module_mod])
    eve_affector_module1 = client.mk_eve_item(
        attrs={eve_affector_attr.id: -60, eve_resist_def_attr.id: eve_resist_attr1.id},
        eff_ids=[eve_module_effect.id],
        defeff_id=eve_module_effect.id)
    eve_affector_module2 = client.mk_eve_item(
        attrs={eve_affector_attr.id: -55, eve_resist_def_attr.id: eve_resist_attr2.id},
        eff_ids=[eve_module_effect.id],
        defeff_id=eve_module_effect.id)
    eve_affectee_ship = client.mk_eve_ship(
        attrs={eve_affectee_attr.id: 500, eve_resist_attr1.id: 0.4, eve_resist_attr2.id: 0.3})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship.id)
    api_affector_module1 = api_affector_fit.add_mod(type_id=eve_affector_module1.id, state=consts.ApiState.active)
    api_affector_module1.change_mod(add_projs=[api_affectee_ship.id])
    api_affector_module2 = api_affector_fit.add_mod(type_id=eve_affector_module2.id, state=consts.ApiState.active)
    api_affector_module2.change_mod(add_projs=[api_affectee_ship.id])
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(317.3)
