from tests import approx


def test_propagation(client, consts):
    # Check that changes to attribute value which is source of modification are propagated to target
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_buff_mult_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_fleet_ships, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 50},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_implant_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_buff_mult_attr_id,
        affectee_attr_id=eve_buff_val_attr_id)
    eve_implant_effect_id = client.mk_eve_effect(mod_info=[eve_implant_mod])
    eve_implant_id = client.mk_eve_item(attrs={eve_buff_mult_attr_id: 2}, eff_ids=[eve_implant_effect_id])
    eve_rig_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 7.5})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit1.id, api_fit2.id])
    api_fit1.set_ship(type_id=eve_ship_id)
    api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fit2.set_ship(type_id=eve_ship_id)
    api_rig = api_fit2.add_rig(type_id=eve_rig_id)
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(11.25)
    api_implant = api_fit1.add_implant(type_id=eve_implant_id)
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(15)
    api_implant.remove()
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(11.25)


def test_self_replace_root(client, consts):
    # Modifiers which target items on ship location shouldn't apply when ship isn't set
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_fleet_ships, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_rig_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 7.5})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship1 = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    api_ship1.remove()
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_fit.set_ship(type_id=eve_ship_id)
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)


def test_fleeted_replace_root(client, consts):
    # Modifiers which target items on ship location shouldn't apply when ship isn't set
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_fleet_ships, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_rig_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 7.5})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit1.id, api_fit2.id])
    api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_ship1 = api_fit2.set_ship(type_id=eve_ship_id)
    api_rig = api_fit2.add_rig(type_id=eve_rig_id)
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    api_ship1.remove()
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_fit2.set_ship(type_id=eve_ship_id)
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
