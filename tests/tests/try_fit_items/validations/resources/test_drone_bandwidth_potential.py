from fw.api import ValOptions


def test_decreasers(client, consts):
    # Tried module which decreases bandwidth is switched off when potential bandwidth is checked
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_max_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.online, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_module(attrs={eve_mod_attr_id: -50}, eff_ids=[eve_mod_effect_id])
    eve_drone_id = client.mk_eve_drone(attrs={eve_use_attr_id: 100})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification - the module halves current bandwidth, the drone fits only into potential one
    api_fit.check_try_fit_items(
        val_options=ValOptions(drone_bandwidth_current=True),
        passing=[],
        failing=[eve_module_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(drone_bandwidth_potential=True),
        passing=[eve_module_id],
        failing=[])


def test_known_failures(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone_id = client.mk_eve_drone(attrs={eve_use_attr_id: 100})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone1 = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    api_drone2 = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification - drones are tried in bay
    api_fit.check_try_fit_items(
        val_options=ValOptions(drone_bandwidth_potential=[api_drone1.id]),
        passing=[],
        failing=[eve_drone_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(drone_bandwidth_potential=[api_drone2.id]),
        passing=[],
        failing=[eve_drone_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(drone_bandwidth_potential=[api_drone1.id, api_drone2.id]),
        passing=[eve_drone_id],
        failing=[])
