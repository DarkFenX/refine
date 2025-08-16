from tests import approx


def test_module_self(client, consts):
    # Shouldn't affect ship of owner even if activated
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.min,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_charge_id = client.mk_eve_item(id_=consts.EveItem.stasis_webification_probe, attrs={eve_affector_attr_id: -40})
    eve_module_effect_id = client.mk_eve_effect(id_=consts.EveEffect.use_missiles, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(eff_ids=[eve_module_effect_id], defeff_id=eve_module_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1000)


def test_module_charge_uncharge(client, consts):
    # Check that it's charge which affects it, not module
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.min,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_charge_id = client.mk_eve_item(id_=consts.EveItem.stasis_webification_probe, attrs={eve_affector_attr_id: -40})
    eve_module_effect_id = client.mk_eve_effect(id_=consts.EveEffect.use_missiles, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(eff_ids=[eve_module_effect_id], defeff_id=eve_module_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship_id)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_affector_module.change_module(add_projs=[api_affectee_ship.id])
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1000)
    # Action
    api_affector_module.change_module(charge_type_id=eve_charge_id)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(600)
    # Action
    api_affector_module.change_module(charge_type_id=None)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1000)


def test_module_state_up_state_down(client, consts):
    # Check that when module is not active, charge does not affect projectee
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.min,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_charge_id = client.mk_eve_item(id_=consts.EveItem.stasis_webification_probe, attrs={eve_affector_attr_id: -40})
    eve_module_effect_id = client.mk_eve_effect(id_=consts.EveEffect.use_missiles, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(eff_ids=[eve_module_effect_id], defeff_id=eve_module_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship_id)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.online,
        charge_type_id=eve_charge_id)
    api_affector_module.change_module(add_projs=[api_affectee_ship.id])
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1000)
    # Action
    api_affector_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(600)
    # Action
    api_affector_module.change_module(state=consts.ApiModuleState.online)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1000)


def test_module_range(client, consts):
    # Check that wubbles use specific range attribute, and uses center-to-surface range to apply its
    # modifiers
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_aoe_range)
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.min,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_charge_id = client.mk_eve_item(
        id_=consts.EveItem.stasis_webification_probe,
        attrs={eve_affector_attr_id: -40, eve_range_attr_id: 15000})
    eve_module_effect_id = client.mk_eve_effect(id_=consts.EveEffect.use_missiles, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(eff_ids=[eve_module_effect_id], defeff_id=eve_module_effect_id)
    eve_affector_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 5000})
    eve_affectee_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 1000, eve_radius_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_fit.set_ship(type_id=eve_affector_ship_id, coordinates=(0, 0, 0))
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id, coordinates=(15501, 0, 0))
    api_affector_module.change_module(add_projs=[api_affectee_ship.id])
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1000)
    # Action
    api_affectee_ship.change_ship(coordinates=(15500, 0, 0))
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(600)
    # Action
    api_affectee_ship.change_ship(coordinates=(15501, 0, 0))
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1000)


def test_charge_proj_effect(client, consts):
    # Check how web bubble works when it's a projected effect - happens to work because it's a buff
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.min,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_charge_id = client.mk_eve_item(id_=consts.EveItem.stasis_webification_probe, attrs={eve_affector_attr_id: -40})
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_charge_id)
    api_proj_effect.change_proj_effect(add_projs=[api_ship.id])
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(600)
