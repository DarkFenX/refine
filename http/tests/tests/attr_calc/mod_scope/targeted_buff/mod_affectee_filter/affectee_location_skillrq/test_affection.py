from tests import approx


def test_affected_child_of_ship_ship(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id, skill_id=eve_skill_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -55},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_rig_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 200}, srqs={eve_skill_id: 1})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_rig = api_fit2.add_rig(type_id=eve_rig_id)
    api_module.change_module(add_projs=[api_ship.id])
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(90)
    api_module.remove()
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(200)


def test_affected_child_of_ship_struct(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_struct_id = client.mk_eve_struct()
    eve_rig_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 200}, srqs={eve_skill_id: 1})
    eve_item_list_id = client.mk_eve_item_list(inc_type_ids=[eve_struct_id])
    eve_buff_id = client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id, skill_id=eve_skill_id)])
    eve_module_id = client.mk_eve_item()
    client.mk_eve_space_comp(type_id=eve_module_id, sw_buffs=({eve_buff_id: -55}, eve_item_list_id))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_struct = api_fit2.set_ship(type_id=eve_struct_id)
    api_rig = api_fit2.add_rig(type_id=eve_rig_id)
    api_module.change_module(add_projs=[api_struct.id])
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(90)
    api_module.remove()
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(200)


def test_unaffected_child_of_ship_unknown(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id, skill_id=eve_skill_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -55},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_rig_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 200}, srqs={eve_skill_id: 1})
    eve_ship_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_rig = api_fit2.add_rig(type_id=eve_rig_id)
    api_module.change_module(add_projs=[api_ship.id])
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    api_module.remove()
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(200)


def test_unaffected_child_of_offlist_ship_ship(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_ship_id = client.mk_eve_ship()
    eve_rig_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 200}, srqs={eve_skill_id: 1})
    eve_item_list_id = client.mk_eve_item_list()
    eve_buff_id = client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id, skill_id=eve_skill_id)])
    eve_module_id = client.mk_eve_item()
    client.mk_eve_space_comp(type_id=eve_module_id, sw_buffs=({eve_buff_id: -55}, eve_item_list_id))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_rig = api_fit2.add_rig(type_id=eve_rig_id)
    api_module.change_module(add_projs=[api_ship.id])
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    api_module.remove()
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(200)


def test_unaffected_child_of_offlist_ship_struct(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_struct_id = client.mk_eve_struct()
    eve_rig_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 200}, srqs={eve_skill_id: 1})
    eve_item_list_id = client.mk_eve_item_list()
    eve_buff_id = client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id, skill_id=eve_skill_id)])
    eve_module_id = client.mk_eve_item()
    client.mk_eve_space_comp(type_id=eve_module_id, sw_buffs=({eve_buff_id: -55}, eve_item_list_id))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_struct = api_fit2.set_ship(type_id=eve_struct_id)
    api_rig = api_fit2.add_rig(type_id=eve_rig_id)
    api_module.change_module(add_projs=[api_struct.id])
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    api_module.remove()
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(200)


def test_unaffected_child_of_char_via_ship(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id, skill_id=eve_skill_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -55},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_implant_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 200}, srqs={eve_skill_id: 1})
    eve_ship_id = client.mk_eve_ship()
    eve_char_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fit2.set_character(type_id=eve_char_id)
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_implant = api_fit2.add_implant(type_id=eve_implant_id)
    api_module.change_module(add_projs=[api_ship.id])
    assert api_implant.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    api_module.remove()
    assert api_implant.update().attrs[eve_affectee_attr_id].dogma == approx(200)


def test_unaffected_root(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id, skill_id=eve_skill_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -55},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 200}, srqs={eve_skill_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_module.change_module(add_projs=[api_ship.id])
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    api_module.remove()
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(200)


def test_unaffected_other_fit(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id, skill_id=eve_skill_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -55},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_rig_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 200}, srqs={eve_skill_id: 1})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fit3 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_fit3.set_ship(type_id=eve_ship_id)
    api_rig = api_fit3.add_rig(type_id=eve_rig_id)
    api_module.change_module(add_projs=[api_ship.id])
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    api_module.remove()
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(200)


def test_unaffected_targeted_child(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id, skill_id=eve_skill_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -55},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_drone_id = client.mk_eve_drone(attrs={eve_affectee_attr_id: 200}, srqs={eve_skill_id: 1})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fit2.set_ship(type_id=eve_ship_id)
    api_drone = api_fit2.add_drone(type_id=eve_drone_id)
    api_module.change_module(add_projs=[api_drone.id])
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(200)


def test_unaffected_via_child(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id, skill_id=eve_skill_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -55},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_rig_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 200}, srqs={eve_skill_id: 1})
    eve_drone_id = client.mk_eve_drone(attrs={eve_affectee_attr_id: 200}, srqs={eve_skill_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 200}, srqs={eve_skill_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_rig = api_fit2.add_rig(type_id=eve_rig_id)
    api_drone = api_fit2.add_drone(type_id=eve_drone_id)
    api_module.change_module(add_projs=[api_drone.id])
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    api_module.remove()
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(200)


def test_unaffected_other_skillreq(client, consts):
    eve_skill1_id = client.mk_eve_item()
    eve_skill2_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id, skill_id=eve_skill1_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -55},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_rig_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 200}, srqs={eve_skill2_id: 1})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_rig = api_fit2.add_rig(type_id=eve_rig_id)
    api_module.change_module(add_projs=[api_ship.id])
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(200)
