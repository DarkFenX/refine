"""
As of 2025-03-27, system buff emitter is used just for insurgency suppression tackle range bonus.
Since it applies only location-based changes, it is not really possible to test if it affects all
entities like drones, or just ships. In the library, system buff emitter buffs are limited to just
ships.
"""

from tests import approx


def test_affectee_filter_item(client, consts):
    eve_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_attr_id)])
    eve_beacon_id = client.mk_eve_item()
    client.mk_eve_space_comp(type_id=eve_beacon_id, se_buffs={eve_buff_id: 10})
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr_id: 200})
    eve_struct_id = client.mk_eve_struct(attrs={eve_attr_id: 200})
    eve_drone_id = client.mk_eve_item(attrs={eve_attr_id: 200})
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
