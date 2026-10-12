from fw import approx


def test_same_mod_count(client, consts):
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
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr1_id)])
    eve_buff2_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr2_id)])
    eve_buff_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    # Effect which changes buff type attribute value on its own item
    eve_assign_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_assign_attr_id,
        affectee_attr_id=eve_buff_type_attr_id)
    eve_assign_effect_id = client.mk_eve_effect(mod_info=[eve_assign_mod])
    eve_fw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff1_id, eve_buff_val_attr_id: 5, eve_assign_attr_id: eve_buff2_id},
        eff_ids=[eve_buff_effect_id, eve_assign_effect_id],
        defeff_id=eve_buff_effect_id)
    # Holds buff so it does not get cleaned up
    client.mk_eve_item(attrs={eve_buff_type_attr_id: eve_buff2_id})
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr1_id: 20, eve_affectee_attr2_id: 50})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fw_effect = api_fit.add_fw_effect(
        type_id=eve_fw_effect_id,
        effect_modes={eve_assign_effect_id: consts.ApiEffMode.force_stop})
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_affectee_attr1_id].modified == approx(100)
    assert api_ship.attrs[eve_affectee_attr2_id].modified == approx(50)
    # Action
    api_fw_effect.change_fw_effect(effect_modes={eve_assign_effect_id: consts.ApiEffMode.force_run})
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_affectee_attr1_id].modified == approx(20)
    assert api_ship.attrs[eve_affectee_attr2_id].modified == approx(250)
    # Action
    api_fw_effect.change_fw_effect(effect_modes={eve_assign_effect_id: consts.ApiEffMode.force_stop})
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_affectee_attr1_id].modified == approx(100)
    assert api_ship.attrs[eve_affectee_attr2_id].modified == approx(50)


def test_different_mod_count(client, consts):
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
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr1_id)])
    eve_buff2_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[
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
    eve_fw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff1_id, eve_buff_val_attr_id: 5, eve_assign_attr_id: eve_buff2_id},
        eff_ids=[eve_buff_effect_id, eve_assign_effect_id],
        defeff_id=eve_buff_effect_id)
    # Holds buff so it does not get cleaned up
    client.mk_eve_item(attrs={eve_buff_type_attr_id: eve_buff2_id})
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr1_id: 20, eve_affectee_attr2_id: 50})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fw_effect = api_fit.add_fw_effect(
        type_id=eve_fw_effect_id,
        effect_modes={eve_assign_effect_id: consts.ApiEffMode.force_stop})
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_affectee_attr1_id].modified == approx(100)
    assert api_ship.attrs[eve_affectee_attr2_id].modified == approx(50)
    # Action
    api_fw_effect.change_fw_effect(effect_modes={eve_assign_effect_id: consts.ApiEffMode.force_run})
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_affectee_attr1_id].modified == approx(20)
    assert api_ship.attrs[eve_affectee_attr2_id].modified == approx(250)
    # Action
    api_fw_effect.change_fw_effect(effect_modes={eve_assign_effect_id: consts.ApiEffMode.force_stop})
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_affectee_attr1_id].modified == approx(100)
    assert api_ship.attrs[eve_affectee_attr2_id].modified == approx(50)
