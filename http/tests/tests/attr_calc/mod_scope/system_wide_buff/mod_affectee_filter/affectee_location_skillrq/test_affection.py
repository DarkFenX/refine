from fw import approx


def test_affected_child_of_ship_ship_multiple(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id, skill_id=eve_skill_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    eve_sw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship()
    eve_module_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 7.5}, srqs={eve_skill_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit1.set_ship(type_id=eve_ship_id)
    api_module1 = api_fit1.add_module(type_id=eve_module_id)
    api_sw_effect = api_sol.add_sw_effect(type_id=eve_sw_effect_id)
    api_fit2 = api_sol.create_fit()
    api_fit2.set_ship(type_id=eve_ship_id)
    api_module2 = api_fit2.add_module(type_id=eve_module_id)
    assert api_module1.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    assert api_module2.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    api_sw_effect.remove()
    assert api_module1.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    assert api_module2.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)


def test_affected_child_of_ship_struct_multiple(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_struct_id = client.mk_eve_struct()
    eve_module_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 7.5}, srqs={eve_skill_id: 1})
    eve_item_list_id = client.mk_eve_item_list(inc_type_ids=[eve_struct_id])
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id, skill_id=eve_skill_id)])
    eve_sw_effect_id = client.mk_eve_item()
    client.mk_eve_space_comp(type_id=eve_sw_effect_id, sw_buffs=({eve_buff_id: 5}, eve_item_list_id))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit1.set_ship(type_id=eve_struct_id)
    api_module1 = api_fit1.add_module(type_id=eve_module_id)
    api_sw_effect = api_sol.add_sw_effect(type_id=eve_sw_effect_id)
    api_fit2 = api_sol.create_fit()
    api_fit2.set_ship(type_id=eve_struct_id)
    api_module2 = api_fit2.add_module(type_id=eve_module_id)
    assert api_module1.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    assert api_module2.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    api_sw_effect.remove()
    assert api_module1.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    assert api_module2.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)


def test_unaffected_child_of_ship_unknown(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id, skill_id=eve_skill_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    eve_sw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_item()
    eve_module_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 7.5}, srqs={eve_skill_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id)
    api_sw_effect = api_sol.add_sw_effect(type_id=eve_sw_effect_id)
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_sw_effect.remove()
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)


def test_unaffected_child_of_offlist_ship_ship(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_ship_id = client.mk_eve_ship()
    eve_module_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 7.5}, srqs={eve_skill_id: 1})
    eve_item_list_id = client.mk_eve_item_list()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id, skill_id=eve_skill_id)])
    eve_sw_effect_id = client.mk_eve_item()
    client.mk_eve_space_comp(type_id=eve_sw_effect_id, sw_buffs=({eve_buff_id: 5}, eve_item_list_id))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id)
    api_sw_effect = api_sol.add_sw_effect(type_id=eve_sw_effect_id)
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_sw_effect.remove()
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)


def test_unaffected_child_of_offlist_ship_struct(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id, skill_id=eve_skill_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    eve_sw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_struct_id = client.mk_eve_struct()
    eve_module_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 7.5}, srqs={eve_skill_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct_id)
    api_module = api_fit.add_module(type_id=eve_module_id)
    api_sw_effect = api_sol.add_sw_effect(type_id=eve_sw_effect_id)
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_sw_effect.remove()
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)


def test_unaffected_child_of_char_via_ship(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id, skill_id=eve_skill_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    eve_sw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_char_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship()
    eve_implant_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 7.5}, srqs={eve_skill_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_sw_effect = api_sol.add_sw_effect(type_id=eve_sw_effect_id)
    api_fit.set_character(type_id=eve_char_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    assert api_implant.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_sw_effect.remove()
    assert api_implant.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)


def test_unaffected_child_of_char_via_char(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_char_id = client.mk_eve_item()
    eve_implant_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 7.5}, srqs={eve_skill_id: 1})
    eve_item_list_id = client.mk_eve_item_list(inc_type_ids=[eve_char_id])
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id, skill_id=eve_skill_id)])
    eve_sw_effect_id = client.mk_eve_item()
    client.mk_eve_space_comp(type_id=eve_sw_effect_id, sw_buffs=({eve_buff_id: 5}, eve_item_list_id))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_char_id)
    api_implant = api_fit.add_module(type_id=eve_implant_id)
    api_sw_effect = api_sol.add_sw_effect(type_id=eve_sw_effect_id)
    assert api_implant.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_sw_effect.remove()
    assert api_implant.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)


def test_unaffected_root(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id, skill_id=eve_skill_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    eve_sw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 7.5}, srqs={eve_skill_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_sw_effect = api_sol.add_sw_effect(type_id=eve_sw_effect_id)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_sw_effect.remove()
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)


def test_unaffected_other_skillrq(client, consts):
    eve_skill1_id = client.mk_eve_item()
    eve_skill2_id = client.mk_eve_item()
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id, skill_id=eve_skill1_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    eve_sw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship()
    eve_module_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 7.5}, srqs={eve_skill2_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_sw_effect = api_sol.add_sw_effect(type_id=eve_sw_effect_id)
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id)
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_sw_effect.remove()
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
