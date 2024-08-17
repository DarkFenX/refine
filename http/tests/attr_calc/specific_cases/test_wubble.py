from tests import approx


def test_module_self(client, consts):
    # Shouldn't affect ship of owner even if activated
    eve_affector_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.min,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_charge = client.mk_eve_item(id_=consts.EveItem.stasis_webification_probe, attrs={eve_affector_attr.id: -40})
    eve_module_effect = client.mk_eve_effect(id_=consts.EveEffect.use_missiles, cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(eff_ids=[eve_module_effect.id], defeff_id=eve_module_effect.id)
    eve_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_fit.add_mod(type_id=eve_module.id, state=consts.ApiState.active, charge_type_id=eve_charge.id)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1000)


def test_module_bundled_proj_unproj(client, consts):
    eve_affector_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.min,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_charge = client.mk_eve_item(id_=consts.EveItem.stasis_webification_probe, attrs={eve_affector_attr.id: -40})
    eve_module_effect = client.mk_eve_effect(id_=consts.EveEffect.use_missiles, cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(eff_ids=[eve_module_effect.id], defeff_id=eve_module_effect.id)
    eve_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship.id)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_mod(
        type_id=eve_module.id,
        state=consts.ApiState.active,
        charge_type_id=eve_charge.id)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1000)
    # Action
    api_affector_module.change_mod(add_projs=[api_affectee_ship.id])
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(600)
    # Action
    api_affector_module.change_mod(rm_projs=[api_affectee_ship.id])
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1000)


def test_module_bundled_remove(client, consts):
    eve_affector_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.min,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_charge = client.mk_eve_item(id_=consts.EveItem.stasis_webification_probe, attrs={eve_affector_attr.id: -40})
    eve_module_effect = client.mk_eve_effect(id_=consts.EveEffect.use_missiles, cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(eff_ids=[eve_module_effect.id], defeff_id=eve_module_effect.id)
    eve_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship.id)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_mod(
        type_id=eve_module.id,
        state=consts.ApiState.active,
        charge_type_id=eve_charge.id)
    api_affector_module.change_mod(add_projs=[api_affectee_ship.id])
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(600)
    # Action
    api_affector_module.remove()
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1000)


def test_module_charge_uncharge(client, consts):
    eve_affector_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.min,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_charge = client.mk_eve_item(id_=consts.EveItem.stasis_webification_probe, attrs={eve_affector_attr.id: -40})
    eve_module_effect = client.mk_eve_effect(id_=consts.EveEffect.use_missiles, cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(eff_ids=[eve_module_effect.id], defeff_id=eve_module_effect.id)
    eve_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship.id)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_affector_module.change_mod(add_projs=[api_affectee_ship.id])
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1000)
    # Action
    api_affector_module.change_mod(charge=eve_charge.id)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(600)
    # Action
    api_affector_module.change_mod(charge=None)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1000)


def test_module_state_up_state_down(client, consts):
    eve_affector_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.min,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_charge = client.mk_eve_item(id_=consts.EveItem.stasis_webification_probe, attrs={eve_affector_attr.id: -40})
    eve_module_effect = client.mk_eve_effect(id_=consts.EveEffect.use_missiles, cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(eff_ids=[eve_module_effect.id], defeff_id=eve_module_effect.id)
    eve_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship.id)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_mod(
        type_id=eve_module.id,
        state=consts.ApiState.online,
        charge_type_id=eve_charge.id)
    api_affector_module.change_mod(add_projs=[api_affectee_ship.id])
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1000)
    # Action
    api_affector_module.change_mod(state=consts.ApiState.active)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(600)
    # Action
    api_affector_module.change_mod(state=consts.ApiState.online)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1000)


def test_charge_proj_effect(client, consts):
    eve_affector_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.min,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_charge = client.mk_eve_item(id_=consts.EveItem.stasis_webification_probe, attrs={eve_affector_attr.id: -40})
    eve_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_charge.id)
    api_proj_effect.change_proj_effect(add_projs=[api_ship.id])
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr.id].dogma == approx(600)
