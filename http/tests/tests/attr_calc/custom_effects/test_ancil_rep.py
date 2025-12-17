from fw import Effect, approx, check_no_field


def test_local_aar(client, consts):
    # Check that paste boost works on local ancillary repairer
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charged_armor_dmg_mult)
    eve_affectee_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.fueled_armor_repair)
    eve_aar_item_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 3, eve_affectee_attr_id: 100},
        eff_ids=[eve_effect_id])
    eve_paste_item_id = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_aar_item = api_fit.add_module(
        type_id=eve_aar_item_id,
        rack=consts.ApiRack.low,
        charge_type_id=eve_paste_item_id)
    # Verification
    api_aar_item.update()
    assert api_aar_item.attrs[eve_affectee_attr_id].dogma == approx(100)
    assert api_aar_item.attrs[eve_affectee_attr_id].extra == approx(300)
    api_mod = api_aar_item.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.extra_mul
    assert api_mod.initial_val == approx(3)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(3)
    assert api_mod.affectors.one().item_id == api_aar_item.id
    assert api_mod.affectors.one().attr_id == eve_affector_attr_id


def test_remote_aar(client, consts):
    # Check that paste boost works on remote ancillary repairer
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charged_armor_dmg_mult)
    eve_affectee_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.ship_mod_ancillary_remote_armor_repairer)
    eve_aar_item_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 3, eve_affectee_attr_id: 100},
        eff_ids=[eve_effect_id])
    eve_paste_item_id = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_aar_item = api_fit.add_module(
        type_id=eve_aar_item_id,
        rack=consts.ApiRack.high,
        charge_type_id=eve_paste_item_id)
    # Verification
    api_aar_item.update()
    assert api_aar_item.attrs[eve_affectee_attr_id].dogma == approx(100)
    assert api_aar_item.attrs[eve_affectee_attr_id].extra == approx(300)
    api_mod = api_aar_item.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.extra_mul
    assert api_mod.initial_val == approx(3)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(3)
    assert api_mod.affectors.one().item_id == api_aar_item.id
    assert api_mod.affectors.one().attr_id == eve_affector_attr_id


def test_charge_switch(client, consts):
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charged_armor_dmg_mult)
    eve_affectee_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.fueled_armor_repair)
    eve_aar_item_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 3, eve_affectee_attr_id: 100},
        eff_ids=[eve_effect_id])
    eve_paste_item_id = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_aar_item = api_fit.add_module(type_id=eve_aar_item_id, rack=consts.ApiRack.low)
    # Verification
    api_aar_item.update()
    assert api_aar_item.attrs[eve_affectee_attr_id].dogma == approx(100)
    assert api_aar_item.attrs[eve_affectee_attr_id].extra == approx(100)
    with check_no_field():
        api_aar_item.mods  # noqa: B018
    # Action
    api_aar_item.change_module(charge_type_id=eve_paste_item_id)
    # Verification
    api_aar_item.update()
    assert api_aar_item.attrs[eve_affectee_attr_id].dogma == approx(100)
    assert api_aar_item.attrs[eve_affectee_attr_id].extra == approx(300)
    assert len(api_aar_item.mods[eve_affectee_attr_id]) == 1
    # Action
    api_aar_item.change_module(charge_type_id=None)
    # Verification
    api_aar_item.update()
    assert api_aar_item.attrs[eve_affectee_attr_id].dogma == approx(100)
    assert api_aar_item.attrs[eve_affectee_attr_id].extra == approx(100)
    with check_no_field():
        api_aar_item.mods  # noqa: B018


def test_charge_type_id_switch(client, consts):
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charged_armor_dmg_mult)
    eve_affectee_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.fueled_armor_repair)
    eve_aar_item_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 3, eve_affectee_attr_id: 100},
        eff_ids=[eve_effect_id])
    eve_paste_item_id = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste)
    eve_other_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_aar_item = api_fit.add_module(
        type_id=eve_aar_item_id,
        rack=consts.ApiRack.low,
        charge_type_id=eve_paste_item_id)
    # Verification
    api_aar_item.update()
    assert api_aar_item.attrs[eve_affectee_attr_id].dogma == approx(100)
    assert api_aar_item.attrs[eve_affectee_attr_id].extra == approx(300)
    assert len(api_aar_item.mods[eve_affectee_attr_id]) == 1
    # Action
    api_aar_item.charge.change_charge(type_id=eve_other_item_id)
    # Verification
    api_aar_item.update()
    assert api_aar_item.attrs[eve_affectee_attr_id].dogma == approx(100)
    assert api_aar_item.attrs[eve_affectee_attr_id].extra == approx(100)
    with check_no_field():
        api_aar_item.mods  # noqa: B018
    # Action
    api_aar_item.charge.change_charge(type_id=eve_paste_item_id)
    # Verification
    api_aar_item.update()
    assert api_aar_item.attrs[eve_affectee_attr_id].dogma == approx(100)
    assert api_aar_item.attrs[eve_affectee_attr_id].extra == approx(300)
    assert len(api_aar_item.mods[eve_affectee_attr_id]) == 1


