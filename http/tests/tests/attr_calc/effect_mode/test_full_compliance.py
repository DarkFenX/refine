from tests import approx


def test_state_offline(client, consts):
    # Offline/passive effects are not run only due to effect fitting usage chance attribute
    # specified
    eve_chance_attr_id = client.mk_eve_attr()
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
    eve_effect2_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.passive,
        chance_attr_id=eve_chance_attr_id,
        mod_info=[eve_mod2])
    eve_item_id = client.mk_eve_item(
        attrs={
            eve_chance_attr_id: 0.2, eve_affector_attr1_id: 20,
            eve_affector_attr2_id: 30, eve_affectee_attr_id: 100},
        eff_ids=[eve_effect1_id, eve_effect2_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_item_id, state=consts.ApiModuleState.ghost)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(100)
    assert api_item.effects[eve_effect1_id].running is False
    assert api_item.effects[eve_effect1_id].mode == consts.ApiEffMode.full_compliance
    assert api_item.effects[eve_effect2_id].running is False
    assert api_item.effects[eve_effect2_id].mode == consts.ApiEffMode.full_compliance
    # Action
    api_item.change_mod(state=consts.ApiModuleState.offline)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(120)
    assert api_item.effects[eve_effect1_id].running is True
    assert api_item.effects[eve_effect1_id].mode == consts.ApiEffMode.full_compliance
    assert api_item.effects[eve_effect2_id].running is False
    assert api_item.effects[eve_effect2_id].mode == consts.ApiEffMode.full_compliance
    # Action
    api_item.change_mod(state=consts.ApiModuleState.online)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(120)
    assert api_item.effects[eve_effect1_id].running is True
    assert api_item.effects[eve_effect1_id].mode == consts.ApiEffMode.full_compliance
    assert api_item.effects[eve_effect2_id].running is False
    assert api_item.effects[eve_effect2_id].mode == consts.ApiEffMode.full_compliance
    # Action
    api_item.change_mod(state=consts.ApiModuleState.active)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(120)
    assert api_item.effects[eve_effect1_id].running is True
    assert api_item.effects[eve_effect1_id].mode == consts.ApiEffMode.full_compliance
    assert api_item.effects[eve_effect2_id].running is False
    assert api_item.effects[eve_effect2_id].mode == consts.ApiEffMode.full_compliance
    # Action
    api_item.change_mod(state=consts.ApiModuleState.overload)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(120)
    assert api_item.effects[eve_effect1_id].running is True
    assert api_item.effects[eve_effect1_id].mode == consts.ApiEffMode.full_compliance
    assert api_item.effects[eve_effect2_id].running is False
    assert api_item.effects[eve_effect2_id].mode == consts.ApiEffMode.full_compliance


def test_state_online_running(client, consts):
    # Online effects should be run only when item has running "online" effect
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_online_effect_id = client.mk_eve_online_effect()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.online, mod_info=[eve_mod])
    eve_item_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 20, eve_affectee_attr_id: 100},
        eff_ids=[eve_online_effect_id, eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_item_id, state=consts.ApiModuleState.offline)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(100)
    assert api_item.effects[eve_online_effect_id].running is False
    assert api_item.effects[eve_online_effect_id].mode == consts.ApiEffMode.full_compliance
    assert api_item.effects[eve_effect_id].running is False
    assert api_item.effects[eve_effect_id].mode == consts.ApiEffMode.full_compliance
    # Action
    api_item.change_mod(state=consts.ApiModuleState.online)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(120)
    assert api_item.effects[eve_online_effect_id].running is True
    assert api_item.effects[eve_online_effect_id].mode == consts.ApiEffMode.full_compliance
    assert api_item.effects[eve_effect_id].running is True
    assert api_item.effects[eve_effect_id].mode == consts.ApiEffMode.full_compliance
    # Action
    api_item.change_mod(effect_modes={eve_online_effect_id: consts.ApiEffMode.force_stop})
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(100)
    assert api_item.effects[eve_online_effect_id].running is False
    assert api_item.effects[eve_online_effect_id].mode == consts.ApiEffMode.force_stop
    assert api_item.effects[eve_effect_id].running is False
    assert api_item.effects[eve_effect_id].mode == consts.ApiEffMode.full_compliance
    # Action - effects from online category rely only on actual "online" effect, ignoring everything
    # else
    api_item.change_mod(
        state=consts.ApiModuleState.offline,
        effect_modes={eve_online_effect_id: consts.ApiEffMode.force_run})
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(120)
    assert api_item.effects[eve_online_effect_id].running is True
    assert api_item.effects[eve_online_effect_id].mode == consts.ApiEffMode.force_run
    assert api_item.effects[eve_effect_id].running is True
    assert api_item.effects[eve_effect_id].mode == consts.ApiEffMode.full_compliance


