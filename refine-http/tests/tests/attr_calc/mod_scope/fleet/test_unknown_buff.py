from fw import approx


def test_static(client, consts):
    # Check that there is no strange side effects when buff isn't defined
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_fleet_ships, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: 7, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    assert api_ship.update().attrs[eve_affectee_attr_id].modified == approx(7.5)
    api_module.remove()
    assert api_ship.update().attrs[eve_affectee_attr_id].modified == approx(7.5)


def test_switch(client, consts):
    # Check that there is no strange side effects when buff isn't defined
    eve_buff_val_mult_attr_id = client.mk_eve_attr()
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff1_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_buff2_id = client.alloc_buff_id()
    eve_module_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_fleet_ships, cat_id=consts.EveEffCat.active)
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
        affector_attr_id=eve_buff_val_mult_attr_id,
        affectee_attr_id=eve_buff_val_attr_id)
    eve_charge_effect_id = client.mk_eve_effect(mod_info=[eve_charge_mod1, eve_charge_mod2])
    eve_charge1_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff1_id, eve_buff_val_mult_attr_id: 4},
        eff_ids=[eve_charge_effect_id])
    eve_charge2_id = client.mk_eve_item(
        # Buff ID which we didn't create
        attrs={eve_buff_type_attr_id: eve_buff2_id, eve_buff_val_mult_attr_id: 8},
        eff_ids=[eve_charge_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 20})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].modified == approx(20)
    # Action
    api_module.change_module(charge_type_id=eve_charge1_id)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].modified == approx(100)
    # Action
    api_module.change_module(charge_type_id=eve_charge2_id)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].modified == approx(20)
    # Action
    api_module.change_module(charge_type_id=None)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].modified == approx(20)
