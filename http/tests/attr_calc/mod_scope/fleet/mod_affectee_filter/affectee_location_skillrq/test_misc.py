from pytest import approx


def test_self_replace_root(client, consts):
    # Modifiers which target items on ship location shouldn't apply when ship isn't set
    eve_skill = client.mk_eve_item()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id, skill_id=eve_skill.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_warfare_link_armor,
        cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_rig = client.mk_eve_item(attrs={eve_affectee_attr.id: 7.5}, srqs={eve_skill.id: 1})
    eve_ship = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship1 = api_fit.set_ship(type_id=eve_ship.id)
    api_fit.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_rig = api_fit.add_rig(type_id=eve_rig.id)
    assert api_rig.update().attrs[eve_affectee_attr.id].dogma == approx(37.5)
    api_ship1.remove()
    assert api_rig.update().attrs[eve_affectee_attr.id].dogma == approx(7.5)
    api_fit.set_ship(type_id=eve_ship.id)
    assert api_rig.update().attrs[eve_affectee_attr.id].dogma == approx(37.5)


def test_fleeted_replace_root(client, consts):
    # Modifiers which target items on ship location shouldn't apply when ship isn't set
    eve_skill = client.mk_eve_item()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id, skill_id=eve_skill.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_warfare_link_armor,
        cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_rig = client.mk_eve_item(attrs={eve_affectee_attr.id: 7.5}, srqs={eve_skill.id: 1})
    eve_ship = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit1.id, api_fit2.id])
    api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_ship1 = api_fit2.set_ship(type_id=eve_ship.id)
    api_rig = api_fit2.add_rig(type_id=eve_rig.id)
    assert api_rig.update().attrs[eve_affectee_attr.id].dogma == approx(37.5)
    api_ship1.remove()
    assert api_rig.update().attrs[eve_affectee_attr.id].dogma == approx(7.5)
    api_fit2.set_ship(type_id=eve_ship.id)
    assert api_rig.update().attrs[eve_affectee_attr.id].dogma == approx(37.5)
