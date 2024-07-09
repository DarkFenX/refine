from pytest import approx


def test_switch(client, consts):
    eve_affectee_attr = client.mk_eve_attr()
    eve_affector_attr = client.mk_eve_attr(id_=consts.EveAttr.skill_level)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item = client.mk_eve_item(attrs={eve_affectee_attr.id: 100}, eff_ids=[eve_effect.id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_skill(type_id=eve_item.id, level=5)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr.id].dogma == approx(500)
    api_mod = api_item.mods.find_by_affector_item(
        affectee_attr_id=eve_affectee_attr.id,
        affector_item_id=api_item.id).one()
    assert api_mod.op == consts.ApiModOp.post_mul
    assert api_mod.initial_val == approx(5)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(5)
    # Action
    api_item.change_skill(level=3)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr.id].dogma == approx(300)
    api_mod = api_item.mods.find_by_affector_item(
        affectee_attr_id=eve_affectee_attr.id,
        affector_item_id=api_item.id).one()
    assert api_mod.op == consts.ApiModOp.post_mul
    assert api_mod.initial_val == approx(3)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(3)
