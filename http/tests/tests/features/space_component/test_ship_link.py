"""
As of 2025-03-27, used only by ships linked to a CRAB or a skyhook. Does not affect child entities
like drones directly, otherwise linking to a CRAB would make carriers/supercarriers' fighters
inoperable.

Ship links have a list of items which are allowed to link to them. The library uses this list to
define if an item can receive ship link effect or not.
"""

from tests import Effect, approx


def test_affectee_filter_item(client, consts):
    eve_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_attr_id)])
    eve_beacon_id = client.mk_eve_item()
    client.mk_eve_space_comp(type_id=eve_beacon_id, sl_buffs={eve_buff_id: 10})
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr_id: 200})
    eve_struct_id = client.mk_eve_struct(attrs={eve_attr_id: 200})
    eve_drone_id = client.mk_eve_drone(attrs={eve_attr_id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.add_sw_effect(type_id=eve_beacon_id)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    assert api_ship.update().attrs[eve_attr_id].dogma == approx(220)
    assert api_drone.update().attrs[eve_attr_id].dogma == approx(200)
    # Action
    api_struct = api_fit.set_ship(type_id=eve_struct_id)
    # Verification
    assert api_struct.update().attrs[eve_attr_id].dogma == approx(200)
    assert api_drone.update().attrs[eve_attr_id].dogma == approx(200)


def test_affectee_filter_location(client, consts):
    eve_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_mods=[client.mk_eve_buff_mod(attr_id=eve_attr_id)])
    eve_beacon_id = client.mk_eve_item()
    client.mk_eve_space_comp(type_id=eve_beacon_id, sl_buffs={eve_buff_id: 10})
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr_id: 200})
    eve_struct_id = client.mk_eve_struct(attrs={eve_attr_id: 200})
    eve_item_id = client.mk_eve_item(attrs={eve_attr_id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.add_sw_effect(type_id=eve_beacon_id)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_item_id)
    api_implant = api_fit.add_implant(type_id=eve_item_id)
    # Verification
    assert api_ship.update().attrs[eve_attr_id].dogma == approx(200)
    assert api_rig.update().attrs[eve_attr_id].dogma == approx(220)
    assert api_implant.update().attrs[eve_attr_id].dogma == approx(200)
    # Action
    api_struct = api_fit.set_ship(type_id=eve_struct_id)
    # Verification
    assert api_struct.update().attrs[eve_attr_id].dogma == approx(200)
    assert api_rig.update().attrs[eve_attr_id].dogma == approx(200)
    assert api_implant.update().attrs[eve_attr_id].dogma == approx(200)


def test_affectee_filter_location_group(client, consts):
    eve_ship_group_id = client.mk_eve_ship_group()
    eve_struct_group_id = client.mk_eve_struct_group()
    eve_item_group_id = client.mk_eve_item_group()
    eve_other_group_id = client.mk_eve_item_group()
    eve_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_grp_mods=[
            client.mk_eve_buff_mod(attr_id=eve_attr_id, group_id=eve_ship_group_id),
            client.mk_eve_buff_mod(attr_id=eve_attr_id, group_id=eve_struct_group_id),
            client.mk_eve_buff_mod(attr_id=eve_attr_id, group_id=eve_item_group_id)])
    eve_beacon_id = client.mk_eve_item()
    client.mk_eve_space_comp(type_id=eve_beacon_id, sl_buffs={eve_buff_id: 10})
    eve_ship_id = client.mk_eve_ship(grp_id=eve_ship_group_id, attrs={eve_attr_id: 200})
    eve_struct_id = client.mk_eve_struct(grp_id=eve_struct_group_id, attrs={eve_attr_id: 200})
    eve_item_id = client.mk_eve_item(grp_id=eve_item_group_id, attrs={eve_attr_id: 200})
    eve_other_id = client.mk_eve_item(grp_id=eve_other_group_id, attrs={eve_attr_id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.add_sw_effect(type_id=eve_beacon_id)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rig1 = api_fit.add_rig(type_id=eve_item_id)
    api_rig2 = api_fit.add_rig(type_id=eve_other_id)
    api_implant = api_fit.add_implant(type_id=eve_item_id)
    # Verification
    assert api_ship.update().attrs[eve_attr_id].dogma == approx(200)
    assert api_rig1.update().attrs[eve_attr_id].dogma == approx(220)
    assert api_rig2.update().attrs[eve_attr_id].dogma == approx(200)
    assert api_implant.update().attrs[eve_attr_id].dogma == approx(200)
    # Action
    api_struct = api_fit.set_ship(type_id=eve_struct_id)
    # Verification
    assert api_struct.update().attrs[eve_attr_id].dogma == approx(200)
    assert api_rig1.update().attrs[eve_attr_id].dogma == approx(200)
    assert api_rig2.update().attrs[eve_attr_id].dogma == approx(200)
    assert api_implant.update().attrs[eve_attr_id].dogma == approx(200)


def test_affectee_filter_location_skillreq(client, consts):
    eve_skill1_id = client.mk_eve_item()
    eve_skill2_id = client.mk_eve_item()
    eve_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_attr_id, skill_id=eve_skill1_id)])
    eve_beacon_id = client.mk_eve_item()
    client.mk_eve_space_comp(type_id=eve_beacon_id, sl_buffs={eve_buff_id: 10})
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr_id: 200}, srqs={eve_skill1_id: 1})
    eve_struct_id = client.mk_eve_struct(attrs={eve_attr_id: 200}, srqs={eve_skill1_id: 1})
    eve_item_id = client.mk_eve_item(attrs={eve_attr_id: 200}, srqs={eve_skill1_id: 1})
    eve_other_id = client.mk_eve_item(attrs={eve_attr_id: 200}, srqs={eve_skill2_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.add_sw_effect(type_id=eve_beacon_id)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rig1 = api_fit.add_rig(type_id=eve_item_id)
    api_rig2 = api_fit.add_rig(type_id=eve_other_id)
    api_implant = api_fit.add_implant(type_id=eve_item_id)
    # Verification
    assert api_ship.update().attrs[eve_attr_id].dogma == approx(200)
    assert api_rig1.update().attrs[eve_attr_id].dogma == approx(220)
    assert api_rig2.update().attrs[eve_attr_id].dogma == approx(200)
    assert api_implant.update().attrs[eve_attr_id].dogma == approx(200)
    # Action
    api_struct = api_fit.set_ship(type_id=eve_struct_id)
    # Verification
    assert api_struct.update().attrs[eve_attr_id].dogma == approx(200)
    assert api_rig1.update().attrs[eve_attr_id].dogma == approx(200)
    assert api_rig2.update().attrs[eve_attr_id].dogma == approx(200)
    assert api_implant.update().attrs[eve_attr_id].dogma == approx(200)


def test_state(client, consts):
    eve_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_attr_id)])
    eve_beacon_id = client.mk_eve_item()
    client.mk_eve_space_comp(type_id=eve_beacon_id, sl_buffs={eve_buff_id: 10})
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr_id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.add_sw_effect(type_id=eve_beacon_id, state=False)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    assert api_ship.update().attrs[eve_attr_id].dogma == approx(200)


def test_effect_mode(client, consts):
    eve_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_attr_id)])
    eve_beacon_id = client.mk_eve_item()
    client.mk_eve_space_comp(type_id=eve_beacon_id, sl_buffs={eve_buff_id: 10})
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr_id: 200})
    client.create_sources()
    api_effect_id = Effect.scsl_to_api(type_id=eve_beacon_id)
    api_sol = client.create_sol()
    api_sw_effect = api_sol.add_sw_effect(type_id=eve_beacon_id)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    assert api_ship.update().attrs[eve_attr_id].dogma == approx(220)
    # Action
    api_sw_effect.change_sw_effect(effect_modes={api_effect_id: consts.ApiEffMode.force_stop})
    # Verification
    assert api_ship.update().attrs[eve_attr_id].dogma == approx(200)
    # Action
    api_sw_effect.change_sw_effect(effect_modes={api_effect_id: consts.ApiEffMode.force_run})
    # Verification
    assert api_ship.update().attrs[eve_attr_id].dogma == approx(220)
