from fw import check_no_field


def test_npc_prop_set_not_loaded(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_cruise_speed_attr_id = client.mk_eve_attr(datas=[eve_d1], id_=consts.EveAttr.entity_cruise_speed)
    eve_drone_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_drone(datas=[eve_d1], id_=eve_drone_id, attrs={eve_cruise_speed_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1, default_npc_prop=consts.ApiNpcProp.cruise)
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    api_drone.update()
    assert api_drone.npc_prop_effective == consts.ApiNpcProp.cruise
    with check_no_field():
        api_drone.npc_prop_override  # ruff:ignore[useless-expression]
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    api_drone.update()
    with check_no_field():
        api_drone.npc_prop_effective  # ruff:ignore[useless-expression]
    with check_no_field():
        api_drone.npc_prop_override  # ruff:ignore[useless-expression]
    # Action
    api_drone.change_drone(npc_prop_override=consts.ApiNpcProp.chase)
    # Verification
    api_drone.update()
    with check_no_field():
        api_drone.npc_prop_effective  # ruff:ignore[useless-expression]
    assert api_drone.npc_prop_override == consts.ApiNpcProp.chase
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    api_drone.update()
    assert api_drone.npc_prop_effective == consts.ApiNpcProp.chase
    assert api_drone.npc_prop_override == consts.ApiNpcProp.chase


def test_npc_prop_single_prop(client, consts):
    # The setting is used only by drones which have cruise speed, i.e. two propulsion modes
    eve_cruise_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.entity_cruise_speed)
    eve_drone1_id = client.mk_eve_drone(attrs={eve_cruise_speed_attr_id: 500})
    eve_drone2_id = client.mk_eve_drone()
    eve_drone3_id = client.mk_eve_drone(attrs={eve_cruise_speed_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol(default_npc_prop=consts.ApiNpcProp.cruise)
    api_fit = api_sol.create_fit()
    # Action
    api_drone = api_fit.add_drone(type_id=eve_drone1_id)
    # Verification
    api_drone.update()
    assert api_drone.npc_prop_effective == consts.ApiNpcProp.cruise
    with check_no_field():
        api_drone.npc_prop_override  # ruff:ignore[useless-expression]
    # Action
    api_drone.change_drone(npc_prop_override=consts.ApiNpcProp.chase)
    # Verification
    api_drone.update()
    assert api_drone.npc_prop_effective == consts.ApiNpcProp.chase
    assert api_drone.npc_prop_override == consts.ApiNpcProp.chase
    # Action
    api_drone.change_drone(type_id=eve_drone2_id)
    # Verification - override is kept, but nothing uses it when the drone has one propulsion mode
    api_drone.update()
    with check_no_field():
        api_drone.npc_prop_effective  # ruff:ignore[useless-expression]
    assert api_drone.npc_prop_override == consts.ApiNpcProp.chase
    # Action
    api_drone.change_drone(type_id=eve_drone3_id)
    # Verification - same when cruise speed is zero
    api_drone.update()
    with check_no_field():
        api_drone.npc_prop_effective  # ruff:ignore[useless-expression]
    assert api_drone.npc_prop_override == consts.ApiNpcProp.chase
    # Action
    api_drone.change_drone(type_id=eve_drone1_id)
    # Verification
    api_drone.update()
    assert api_drone.npc_prop_effective == consts.ApiNpcProp.chase
    assert api_drone.npc_prop_override == consts.ApiNpcProp.chase
