from fw import approx


def test_add_afor_afee_proj_remove_state_proj_fit(client, consts):
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -55},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_affectee_module_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 200})
    eve_affectee_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affectee_module = api_affectee_fit.add_module(type_id=eve_affectee_module_id)
    assert api_affectee_module.update().attrs[eve_affectee_attr_id].modified == approx(200)
    api_affector_module.change_module(add_proj_item_ids=[api_affectee_ship.id])
    assert api_affectee_module.update().attrs[eve_affectee_attr_id].modified == approx(90)
    api_affector_module.change_module(state=consts.ApiModuleState.online)
    assert api_affectee_module.update().attrs[eve_affectee_attr_id].modified == approx(200)
    api_affector_module.change_module(rm_proj_item_ids=[api_affectee_ship.id])
    assert api_affectee_module.update().attrs[eve_affectee_attr_id].modified == approx(200)
    api_affectee_fit.remove()
    api_affectee_module.update(status_code=404)
    api_affector_fit.remove()


def test_add_afee_afor_proj_state_remove_afor_afee(client, consts):
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -55},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_affectee_module_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 200})
    eve_affectee_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affectee_module = api_affectee_fit.add_module(type_id=eve_affectee_module_id)
    assert api_affectee_module.update().attrs[eve_affectee_attr_id].modified == approx(200)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.online)
    assert api_affectee_module.update().attrs[eve_affectee_attr_id].modified == approx(200)
    api_affector_module.change_module(add_proj_item_ids=[api_affectee_ship.id])
    assert api_affectee_module.update().attrs[eve_affectee_attr_id].modified == approx(200)
    api_affector_module.change_module(state=consts.ApiModuleState.active)
    assert api_affectee_module.update().attrs[eve_affectee_attr_id].modified == approx(90)
    api_affector_module.remove()
    assert api_affectee_module.update().attrs[eve_affectee_attr_id].modified == approx(200)
    api_affectee_module.remove()
    api_affectee_module.update(status_code=404)
    api_affectee_fit.remove()
    api_affector_fit.remove()


def test_add_afee_afor_proj_remove_afee(client, consts):
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -55},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_affectee_module_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 200})
    eve_affectee_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affectee_module = api_affectee_fit.add_module(type_id=eve_affectee_module_id)
    assert api_affectee_module.update().attrs[eve_affectee_attr_id].modified == approx(200)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    assert api_affectee_module.update().attrs[eve_affectee_attr_id].modified == approx(200)
    api_affector_module.change_module(add_proj_item_ids=[api_affectee_ship.id])
    assert api_affectee_module.update().attrs[eve_affectee_attr_id].modified == approx(90)
    api_affectee_module.remove()
    api_affectee_module.update(status_code=404)
    api_affectee_fit.remove()
    api_affector_fit.remove()


def test_attr_change_same_mod_count(client, consts):
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
        loc_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr1_id)])
    eve_buff2_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr2_id)])
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
    eve_affectee_module_id = client.mk_eve_item(attrs={eve_affectee_attr1_id: 20, eve_affectee_attr2_id: 50})
    eve_affectee_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affectee_module = api_affectee_fit.add_module(type_id=eve_affectee_module_id)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_affectee_ship.id])
    # Verification
    api_affectee_module.update()
    assert api_affectee_module.attrs[eve_affectee_attr1_id].modified == approx(20)
    assert api_affectee_module.attrs[eve_affectee_attr2_id].modified == approx(50)
    # Action
    api_affector_module.change_module(charge_type_id=eve_charge1_id)
    # Verification
    api_affectee_module.update()
    assert api_affectee_module.attrs[eve_affectee_attr1_id].modified == approx(100)
    assert api_affectee_module.attrs[eve_affectee_attr2_id].modified == approx(50)
    # Action
    api_affector_module.change_module(charge_type_id=eve_charge2_id)
    # Verification
    api_affectee_module.update()
    assert api_affectee_module.attrs[eve_affectee_attr1_id].modified == approx(20)
    assert api_affectee_module.attrs[eve_affectee_attr2_id].modified == approx(500)
    # Action
    api_affector_module.change_module(charge_type_id=None)
    # Verification
    api_affectee_module.update()
    assert api_affectee_module.attrs[eve_affectee_attr1_id].modified == approx(20)
    assert api_affectee_module.attrs[eve_affectee_attr2_id].modified == approx(50)


def test_attr_change_different_mod_count(client, consts):
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
        loc_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr1_id)])
    eve_buff2_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_mods=[
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
    eve_affectee_module_id = client.mk_eve_item(attrs={eve_affectee_attr1_id: 20, eve_affectee_attr2_id: 50})
    eve_affectee_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affectee_module = api_affectee_fit.add_module(type_id=eve_affectee_module_id)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge1_id,
        proj_item_ids=[api_affectee_ship.id])
    # Verification
    api_affectee_module.update()
    assert api_affectee_module.attrs[eve_affectee_attr1_id].modified == approx(100)
    assert api_affectee_module.attrs[eve_affectee_attr2_id].modified == approx(50)
    # Action
    api_affector_module.change_module(charge_type_id=eve_charge2_id)
    # Verification
    api_affectee_module.update()
    assert api_affectee_module.attrs[eve_affectee_attr1_id].modified == approx(20)
    assert api_affectee_module.attrs[eve_affectee_attr2_id].modified == approx(500)
    # Action
    api_affector_module.change_module(charge_type_id=eve_charge1_id)
    # Verification
    api_affectee_module.update()
    assert api_affectee_module.attrs[eve_affectee_attr1_id].modified == approx(100)
    assert api_affectee_module.attrs[eve_affectee_attr2_id].modified == approx(50)
