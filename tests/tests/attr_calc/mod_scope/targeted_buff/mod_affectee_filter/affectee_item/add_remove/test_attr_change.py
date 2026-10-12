from fw import approx


def test_same_mod_count(client, consts):
    # Under the hood, IDs are assigned to modifiers; in this case, the ID is reused by new buff
    # modifier
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_mult_attr_id = client.mk_eve_attr()
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
    eve_module_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_ships, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_buff_val_attr_id: 1.25},
        eff_ids=[eve_module_effect_id],
        defeff_id=eve_module_effect_id)
    eve_charge_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_buff_type_attr_id,
        affectee_attr_id=eve_buff_type_attr_id)
    eve_charge_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_mult_attr_id,
        affectee_attr_id=eve_buff_val_attr_id)
    eve_charge_effect_id = client.mk_eve_effect(mod_info=[eve_charge_mod1, eve_charge_mod2])
    eve_charge1_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff1_id, eve_mult_attr_id: 4},
        eff_ids=[eve_charge_effect_id])
    eve_charge2_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff2_id, eve_mult_attr_id: 8},
        eff_ids=[eve_charge_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr1_id: 20, eve_affectee_attr2_id: 50})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship_id)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_affectee_ship.id])
    # Verification
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr1_id].modified == approx(20)
    assert api_affectee_ship.attrs[eve_affectee_attr2_id].modified == approx(50)
    # Action
    api_affector_module.change_module(charge_type_id=eve_charge1_id)
    # Verification
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr1_id].modified == approx(100)
    assert api_affectee_ship.attrs[eve_affectee_attr2_id].modified == approx(50)
    # Action
    api_affector_module.change_module(charge_type_id=eve_charge2_id)
    # Verification
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr1_id].modified == approx(20)
    assert api_affectee_ship.attrs[eve_affectee_attr2_id].modified == approx(500)
    # Action
    api_affector_module.change_module(charge_type_id=None)
    # Verification
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr1_id].modified == approx(20)
    assert api_affectee_ship.attrs[eve_affectee_attr2_id].modified == approx(50)


def test_different_mod_count(client, consts):
    # Under the hood, IDs are assigned to modifiers; in this case, buffs' modifier count does not
    # match, so IDs are not fully reused
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_mult_attr_id = client.mk_eve_attr()
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
    eve_module_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_ships, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_buff_val_attr_id: 1.25},
        eff_ids=[eve_module_effect_id],
        defeff_id=eve_module_effect_id)
    eve_charge_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_buff_type_attr_id,
        affectee_attr_id=eve_buff_type_attr_id)
    eve_charge_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_mult_attr_id,
        affectee_attr_id=eve_buff_val_attr_id)
    eve_charge_effect_id = client.mk_eve_effect(mod_info=[eve_charge_mod1, eve_charge_mod2])
    eve_charge1_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff1_id, eve_mult_attr_id: 4},
        eff_ids=[eve_charge_effect_id])
    eve_charge2_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff2_id, eve_mult_attr_id: 8},
        eff_ids=[eve_charge_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr1_id: 20, eve_affectee_attr2_id: 50})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship_id)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge1_id,
        proj_item_ids=[api_affectee_ship.id])
    # Verification
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr1_id].modified == approx(100)
    assert api_affectee_ship.attrs[eve_affectee_attr2_id].modified == approx(50)
    # Action
    api_affector_module.change_module(charge_type_id=eve_charge2_id)
    # Verification
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr1_id].modified == approx(20)
    assert api_affectee_ship.attrs[eve_affectee_attr2_id].modified == approx(500)
    # Action
    api_affector_module.change_module(charge_type_id=eve_charge1_id)
    # Verification
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr1_id].modified == approx(100)
    assert api_affectee_ship.attrs[eve_affectee_attr2_id].modified == approx(50)