def test_state_online_absent(client, consts):
    # No online effect - other effects from online category are not running
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.online, mod_info=[eve_mod])
    eve_item_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 20, eve_affectee_attr_id: 100},
        eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_item_id, state=consts.ApiModuleState.online)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(100)
    assert api_item.effects[eve_effect_id].running is False
    assert api_item.effects[eve_effect_id].mode == consts.ApiEffMode.full_compliance
    # Action
    api_item.change_mod(state=consts.ApiModuleState.overload)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(100)
    assert api_item.effects[eve_effect_id].running is False
    assert api_item.effects[eve_effect_id].mode == consts.ApiEffMode.full_compliance


def test_state_active_default(client, consts):
    # Default active effect is run, non-default is not
    eve_affector_attr1_id = client.mk_eve_attr()
    eve_affector_attr2_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr1_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect1_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, mod_info=[eve_mod1])
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr2_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect2_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, mod_info=[eve_mod2])
    eve_item_id = client.mk_eve_item(
        attrs={eve_affector_attr1_id: 20, eve_affector_attr2_id: 30, eve_affectee_attr_id: 100},
        eff_ids=[eve_effect1_id, eve_effect2_id],
        defeff_id=eve_effect1_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_item_id, state=consts.ApiModuleState.online)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(100)
    assert api_item.effects[eve_effect1_id].running is False
    assert api_item.effects[eve_effect1_id].mode == consts.ApiEffMode.full_compliance
    assert api_item.effects[eve_effect2_id].running is False
    assert api_item.effects[eve_effect2_id].mode == consts.ApiEffMode.full_compliance
    # Action
    api_item.change_mod(state=consts.ApiModuleState.active)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(120)
    assert api_item.effects[eve_effect1_id].running is True
    assert api_item.effects[eve_effect1_id].mode == consts.ApiEffMode.full_compliance
    assert api_item.effects[eve_effect2_id].running is False
    # Action
    api_item.change_mod(state=consts.ApiModuleState.overload)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(120)
    assert api_item.effects[eve_effect1_id].running is True
    assert api_item.effects[eve_effect1_id].mode == consts.ApiEffMode.full_compliance
    assert api_item.effects[eve_effect2_id].running is False
    assert api_item.effects[eve_effect2_id].mode == consts.ApiEffMode.full_compliance


def test_state_active_absent(client, consts):
    # No default - nothing is running
    eve_affector_attr1_id = client.mk_eve_attr()
    eve_affector_attr2_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr1_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect1_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, mod_info=[eve_mod1])
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr2_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect2_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, mod_info=[eve_mod2])
    eve_item_id = client.mk_eve_item(
        attrs={eve_affector_attr1_id: 20, eve_affector_attr2_id: 30, eve_affectee_attr_id: 100},
        eff_ids=[eve_effect1_id, eve_effect2_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_item_id, state=consts.ApiModuleState.online)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(100)
    assert api_item.effects[eve_effect1_id].running is False
    assert api_item.effects[eve_effect1_id].mode == consts.ApiEffMode.full_compliance
    assert api_item.effects[eve_effect2_id].running is False
    assert api_item.effects[eve_effect2_id].mode == consts.ApiEffMode.full_compliance
    # Action
    api_item.change_mod(state=consts.ApiModuleState.active)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(100)
    assert api_item.effects[eve_effect1_id].running is False
    assert api_item.effects[eve_effect1_id].mode == consts.ApiEffMode.full_compliance
    assert api_item.effects[eve_effect2_id].running is False
    assert api_item.effects[eve_effect2_id].mode == consts.ApiEffMode.full_compliance
    # Action
    api_item.change_mod(state=consts.ApiModuleState.overload)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(100)
    assert api_item.effects[eve_effect1_id].running is False
    assert api_item.effects[eve_effect1_id].mode == consts.ApiEffMode.full_compliance
    assert api_item.effects[eve_effect2_id].running is False
    assert api_item.effects[eve_effect2_id].mode == consts.ApiEffMode.full_compliance


def test_state_overload(client, consts):
    # Overload just needs item to be overloaded and nothing else
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.overload, mod_info=[eve_mod])
    eve_item_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 20, eve_affectee_attr_id: 100},
        eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_item_id, state=consts.ApiModuleState.online)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(100)
    assert api_item.effects[eve_effect_id].running is False
    assert api_item.effects[eve_effect_id].mode == consts.ApiEffMode.full_compliance
    # Action
    api_item.change_mod(state=consts.ApiModuleState.overload)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(120)
    assert api_item.effects[eve_effect_id].running is True
    assert api_item.effects[eve_effect_id].mode == consts.ApiEffMode.full_compliance
