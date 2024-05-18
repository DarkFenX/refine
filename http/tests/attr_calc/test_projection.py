from pytest import approx


def test_falloff_state_change(client, consts):
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_optimal_attr = client.mk_eve_attr()
    eve_falloff_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr.id,
        falloff_attr_id=eve_falloff_attr.id,
        mod_info=[eve_mod])
    eve_module = client.mk_eve_item(
        attrs={eve_affector_attr.id: -60, eve_optimal_attr.id: 10000, eve_falloff_attr.id: 5000},
        eff_ids=[eve_effect.id],
        defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item(attrs={eve_affectee_attr.id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.online)
    api_module.change_mod(add_tgts=[(api_ship.id, 15000)])
    assert api_ship.update().attrs[eve_affectee_attr.id].dogma == approx(500)
    api_module.change_mod(state=consts.ApiState.active)
    assert api_ship.update().attrs[eve_affectee_attr.id].dogma == approx(350)
    api_module.change_mod(state=consts.ApiState.online)
    assert api_ship.update().attrs[eve_affectee_attr.id].dogma == approx(500)
