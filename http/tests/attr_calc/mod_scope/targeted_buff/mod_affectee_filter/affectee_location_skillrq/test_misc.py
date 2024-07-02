from pytest import approx


def test_replace_target(client, consts):
    eve_skill = client.mk_eve_item()
    eve_affector_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id, skill_id=eve_skill.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(attrs={eve_affector_attr.id: -55}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_rig = client.mk_eve_item(attrs={eve_affectee_attr.id: 200}, srqs={eve_skill.id: 1})
    eve_ship = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_ship1 = api_fit2.set_ship(type_id=eve_ship.id)
    api_rig = api_fit2.add_rig(type_id=eve_rig.id)
    api_module.change_mod(add_projs=[api_ship1.id])
    assert api_rig.update().attrs[eve_affectee_attr.id].dogma == approx(90)
    api_ship1.remove()
    assert api_rig.update().attrs[eve_affectee_attr.id].dogma == approx(200)
    api_ship2 = api_fit2.set_ship(type_id=eve_ship.id)
    assert api_rig.update().attrs[eve_affectee_attr.id].dogma == approx(200)
    api_module.change_mod(add_projs=[api_ship2.id])
    assert api_rig.update().attrs[eve_affectee_attr.id].dogma == approx(90)
