from pytest import approx


def test_stacking(client, consts):
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr(stackable=False)
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
    eve_affector_module1 = client.mk_eve_item(
        attrs={eve_affector_attr.id: -80, eve_optimal_attr.id: 1000, eve_falloff_attr.id: 10000},
        eff_ids=[eve_effect.id],
        defeff_id=eve_effect.id)
    eve_affector_module2 = client.mk_eve_item(
        attrs={eve_affector_attr.id: -30, eve_optimal_attr.id: 12000, eve_falloff_attr.id: 2000},
        eff_ids=[eve_effect.id],
        defeff_id=eve_effect.id)
    eve_affectee_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship.id)
    api_affector_module1 = api_affector_fit.add_mod(type_id=eve_affector_module1.id, state=consts.ApiState.active)
    api_affector_module1.change_mod(add_projs=[(api_affectee_ship.id, 18000)])
    api_affector_module2 = api_affector_fit.add_mod(type_id=eve_affector_module2.id, state=consts.ApiState.active)
    api_affector_module2.change_mod(add_projs=[(api_affectee_ship.id, 13000)])
    # Second module has stronger effect after range factored in, and thus is penalized less. If it
    # was the other way around, the value would've been ~348.2
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(338.79774)
