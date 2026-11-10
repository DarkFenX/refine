from fw import approx


def test_local(client, consts):
    # All effects which were running stop, while another one starts
    eve_affector_attr1_id = client.mk_eve_attr()
    eve_affector_attr2_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr1_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect1_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_mod1])
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr2_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect2_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_mod2])
    eve_module_id = client.mk_eve_item(
        attrs={eve_affector_attr1_id: 20, eve_affector_attr2_id: 30, eve_affectee_attr_id: 100},
        eff_ids=[eve_effect1_id, eve_effect2_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.online,
        effect_modes={eve_effect2_id: consts.ApiEffMode.force_stop})
    # Verification
    api_module.update()
    assert api_module.attrs[eve_affectee_attr_id].modified == approx(120)
    assert api_module.effects[eve_effect1_id].running is True
    assert api_module.effects[eve_effect2_id].running is False
    # Action
    api_module.change_module(effect_modes={
        eve_effect1_id: consts.ApiEffMode.force_stop,
        eve_effect2_id: consts.ApiEffMode.full_compliance})
    # Verification
    api_module.update()
    assert api_module.attrs[eve_affectee_attr_id].modified == approx(130)
    assert api_module.effects[eve_effect1_id].running is False
    assert api_module.effects[eve_effect2_id].running is True
    # Action
    api_module.change_module(effect_modes={
        eve_effect1_id: consts.ApiEffMode.full_compliance,
        eve_effect2_id: consts.ApiEffMode.force_stop})
    # Verification
    api_module.update()
    assert api_module.attrs[eve_affectee_attr_id].modified == approx(120)
    assert api_module.effects[eve_effect1_id].running is True
    assert api_module.effects[eve_effect2_id].running is False
    # Action
    api_module.change_module(effect_modes={eve_effect2_id: consts.ApiEffMode.full_compliance})
    # Verification
    api_module.update()
    assert api_module.attrs[eve_affectee_attr_id].modified == approx(156)
    assert api_module.effects[eve_effect1_id].running is True
    assert api_module.effects[eve_effect2_id].running is True


def test_proj(client, consts):
    # Started effects are projected and stopped ones unprojected in the same update
    eve_affector_attr1_id = client.mk_eve_attr()
    eve_affector_attr2_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr1_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect1_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod1])
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr2_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect2_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod2])
    eve_module_id = client.mk_eve_item(
        attrs={eve_affector_attr1_id: 20, eve_affector_attr2_id: 30},
        eff_ids=[eve_effect1_id, eve_effect2_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship_id)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_affectee_ship.id],
        effect_modes={
            eve_effect1_id: consts.ApiEffMode.force_run,
            eve_effect2_id: consts.ApiEffMode.force_stop})
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].modified == approx(120)
    # Action
    api_affector_module.change_module(effect_modes={
        eve_effect1_id: consts.ApiEffMode.force_stop,
        eve_effect2_id: consts.ApiEffMode.force_run})
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].modified == approx(130)
    # Action
    api_affector_module.change_module(effect_modes={
        eve_effect1_id: consts.ApiEffMode.force_run,
        eve_effect2_id: consts.ApiEffMode.force_stop})
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].modified == approx(120)
    # Action
    api_affector_module.change_module(rm_proj_item_ids=[api_affectee_ship.id])
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].modified == approx(100)
