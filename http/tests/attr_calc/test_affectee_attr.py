from tests import approx


def test_multiple(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr1_id = client.mk_eve_attr()
    eve_affectee_attr2_id = client.mk_eve_attr()
    eve_affectee_attr3_id = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr1_id)
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr2_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod1, eve_mod2])
    eve_item_id = client.mk_eve_item(
        attrs={
            eve_affector_attr_id: 20, eve_affectee_attr1_id: 50,
            eve_affectee_attr2_id: 80, eve_affectee_attr3_id: 100},
        eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_rig(type_id=eve_item_id)
    api_item.update()
    # First attribute should be modified by modifier 1
    assert api_item.attrs[eve_affectee_attr1_id].dogma == approx(60)
    # Second should be modified by modifier 2
    assert api_item.attrs[eve_affectee_attr2_id].dogma == approx(96)
    # Third should stay unmodified
    assert api_item.attrs[eve_affectee_attr3_id].dogma == approx(100)
