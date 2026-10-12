from fw import approx


def test_add_sw_fit_item_remove_sw_item_fit(client, consts):
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    eve_sw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship()
    eve_module_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_sw_effect = api_sol.add_sw_effect(type_id=eve_sw_effect_id)
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id)
    assert api_module.update().attrs[eve_affectee_attr_id].modified == approx(37.5)
    api_sw_effect.remove()
    assert api_module.update().attrs[eve_affectee_attr_id].modified == approx(7.5)
    api_module.remove()
    api_module.update(status_code=404)
    api_fit.remove()


def test_add_fit_sw_item_remove_item_sw_fit(client, consts):
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    eve_sw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship()
    eve_module_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_sw_effect = api_sol.add_sw_effect(type_id=eve_sw_effect_id)
    api_module = api_fit.add_module(type_id=eve_module_id)
    assert api_module.update().attrs[eve_affectee_attr_id].modified == approx(37.5)
    api_module.remove()
    api_module.update(status_code=404)
    api_sw_effect.remove()
    api_fit.remove()


def test_add_fit_item_sw_remove_fit_sw(client, consts):
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    eve_sw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship()
    eve_module_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id)
    assert api_module.update().attrs[eve_affectee_attr_id].modified == approx(7.5)
    api_sw_effect = api_sol.add_sw_effect(type_id=eve_sw_effect_id)
    assert api_module.update().attrs[eve_affectee_attr_id].modified == approx(37.5)
    api_fit.remove()
    api_module.update(status_code=404)
    api_sw_effect.remove()


def test_add_sw_fit_item_state_remove_state_item_fit_sw(client, consts):
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    eve_sw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship()
    eve_module_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_sw_effect = api_sol.add_sw_effect(type_id=eve_sw_effect_id, state=False)
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id)
    assert api_module.update().attrs[eve_affectee_attr_id].modified == approx(7.5)
    api_sw_effect.change_sw_effect(state=True)
    assert api_module.update().attrs[eve_affectee_attr_id].modified == approx(37.5)
    api_sw_effect.change_sw_effect(state=False)
    assert api_module.update().attrs[eve_affectee_attr_id].modified == approx(7.5)
    api_module.remove()
    api_module.update(status_code=404)
    api_fit.remove()
    api_sw_effect.remove()


def test_attr_change_same_mod_count(client, consts):
    # Under the hood, IDs are assigned to modifiers; in this case, the ID is reused by new buff
    # modifier
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_assign_attr_id = client.mk_eve_attr()
    eve_affectee_attr1_id = client.mk_eve_attr()
    eve_affectee_attr2_id = client.mk_eve_attr()
    eve_buff1_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr1_id)])
    eve_buff2_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr2_id)])
    eve_buff_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    # Effect which changes buff type attribute value on its own item
    eve_assign_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_assign_attr_id,
        affectee_attr_id=eve_buff_type_attr_id)
    eve_assign_effect_id = client.mk_eve_effect(mod_info=[eve_assign_mod])
    eve_sw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff1_id, eve_buff_val_attr_id: 5, eve_assign_attr_id: eve_buff2_id},
        eff_ids=[eve_buff_effect_id, eve_assign_effect_id],
        defeff_id=eve_buff_effect_id)
    # Holds buff so it does not get cleaned up
    client.mk_eve_item(attrs={eve_buff_type_attr_id: eve_buff2_id})
    eve_module_id = client.mk_eve_item(attrs={eve_affectee_attr1_id: 20, eve_affectee_attr2_id: 50})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id)
    api_sw_effect = api_sol.add_sw_effect(
        type_id=eve_sw_effect_id,
        effect_modes={eve_assign_effect_id: consts.ApiEffMode.force_stop})
    # Verification
    api_module.update()
    assert api_module.attrs[eve_affectee_attr1_id].modified == approx(100)
    assert api_module.attrs[eve_affectee_attr2_id].modified == approx(50)
    # Action
    api_sw_effect.change_sw_effect(effect_modes={eve_assign_effect_id: consts.ApiEffMode.force_run})
    # Verification
    api_module.update()
    assert api_module.attrs[eve_affectee_attr1_id].modified == approx(20)
    assert api_module.attrs[eve_affectee_attr2_id].modified == approx(250)
    # Action
    api_sw_effect.change_sw_effect(effect_modes={eve_assign_effect_id: consts.ApiEffMode.force_stop})
    # Verification
    api_module.update()
    assert api_module.attrs[eve_affectee_attr1_id].modified == approx(100)
    assert api_module.attrs[eve_affectee_attr2_id].modified == approx(50)


def test_attr_change_different_mod_count(client, consts):
    # Under the hood, IDs are assigned to modifiers; in this case, buffs' modifier count does not
    # match, so IDs are not fully reused
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_assign_attr_id = client.mk_eve_attr()
    eve_affectee_attr1_id = client.mk_eve_attr()
    eve_affectee_attr2_id = client.mk_eve_attr()
    eve_affectee_attr3_id = client.mk_eve_attr()
    eve_buff1_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr1_id)])
    eve_buff2_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_mods=[
            client.mk_eve_buff_mod(attr_id=eve_affectee_attr2_id),
            client.mk_eve_buff_mod(attr_id=eve_affectee_attr3_id)])
    eve_buff_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    # Effect which changes buff type attribute value on its own item
    eve_assign_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_assign_attr_id,
        affectee_attr_id=eve_buff_type_attr_id)
    eve_assign_effect_id = client.mk_eve_effect(mod_info=[eve_assign_mod])
    eve_sw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff1_id, eve_buff_val_attr_id: 5, eve_assign_attr_id: eve_buff2_id},
        eff_ids=[eve_buff_effect_id, eve_assign_effect_id],
        defeff_id=eve_buff_effect_id)
    # Holds buff so it does not get cleaned up
    client.mk_eve_item(attrs={eve_buff_type_attr_id: eve_buff2_id})
    eve_module_id = client.mk_eve_item(attrs={eve_affectee_attr1_id: 20, eve_affectee_attr2_id: 50})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id)
    api_sw_effect = api_sol.add_sw_effect(
        type_id=eve_sw_effect_id,
        effect_modes={eve_assign_effect_id: consts.ApiEffMode.force_stop})
    # Verification
    api_module.update()
    assert api_module.attrs[eve_affectee_attr1_id].modified == approx(100)
    assert api_module.attrs[eve_affectee_attr2_id].modified == approx(50)
    # Action
    api_sw_effect.change_sw_effect(effect_modes={eve_assign_effect_id: consts.ApiEffMode.force_run})
    # Verification
    api_module.update()
    assert api_module.attrs[eve_affectee_attr1_id].modified == approx(20)
    assert api_module.attrs[eve_affectee_attr2_id].modified == approx(250)
    # Action
    api_sw_effect.change_sw_effect(effect_modes={eve_assign_effect_id: consts.ApiEffMode.force_stop})
    # Verification
    api_module.update()
    assert api_module.attrs[eve_affectee_attr1_id].modified == approx(100)
    assert api_module.attrs[eve_affectee_attr2_id].modified == approx(50)
