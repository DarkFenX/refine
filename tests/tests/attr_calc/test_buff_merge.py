"""
This module contains a set of unrealistic (as in, never seen in EVE) & complex scenarios, which
check how attribute calculator processes situations when attributes used in buff-merge process are
updated in various scenarios.
"""

from fw import approx


def test_attr_change_start(client, consts):
    # The effect changes buff type attribute on its own item, so its buff modifiers have to be
    # generated using the changed value
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
    eve_assign_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_assign_attr_id,
        affectee_attr_id=eve_buff_type_attr_id)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.buff_fleet_ships,
        cat_id=consts.EveEffCat.active,
        mod_info=[eve_assign_mod])
    eve_module_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff1_id, eve_buff_val_attr_id: 5, eve_assign_attr_id: eve_buff2_id},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    # Holds buff so it does not get cleaned up
    client.mk_eve_item(attrs={eve_buff_type_attr_id: eve_buff2_id})
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr1_id: 7.5, eve_affectee_attr2_id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    # Verification - 2nd buff ID should be used, since the effect itself post-assigns it
    assert api_module.update().attrs[eve_buff_type_attr_id].modified == approx(eve_buff2_id)
    api_ship.update()
    assert api_ship.attrs[eve_affectee_attr1_id].modified == approx(7.5)
    assert api_ship.attrs[eve_affectee_attr2_id].modified == approx(37.5)


