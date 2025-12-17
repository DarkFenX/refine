from fw import approx


def test_unresisted(client, consts):
    # System effects never define resists, so the engine does not support those
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_resist_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.system,
        resist_attr_id=eve_resist_attr_id,
        mod_info=[eve_mod])
    eve_affector_sw_effect_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100, eve_resist_attr_id: 0.4})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_affectee_item = api_fit.set_ship(type_id=eve_affectee_ship_id)
    api_sol.add_sw_effect(type_id=eve_affector_sw_effect_id)
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