def test_mult_change(client, consts):
    eve_ship_id = client.mk_eve_ship()
    eve_aar_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charged_armor_dmg_mult)
    eve_aar_affectee_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount)
    eve_mod_affector_attr_id = client.mk_eve_attr()
    eve_aar_effect_id = client.mk_eve_effect(id_=consts.EveEffect.fueled_armor_repair)
    eve_aar_item_id = client.mk_eve_item(
        attrs={eve_aar_affector_attr_id: 3, eve_aar_affectee_attr_id: 100},
        eff_ids=[eve_aar_effect_id])
    eve_paste_item_id = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_affector_attr_id,
        affectee_attr_id=eve_aar_affector_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_mod_item_id = client.mk_eve_item(attrs={eve_mod_affector_attr_id: 25}, eff_ids=[eve_mod_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_aar_item = api_fit.add_module(
        type_id=eve_aar_item_id,
        rack=consts.ApiRack.low,
        charge_type_id=eve_paste_item_id)
    # Verification
    api_aar_item.update()
    assert api_aar_item.attrs[eve_aar_affectee_attr_id].dogma == approx(100)
    assert api_aar_item.attrs[eve_aar_affectee_attr_id].extra == approx(300)
    # Action
    api_mod_item = api_fit.add_rig(type_id=eve_mod_item_id)
    # Verification
    api_aar_item.update()
    assert api_aar_item.attrs[eve_aar_affectee_attr_id].dogma == approx(100)
    assert api_aar_item.attrs[eve_aar_affectee_attr_id].extra == approx(375)
    # Action
    api_mod_item.remove()
    # Verification
    api_aar_item.update()
    assert api_aar_item.attrs[eve_aar_affectee_attr_id].dogma == approx(100)
    assert api_aar_item.attrs[eve_aar_affectee_attr_id].extra == approx(300)


def test_penalties(client, consts):
    # Check that paste multiplier is not stacking penalized against other multiplications
    eve_ship_id = client.mk_eve_ship()
    eve_aar_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charged_armor_dmg_mult)
    eve_aar_affectee_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount, stackable=False)
    eve_mod_affector_attr_id = client.mk_eve_attr()
    eve_aar_effect_id = client.mk_eve_effect(id_=consts.EveEffect.fueled_armor_repair)
    eve_aar_id = client.mk_eve_item(
        attrs={eve_aar_affector_attr_id: 3, eve_aar_affectee_attr_id: 100},
        eff_ids=[eve_aar_effect_id])
    eve_paste_id = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste)
    eve_rig_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_mod_affector_attr_id,
        affectee_attr_id=eve_aar_affectee_attr_id)
    eve_rig_effect_id = client.mk_eve_effect(mod_info=[eve_rig_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_affector_attr_id: 1.5}, eff_ids=[eve_rig_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_aar = api_fit.add_module(type_id=eve_aar_id, rack=consts.ApiRack.low, charge_type_id=eve_paste_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_aar.update()
    assert api_aar.attrs[eve_aar_affectee_attr_id].dogma == approx(150)
    assert api_aar.attrs[eve_aar_affectee_attr_id].extra == approx(450)
    api_mods = api_aar.mods[eve_aar_affectee_attr_id]
    assert len(api_mods) == 2
    api_mod_paste = api_mods.find_by_affector_item(affector_item_id=api_aar.id).one()
    assert api_mod_paste.op == consts.ApiModOp.extra_mul
    assert api_mod_paste.initial_val == approx(3)
    assert api_mod_paste.stacking_mult is None
    assert api_mod_paste.applied_val == approx(3)
    api_mod_rig = api_mods.find_by_affector_item(affector_item_id=api_rig.id).one()
    assert api_mod_rig.op == consts.ApiModOp.post_mul
    assert api_mod_rig.initial_val == approx(1.5)
    assert api_mod_rig.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_mod_rig.applied_val == approx(1.5)


def test_state(client, consts):
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charged_armor_dmg_mult)
    eve_affectee_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.fueled_armor_repair)
    eve_aar_item_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 3, eve_affectee_attr_id: 100},
        eff_ids=[eve_effect_id])
    eve_paste_item_id = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste)
    client.create_sources()
    api_custom_effect_id = Effect.custom_to_api(custom_effect_id=consts.CustomEffect.aar_paste_boost)
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_aar_item = api_fit.add_module(
        type_id=eve_aar_item_id,
        rack=consts.ApiRack.low,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_paste_item_id)
    # Verification
    assert api_aar_item.update().attrs[eve_affectee_attr_id].extra == approx(300)
    # Action
    api_aar_item.change_module(state=consts.ApiModuleState.disabled)
    # Verification
    assert api_aar_item.update().attrs[eve_affectee_attr_id].extra == approx(300)
    # Action
    api_aar_item.change_module(state=consts.ApiModuleState.active)
    # Verification
    assert api_aar_item.update().attrs[eve_affectee_attr_id].extra == approx(300)
    # Action
    api_aar_item.change_module(effect_modes={api_custom_effect_id: consts.ApiEffMode.force_stop})
    # Verification
    assert api_aar_item.update().attrs[eve_affectee_attr_id].extra == approx(100)
    # Action & verification
    api_aar_item.change_module(charge_type_id=None)
    api_aar_item.remove()