def test_attr_change_stop(client, consts):
    # Stopping the effect changes buff type back, which re-generates buffs during removal. Switch
    # effect makes the replaced buff come from an attribute change instead of effect start, and
    # buff 1 is bigger than buff 2, so leftovers show regardless of modifier processing order
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_assign_attr_id = client.mk_eve_attr()
    eve_switch_attr_id = client.mk_eve_attr()
    eve_affectee_attr1_id = client.mk_eve_attr()
    eve_affectee_attr2_id = client.mk_eve_attr()
    eve_affectee_attr3_id = client.mk_eve_attr()
    eve_buff1_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[
            client.mk_eve_buff_mod(attr_id=eve_affectee_attr1_id),
            client.mk_eve_buff_mod(attr_id=eve_affectee_attr2_id)])
    eve_buff2_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr3_id)])
    eve_assign_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_assign_attr_id,
        affectee_attr_id=eve_buff_type_attr_id)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.buff_fleet_ships,
        cat_id=consts.EveEffCat.active,
        mod_info=[eve_assign_mod])
    eve_switch_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_switch_attr_id,
        affectee_attr_id=eve_assign_attr_id)
    eve_switch_effect_id = client.mk_eve_effect(mod_info=[eve_switch_mod])
    eve_module_id = client.mk_eve_item(
        attrs={
            eve_buff_type_attr_id: eve_buff1_id,
            eve_buff_val_attr_id: 5,
            eve_assign_attr_id: eve_buff1_id,
            eve_switch_attr_id: eve_buff2_id},
        eff_ids=[eve_effect_id, eve_switch_effect_id],
        defeff_id=eve_effect_id)
    # Keeps the assigned buff during data generation
    client.mk_eve_item(attrs={eve_buff_type_attr_id: eve_buff2_id})
    eve_ship_id = client.mk_eve_ship(
        attrs={eve_affectee_attr1_id: 7.5, eve_affectee_attr2_id: 7.5, eve_affectee_attr3_id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        effect_modes={eve_switch_effect_id: consts.ApiEffMode.force_stop})
    # Verification
    assert api_module.update().attrs[eve_buff_type_attr_id].modified == approx(eve_buff1_id)
    api_ship.update()
    assert api_ship.attrs[eve_affectee_attr1_id].modified == approx(37.5)
    assert api_ship.attrs[eve_affectee_attr2_id].modified == approx(37.5)
    assert api_ship.attrs[eve_affectee_attr3_id].modified == approx(7.5)
    # Action
    api_module.change_module(effect_modes={eve_switch_effect_id: consts.ApiEffMode.force_run})
    # Verification
    assert api_module.update().attrs[eve_buff_type_attr_id].modified == approx(eve_buff2_id)
    api_ship.update()
    assert api_ship.attrs[eve_affectee_attr1_id].modified == approx(7.5)
    assert api_ship.attrs[eve_affectee_attr2_id].modified == approx(7.5)
    assert api_ship.attrs[eve_affectee_attr3_id].modified == approx(37.5)
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    # Verification
    assert api_module.update().attrs[eve_buff_type_attr_id].modified == approx(eve_buff1_id)
    api_ship.update()
    assert api_ship.attrs[eve_affectee_attr1_id].modified == approx(7.5)
    assert api_ship.attrs[eve_affectee_attr2_id].modified == approx(7.5)
    assert api_ship.attrs[eve_affectee_attr3_id].modified == approx(7.5)


def test_attr_change_stop_other_effect(client, consts):
    # Buffs of another effect of the same item are re-generated too, and have to stay. The other
    # effect buffs only the fleeted ship to check effects separately; layout as in stop test
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_assign_attr_id = client.mk_eve_attr()
    eve_switch_attr_id = client.mk_eve_attr()
    eve_affectee_attr1_id = client.mk_eve_attr()
    eve_affectee_attr2_id = client.mk_eve_attr()
    eve_affectee_attr3_id = client.mk_eve_attr()
    eve_buff1_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[
            client.mk_eve_buff_mod(attr_id=eve_affectee_attr1_id),
            client.mk_eve_buff_mod(attr_id=eve_affectee_attr2_id)])
    eve_buff2_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr3_id)])
    eve_assign_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_assign_attr_id,
        affectee_attr_id=eve_buff_type_attr_id)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.buff_fleet_ships,
        cat_id=consts.EveEffCat.active,
        mod_info=[eve_assign_mod])
    eve_other_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.buff_fleet_filtered,
        cat_id=consts.EveEffCat.active)
    eve_switch_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_switch_attr_id,
        affectee_attr_id=eve_assign_attr_id)
    eve_switch_effect_id = client.mk_eve_effect(mod_info=[eve_switch_mod])
    eve_module_id = client.mk_eve_item(
        attrs={
            eve_buff_type_attr_id: eve_buff1_id,
            eve_buff_val_attr_id: 5,
            eve_assign_attr_id: eve_buff1_id,
            eve_switch_attr_id: eve_buff2_id},
        eff_ids=[eve_effect_id, eve_other_effect_id, eve_switch_effect_id],
        defeff_id=eve_effect_id)
    # Keeps the assigned buff during data generation
    client.mk_eve_item(attrs={eve_buff_type_attr_id: eve_buff2_id})
    eve_self_ship_id = client.mk_eve_ship(
        attrs={eve_affectee_attr1_id: 7.5, eve_affectee_attr2_id: 7.5, eve_affectee_attr3_id: 7.5})
    eve_fleeted_ship_id = client.mk_eve_ship(
        attrs={eve_affectee_attr1_id: 7.5, eve_affectee_attr2_id: 7.5, eve_affectee_attr3_id: 7.5})
    client.mk_eve_item_list(id_=consts.UtilItemList.buff_fleet_filter, inc_type_ids=[eve_fleeted_ship_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fleet = api_sol.create_fleet()
    api_self_fit = api_sol.create_fit(fleet_id=api_fleet.id)
    api_self_ship = api_self_fit.set_ship(type_id=eve_self_ship_id)
    api_fleeted_fit = api_sol.create_fit(fleet_id=api_fleet.id)
    api_fleeted_ship = api_fleeted_fit.set_ship(type_id=eve_fleeted_ship_id)
    api_module = api_self_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        effect_modes={
            eve_other_effect_id: consts.ApiEffMode.state_compliance,
            eve_switch_effect_id: consts.ApiEffMode.force_stop})
    # Action
    api_module.change_module(effect_modes={eve_switch_effect_id: consts.ApiEffMode.force_run})
    # Verification
    assert api_module.update().attrs[eve_buff_type_attr_id].modified == approx(eve_buff2_id)
    api_fleeted_ship.update()
    assert api_fleeted_ship.attrs[eve_affectee_attr1_id].modified == approx(7.5)
    assert api_fleeted_ship.attrs[eve_affectee_attr2_id].modified == approx(7.5)
    assert api_fleeted_ship.attrs[eve_affectee_attr3_id].modified == approx(37.5)
    # Action
    api_module.change_module(effect_modes={eve_effect_id: consts.ApiEffMode.force_stop})
    # Verification
    assert api_module.update().attrs[eve_buff_type_attr_id].modified == approx(eve_buff1_id)
    api_fleeted_ship.update()
    assert api_fleeted_ship.attrs[eve_affectee_attr1_id].modified == approx(37.5)
    assert api_fleeted_ship.attrs[eve_affectee_attr2_id].modified == approx(37.5)
    assert api_fleeted_ship.attrs[eve_affectee_attr3_id].modified == approx(7.5)
    api_self_ship.update()
    assert api_self_ship.attrs[eve_affectee_attr1_id].modified == approx(7.5)
    assert api_self_ship.attrs[eve_affectee_attr2_id].modified == approx(7.5)
    assert api_self_ship.attrs[eve_affectee_attr3_id].modified == approx(7.5)
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_fleeted_ship.update()
    assert api_fleeted_ship.attrs[eve_affectee_attr1_id].modified == approx(7.5)
    assert api_fleeted_ship.attrs[eve_affectee_attr2_id].modified == approx(7.5)
    assert api_fleeted_ship.attrs[eve_affectee_attr3_id].modified == approx(7.5)
