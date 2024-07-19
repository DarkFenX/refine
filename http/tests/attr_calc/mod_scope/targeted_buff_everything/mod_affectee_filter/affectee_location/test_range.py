from tests import approx


def test_range(client, consts):
    eve_affector_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr = client.mk_eve_attr()
    eve_optimal_attr = client.mk_eve_attr()
    eve_falloff_attr = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.doomsday_aoe_web,
        cat_id=consts.EveEffCat.active,
        range_attr_id=eve_optimal_attr.id,
        falloff_attr_id=eve_falloff_attr.id)
    eve_affector_module = client.mk_eve_item(
        attrs={eve_affector_attr.id: -55, eve_optimal_attr.id: 10000, eve_falloff_attr.id: 5000},
        eff_ids=[eve_effect.id],
        defeff_id=eve_effect.id)
    eve_affectee_module = client.mk_eve_item(attrs={eve_affectee_attr.id: 200})
    eve_affectee_ship = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship.id)
    api_affectee_module = api_affectee_fit.add_mod(type_id=eve_affectee_module.id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    assert api_affectee_module.update().attrs[eve_affectee_attr.id].dogma == approx(200)
    api_affector_module.change_mod(add_projs=[(api_affectee_ship.id, 9000)])
    assert api_affectee_module.update().attrs[eve_affectee_attr.id].dogma == approx(90)
    api_affector_module.change_mod(change_projs=[(api_affectee_ship.id, 11000)])
    # Falloff attribute is ignored for buffs
    assert api_affectee_module.update().attrs[eve_affectee_attr.id].dogma == approx(200)
    api_affector_module.change_mod(rm_projs=[api_affectee_ship.id])
    assert api_affectee_module.update().attrs[eve_affectee_attr.id].dogma == approx(200)
