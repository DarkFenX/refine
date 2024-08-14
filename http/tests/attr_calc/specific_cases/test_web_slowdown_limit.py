from tests import approx, check_no_field


def test_falloff(client, consts):
    # Here we check that limit is applied after range reductions. There is no actual ingame scenario
    # where it can be checked, but CCP Kestrel confirmed it works like this
    eve_limit_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor_floor, def_val=-99)
    eve_affector_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_optimal_attr = client.mk_eve_attr(id_=consts.EveAttr.max_range)
    eve_falloff_attr = client.mk_eve_attr(id_=consts.EveAttr.falloff)
    eve_web_effect = client.mk_eve_effect(
        id_=consts.EveEffect.remote_webifier_falloff,
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr.id,
        falloff_attr_id=eve_falloff_attr.id)
    eve_affector_module = client.mk_eve_item(
        attrs={eve_affector_attr.id: -150, eve_optimal_attr.id: 10000, eve_falloff_attr.id: 5000},
        eff_ids=[eve_web_effect.id],
        defeff_id=eve_web_effect.id)
    eve_affectee_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship.id)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    api_affector_module.change_mod(add_projs=[(api_affectee_ship.id, 5000)])
    # Verification
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr.id].dogma == approx(10)
    assert api_affectee_ship.attrs[eve_affectee_attr.id].extra == approx(10)
    api_affectee_mod = api_affectee_ship.mods[eve_affectee_attr.id].one()
    assert api_affectee_mod.op == consts.ApiModOp.post_percent
    assert api_affectee_mod.initial_val == approx(-150)
    assert api_affectee_mod.range_mult == approx(1.0)
    assert api_affectee_mod.applied_val == approx(-99)
    assert api_affectee_mod.affectors.one().item_id == api_affector_module.id
    assert api_affectee_mod.affectors.one().attr_id == eve_affector_attr.id
    api_affector_module.update()
    assert api_affector_module.attrs[eve_affector_attr.id].dogma == approx(-150)
    assert api_affector_module.attrs[eve_affector_attr.id].extra == approx(-150)
    assert api_affector_module.attrs[eve_limit_attr.id].dogma == approx(-99)
    assert api_affector_module.attrs[eve_limit_attr.id].extra == approx(-99)
    with check_no_field():
        api_affector_module.mods  # pylint: disable=W0104
    # Action
    api_affector_module.change_mod(change_projs=[(api_affectee_ship.id, 15000)])
    # Verification
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr.id].dogma == approx(250)
    assert api_affectee_ship.attrs[eve_affectee_attr.id].extra == approx(250)
    api_affectee_mod = api_affectee_ship.mods[eve_affectee_attr.id].one()
    assert api_affectee_mod.op == consts.ApiModOp.post_percent
    assert api_affectee_mod.initial_val == approx(-150)
    assert api_affectee_mod.range_mult == approx(0.5)
    assert api_affectee_mod.applied_val == approx(-75)
    assert api_affectee_mod.affectors.one().item_id == api_affector_module.id
    assert api_affectee_mod.affectors.one().attr_id == eve_affector_attr.id
    api_affector_module.update()
    assert api_affector_module.attrs[eve_affector_attr.id].dogma == approx(-150)
    assert api_affector_module.attrs[eve_affector_attr.id].extra == approx(-150)
    assert api_affector_module.attrs[eve_limit_attr.id].dogma == approx(-99)
    assert api_affector_module.attrs[eve_limit_attr.id].extra == approx(-99)
    with check_no_field():
        api_affector_module.mods  # pylint: disable=W0104


