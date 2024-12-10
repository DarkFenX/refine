from tests import approx


def test_switch(client, consts):
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
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(300)
    api_mod = api_item.mods.find_by_affector_item(
        affectee_attr_id=eve_affectee_attr_id,
        affector_item_id=api_item.id).one()
    assert api_mod.op == consts.ApiModOp.post_mul
    assert api_mod.initial_val == approx(3)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(3)
