from tests import approx


def test_fail_single(client, consts):
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_slots_left)
    eve_rig_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.rig_slots])
    assert api_val.passed is False
    assert api_val.details.rig_slots.used == approx(1)
    assert api_val.details.rig_slots.total == approx(0)
    assert len(api_val.details.rig_slots.users) == 1
    assert api_rig.id in api_val.details.rig_slots.users