def test_resist(client, consts):
    # There is a possibility of this scenario (webbing a super with a rolled web from a serp ship in
    # Offikatlin) but it's yet to be confirmed. For now, assume it works similar to range reductions
    eve_limit_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor_floor, def_val=-99)
    eve_affector_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_resist_attr = client.mk_eve_attr()
    eve_resist_def_attr = client.mk_eve_attr(id_=consts.EveAttr.remote_resistance_id)
    eve_web_effect = client.mk_eve_effect(id_=consts.EveEffect.remote_webifier_falloff, cat_id=consts.EveEffCat.target)
    eve_affector_module = client.mk_eve_item(
        attrs={eve_affector_attr.id: -150, eve_resist_def_attr.id: eve_resist_attr.id},
        eff_ids=[eve_web_effect.id],
        defeff_id=eve_web_effect.id)
    eve_affectee_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 1000, eve_resist_attr.id: 0.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship.id)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    api_affector_module.change_mod(add_projs=[api_affectee_ship.id])
    # Verification
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr.id].dogma == approx(250)
    assert api_affectee_ship.attrs[eve_affectee_attr.id].extra == approx(250)
    api_affectee_mod = api_affectee_ship.mods[eve_affectee_attr.id].one()
    assert api_affectee_mod.op == consts.ApiModOp.post_percent
    assert api_affectee_mod.initial_val == approx(-150)
    assert api_affectee_mod.resist_mult == approx(0.5)
    assert api_affectee_mod.applied_val == approx(-75)
    assert api_affectee_mod.affectors.one().item_id == api_affector_module.id
    assert api_affectee_mod.affectors.one().attr_id == eve_affector_attr.id
    api_affector_module.update()
    assert api_affector_module.attrs[eve_affector_attr.id].dogma == approx(-150)
    assert api_affector_module.attrs[eve_affector_attr.id].extra == approx(-150)
    assert api_affector_module.attrs[eve_limit_attr.id].dogma == approx(-99)
    assert api_affector_module.attrs[eve_limit_attr.id].extra == approx(-99)
    with check_no_field():
        api_affector_module.mods  # pylint: disable=W0104


def test_limit_update(client, consts):
    # Here we check that target speed is updated after limit value is updated. There is no actual
    # ingame scenario where it can be checked, but CCP Kestrel confirmed it works like this
    eve_limit_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor_floor, def_val=-99)
    eve_change_attr = client.mk_eve_attr()
    eve_affector_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_change_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_change_attr.id,
        affectee_attr_id=eve_limit_attr.id)
    eve_change_effect = client.mk_eve_effect(mod_info=[eve_change_mod])
    eve_rig = client.mk_eve_item(attrs={eve_change_attr.id: -70}, eff_ids=[eve_change_effect.id])
    eve_web_effect = client.mk_eve_effect(id_=consts.EveEffect.remote_webifier_falloff, cat_id=consts.EveEffCat.target)
    eve_affector_module = client.mk_eve_item(
        attrs={eve_affector_attr.id: -150},
        eff_ids=[eve_web_effect.id],
        defeff_id=eve_web_effect.id)
    eve_affector_ship = client.mk_eve_ship()
    eve_affectee_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship.id)
    api_affector_fit = api_sol.create_fit()
    api_affector_fit.set_ship(type_id=eve_affector_ship.id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    api_affector_module.change_mod(add_projs=[api_affectee_ship.id])
    # Verification
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr.id].dogma == approx(10)
    assert api_affectee_ship.attrs[eve_affectee_attr.id].extra == approx(10)
    api_affectee_mod = api_affectee_ship.mods[eve_affectee_attr.id].one()
    assert api_affectee_mod.op == consts.ApiModOp.post_percent
    assert api_affectee_mod.initial_val == approx(-150)
    assert api_affectee_mod.applied_val == approx(-99)
    assert api_affectee_mod.affectors.one().item_id == api_affector_module.id
    assert api_affectee_mod.affectors.one().attr_id == eve_affector_attr.id
    api_affector_module.update()
    assert api_affector_module.attrs[eve_affector_attr.id].dogma == approx(-150)
    assert api_affector_module.attrs[eve_affector_attr.id].extra == approx(-150)
    assert api_affector_module.attrs[eve_limit_attr.id].dogma == approx(-99)
    assert api_affector_module.attrs[eve_limit_attr.id].dogma == approx(-99)
    with check_no_field():
        api_affector_module.mods  # pylint: disable=W0104
    # Action
    api_affector_fit.add_rig(type_id=eve_rig.id)
    # Verification
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr.id].dogma == approx(300)
    assert api_affectee_ship.attrs[eve_affectee_attr.id].extra == approx(300)
    api_affectee_mod = api_affectee_ship.mods[eve_affectee_attr.id].one()
    assert api_affectee_mod.op == consts.ApiModOp.post_percent
    assert api_affectee_mod.initial_val == approx(-150)
    assert api_affectee_mod.applied_val == approx(-70)
    assert api_affectee_mod.affectors.one().item_id == api_affector_module.id
    assert api_affectee_mod.affectors.one().attr_id == eve_affector_attr.id
    api_affector_module.update()
    assert api_affector_module.attrs[eve_affector_attr.id].dogma == approx(-150)
    assert api_affector_module.attrs[eve_affector_attr.id].extra == approx(-150)
    assert api_affector_module.attrs[eve_limit_attr.id].dogma == approx(-70)
    assert api_affector_module.attrs[eve_limit_attr.id].dogma == approx(-70)
    assert eve_affector_attr.id not in api_affector_module.mods
