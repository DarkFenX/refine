from tests import Effect, Muta, approx, check_no_field
from tests.fw.api import ValOptions


def test_offense_src_module_tgt_ship_project_unproject(client, consts):
    # Also test that only validation of source fit is affected
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_offensive_modifiers)
    eve_effect_attr_id = client.mk_eve_attr()
    eve_src_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_effect_attr_id,
        affectee_attr_id=eve_effect_attr_id)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, is_offensive=True, mod_info=[eve_src_mod])
    eve_src_item_id = client.mk_eve_item(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_tgt_item_id = client.mk_eve_ship(attrs={eve_immunity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_item = api_src_fit.add_module(type_id=eve_src_item_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item_id)
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_tgt_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_src_item.change_module(add_projs=[api_tgt_item.id])
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is False
    assert api_val.details.offense_immunity == {api_src_item.id: [api_tgt_item.id]}
    api_val = api_tgt_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_src_item.change_module(rm_projs=[api_tgt_item.id])
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_tgt_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_offense_src_drone_fighter(client, consts):
    # Also check multiple projector items
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_offensive_modifiers)
    eve_effect_attr_id = client.mk_eve_attr()
    eve_src_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_effect_attr_id,
        affectee_attr_id=eve_effect_attr_id)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, is_offensive=True, mod_info=[eve_src_mod])
    eve_src_item_id = client.mk_eve_item(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_tgt_item_id = client.mk_eve_ship(attrs={eve_immunity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_drone = api_src_fit.add_drone(type_id=eve_src_item_id, state=consts.ApiMinionState.engaging)
    api_src_fighter = api_src_fit.add_fighter(type_id=eve_src_item_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item_id)
    api_src_drone.change_drone(add_projs=[api_tgt_item.id])
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is False
    assert api_val.details.offense_immunity == {api_src_drone.id: [api_tgt_item.id]}
    # Action
    api_src_fighter.change_fighter(add_projs=[api_tgt_item.id])
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is False
    assert api_val.details.offense_immunity == {
        api_src_drone.id: [api_tgt_item.id],
        api_src_fighter.id: [api_tgt_item.id]}
    # Action
    api_src_drone.remove()
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is False
    assert api_val.details.offense_immunity == {api_src_fighter.id: [api_tgt_item.id]}
    # Action
    api_src_fighter.remove()
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_offense_src_proj_effect(client, consts):
    # Projected effects do not apply targeted modifications, and this validation applies only to
    # targeted effects
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_offensive_modifiers)
    eve_effect_attr_id = client.mk_eve_attr()
    eve_src_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_effect_attr_id,
        affectee_attr_id=eve_effect_attr_id)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, is_offensive=True, mod_info=[eve_src_mod])
    eve_src_item_id = client.mk_eve_item(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_tgt_item_id = client.mk_eve_ship(attrs={eve_immunity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_proj_effect = api_sol.add_proj_effect(type_id=eve_src_item_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item_id)
    api_src_proj_effect.change_proj_effect(add_projs=[api_tgt_item.id])
    # Verification
    api_val = api_sol.validate(fit_ids=[api_src_fit.id], options=ValOptions(offense_immunity=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_offense_tgt_drone_fighter(client, consts):
    # Also check multiple projectee items
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_offensive_modifiers)
    eve_effect_attr_id = client.mk_eve_attr()
    eve_src_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_effect_attr_id,
        affectee_attr_id=eve_effect_attr_id)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, is_offensive=True, mod_info=[eve_src_mod])
    eve_src_item_id = client.mk_eve_item(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_tgt_item_id = client.mk_eve_item(attrs={eve_immunity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_item = api_src_fit.add_module(type_id=eve_src_item_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_drone = api_tgt_fit.add_drone(type_id=eve_tgt_item_id)
    api_tgt_fighter = api_tgt_fit.add_fighter(type_id=eve_tgt_item_id)
    api_src_item.change_module(add_projs=[api_tgt_drone.id, api_tgt_fighter.id])
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is False
    assert api_val.details.offense_immunity == {api_src_item.id: sorted([api_tgt_drone.id, api_tgt_fighter.id])}


def test_offense_multiple_src_effects(client, consts):
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_offensive_modifiers)
    eve_effect_attr_id = client.mk_eve_attr()
    eve_src_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_effect_attr_id,
        affectee_attr_id=eve_effect_attr_id)
    eve_src_effect1_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, is_offensive=True, mod_info=[eve_src_mod])
    eve_src_effect2_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, is_offensive=True, mod_info=[eve_src_mod])
    eve_src_item_id = client.mk_eve_item(eff_ids=[eve_src_effect1_id, eve_src_effect2_id], defeff_id=eve_src_effect1_id)
    eve_tgt_item_id = client.mk_eve_ship(attrs={eve_immunity_attr_id: 1})
    client.create_sources()
    api_src_effect1_id = Effect.dogma_to_api(dogma_effect_id=eve_src_effect1_id)
    api_src_effect2_id = Effect.dogma_to_api(dogma_effect_id=eve_src_effect2_id)
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_item = api_src_fit.add_module(type_id=eve_src_item_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item_id)
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_src_item.change_module(add_projs=[api_tgt_item.id])
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is False
    assert api_val.details.offense_immunity == {api_src_item.id: [api_tgt_item.id]}
    # Action
    api_src_item.change_module(effect_modes={api_src_effect2_id: consts.ApiEffMode.state_compliance})
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is False
    assert api_val.details.offense_immunity == {api_src_item.id: [api_tgt_item.id]}
    # Action
    api_src_item.change_module(effect_modes={api_src_effect1_id: consts.ApiEffMode.force_stop})
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is False
    assert api_val.details.offense_immunity == {api_src_item.id: [api_tgt_item.id]}
    # Action
    api_src_item.change_module(effect_modes={api_src_effect2_id: consts.ApiEffMode.force_stop})
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_offense_buff(client, consts):
    # Use disrupt lance to apply offensive debuff
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_offensive_modifiers)
    eve_affectee_attr_id = client.mk_eve_attr(stackable=True)
    client.mk_eve_buff(
        id_=consts.EveBuff.remote_repair_impedance,
        aggr_mode=consts.EveBuffAggrMode.min,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.debuff_lance,
        cat_id=consts.EveEffCat.active,
        is_offensive=True)
    eve_affector_module_id = client.mk_eve_item(eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_affectee_ship_id = client.mk_eve_ship(attrs={eve_immunity_attr_id: 1, eve_affectee_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_affectee_ship_id)
    api_src_fit = api_sol.create_fit()
    api_src_module = api_src_fit.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_src_module.change_module(add_projs=[api_tgt_ship.id])
    # Verification - check that effect is applied, and check that validation is passed
    api_tgt_ship.update()
    assert api_tgt_ship.attrs[eve_affectee_attr_id].dogma == approx(0.5)
    assert api_tgt_ship.attrs[eve_immunity_attr_id].dogma == approx(1)
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_offense_flag_values(client, consts):
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_offensive_modifiers)
    eve_effect_attr_id = client.mk_eve_attr()
    eve_src_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_effect_attr_id,
        affectee_attr_id=eve_effect_attr_id)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, is_offensive=True, mod_info=[eve_src_mod])
    eve_src_item_id = client.mk_eve_item(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_tgt_item1_id = client.mk_eve_item(attrs={eve_immunity_attr_id: -1})
    eve_tgt_item2_id = client.mk_eve_item(attrs={eve_immunity_attr_id: 0})
    eve_tgt_item3_id = client.mk_eve_item(attrs={eve_immunity_attr_id: 0.1})
    eve_tgt_item4_id = client.mk_eve_item(attrs={eve_immunity_attr_id: 50.3})
    eve_tgt_item5_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_item = api_src_fit.add_module(type_id=eve_src_item_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item1 = api_tgt_fit.add_drone(type_id=eve_tgt_item1_id)
    api_tgt_item2 = api_tgt_fit.add_drone(type_id=eve_tgt_item2_id)
    api_tgt_item3 = api_tgt_fit.add_drone(type_id=eve_tgt_item3_id)
    api_tgt_item4 = api_tgt_fit.add_drone(type_id=eve_tgt_item4_id)
    api_tgt_item5 = api_tgt_fit.add_drone(type_id=eve_tgt_item5_id)
    api_src_item.change_module(
        add_projs=[api_tgt_item1.id, api_tgt_item2.id, api_tgt_item3.id, api_tgt_item4.id, api_tgt_item5.id])
    # Verification - no attr or value 0 doesn't fail validation
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is False
    assert api_val.details.offense_immunity == {api_src_item.id: [api_tgt_item1.id, api_tgt_item3.id, api_tgt_item4.id]}


def test_offense_tgt_modified(client, consts):
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_offensive_modifiers)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_effect_attr_id = client.mk_eve_attr()
    eve_src_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_effect_attr_id,
        affectee_attr_id=eve_effect_attr_id)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, is_offensive=True, mod_info=[eve_src_mod])
    eve_src_item_id = client.mk_eve_item(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_tgt_item_id = client.mk_eve_ship(attrs={eve_immunity_attr_id: 0})
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_immunity_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_mod_item_id = client.mk_eve_item(attrs={eve_mod_attr_id: 1}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_item = api_src_fit.add_module(type_id=eve_src_item_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item_id)
    api_src_item.change_module(add_projs=[api_tgt_item.id])
    # Verification
    assert api_tgt_item.update().attrs[eve_immunity_attr_id].extra == approx(0)
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_mod_item = api_tgt_fit.add_module(type_id=eve_mod_item_id)
    # Verification
    assert api_tgt_item.update().attrs[eve_immunity_attr_id].extra == approx(1)
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is False
    assert api_val.details.offense_immunity == {api_src_item.id: [api_tgt_item.id]}
    # Action
    api_mod_item.remove()
    # Verification
    assert api_tgt_item.update().attrs[eve_immunity_attr_id].extra == approx(0)
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_offense_tgt_mutation(client, consts):
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_offensive_modifiers)
    eve_effect_attr_id = client.mk_eve_attr()
    eve_src_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_effect_attr_id,
        affectee_attr_id=eve_effect_attr_id)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, is_offensive=True, mod_info=[eve_src_mod])
    eve_src_item_id = client.mk_eve_item(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_tgt_base_item_id = client.mk_eve_item(attrs={eve_immunity_attr_id: 0})
    eve_tgt_mutated_item_id = client.mk_eve_item(attrs={eve_immunity_attr_id: 1})
    eve_tgt_mutator_id = client.mk_eve_mutator(
        items=[([eve_tgt_base_item_id], eve_tgt_mutated_item_id)],
        attrs={eve_immunity_attr_id: (0, 2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_item = api_src_fit.add_module(type_id=eve_src_item_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.add_drone(type_id=eve_tgt_base_item_id)
    api_src_item.change_module(add_projs=[api_tgt_item.id])
    # Verification
    assert api_tgt_item.update().attrs[eve_immunity_attr_id].extra == approx(0)
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_tgt_item.change_drone(mutation=eve_tgt_mutator_id)
    # Verification
    assert api_tgt_item.update().attrs[eve_immunity_attr_id].extra == approx(1)
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is False
    assert api_val.details.offense_immunity == {api_src_item.id: [api_tgt_item.id]}
    # Action
    api_tgt_item.change_drone(mutation={eve_immunity_attr_id: Muta.roll_to_api(val=0)})
    # Verification
    assert api_tgt_item.update().attrs[eve_immunity_attr_id].extra == approx(0)
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_tgt_item.change_drone(mutation={eve_immunity_attr_id: Muta.roll_to_api(val=0.1)})
    # Verification
    assert api_tgt_item.update().attrs[eve_immunity_attr_id].extra == approx(0.2)
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is False
    assert api_val.details.offense_immunity == {api_src_item.id: [api_tgt_item.id]}
    # Action
    api_tgt_item.change_drone(mutation=None)
    # Verification
    assert api_tgt_item.update().attrs[eve_immunity_attr_id].extra == approx(0)
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_offense_src_mutation(client, consts):
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_offensive_modifiers)
    eve_effect_attr_id = client.mk_eve_attr()
    eve_src_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_effect_attr_id,
        affectee_attr_id=eve_effect_attr_id)
    eve_src_base_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        is_offensive=True)
    eve_src_mutated_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        is_offensive=True,
        mod_info=[eve_src_mod])
    eve_src_base_item_id = client.mk_eve_item(
        eff_ids=[eve_src_base_effect_id],
        defeff_id=eve_src_base_effect_id)
    eve_src_mutated_item_id = client.mk_eve_item(
        eff_ids=[eve_src_mutated_effect_id],
        defeff_id=eve_src_mutated_effect_id)
    eve_src_mutator_id = client.mk_eve_mutator(items=[([eve_src_base_item_id], eve_src_mutated_item_id)])
    eve_tgt_item_id = client.mk_eve_ship(attrs={eve_immunity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_item = api_src_fit.add_module(type_id=eve_src_base_item_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item_id)
    api_src_item.change_module(add_projs=[api_tgt_item.id])
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_src_item.change_module(mutation=eve_src_mutator_id)
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is False
    assert api_val.details.offense_immunity == {api_src_item.id: [api_tgt_item.id]}
    # Action
    api_src_item.change_module(mutation=None)
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_offense_criterion_no_modifiers(client, consts):
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_offensive_modifiers)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, is_offensive=True)
    eve_src_item_id = client.mk_eve_item(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_tgt_item_id = client.mk_eve_ship(attrs={eve_immunity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_item = api_src_fit.add_module(type_id=eve_src_item_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item_id)
    api_src_item.change_module(add_projs=[api_tgt_item.id])
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_offense_criterion_not_offense(client, consts):
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_offensive_modifiers)
    eve_effect_attr_id = client.mk_eve_attr()
    eve_src_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_effect_attr_id,
        affectee_attr_id=eve_effect_attr_id)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, is_offensive=False, mod_info=[eve_src_mod])
    eve_src_item_id = client.mk_eve_item(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_tgt_item_id = client.mk_eve_ship(attrs={eve_immunity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_item = api_src_fit.add_module(type_id=eve_src_item_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item_id)
    api_src_item.change_module(add_projs=[api_tgt_item.id])
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_offense_criterion_effect_cat(client, consts):
    # Only targeted effects are subject for the validation
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_offensive_modifiers)
    eve_effect_attr_id = client.mk_eve_attr()
    eve_src_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_effect_attr_id,
        affectee_attr_id=eve_effect_attr_id)
    eve_src_effect_id = client.mk_eve_effect(
        # AoE web just to enforce effect have some buffs, to let it go through generic projection
        # filters and reach validation service
        id_=consts.EveEffect.doomsday_aoe_web,
        cat_id=consts.EveEffCat.active,
        is_offensive=True,
        mod_info=[eve_src_mod])
    eve_src_item_id = client.mk_eve_item(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_tgt_item_id = client.mk_eve_ship(attrs={eve_immunity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_item = api_src_fit.add_module(type_id=eve_src_item_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item_id)
    api_src_item.change_module(add_projs=[api_tgt_item.id])
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_assist(client, consts):
    # Check different disallow-vs-ewar-immune flag values here too
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_offensive_modifiers)
    eve_assist_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_vs_ew_immune_tgt)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, is_assistance=True, is_offensive=False)
    eve_src_item1_id = client.mk_eve_item(
        attrs={eve_assist_attr_id: -1},
        eff_ids=[eve_src_effect_id],
        defeff_id=eve_src_effect_id)
    eve_src_item2_id = client.mk_eve_item(
        attrs={eve_assist_attr_id: 0},
        eff_ids=[eve_src_effect_id],
        defeff_id=eve_src_effect_id)
    eve_src_item3_id = client.mk_eve_item(
        attrs={eve_assist_attr_id: 0.1},
        eff_ids=[eve_src_effect_id],
        defeff_id=eve_src_effect_id)
    eve_src_item4_id = client.mk_eve_item(
        attrs={eve_assist_attr_id: 50.3},
        eff_ids=[eve_src_effect_id],
        defeff_id=eve_src_effect_id)
    eve_src_item5_id = client.mk_eve_item(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_tgt_item_id = client.mk_eve_ship(attrs={eve_immunity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_item1 = api_src_fit.add_module(type_id=eve_src_item1_id, state=consts.ApiModuleState.active)
    api_src_item2 = api_src_fit.add_module(type_id=eve_src_item2_id, state=consts.ApiModuleState.active)
    api_src_item3 = api_src_fit.add_module(type_id=eve_src_item3_id, state=consts.ApiModuleState.active)
    api_src_item4 = api_src_fit.add_module(type_id=eve_src_item4_id, state=consts.ApiModuleState.active)
    api_src_item5 = api_src_fit.add_module(type_id=eve_src_item5_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item_id)
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_src_item1.change_module(add_projs=[api_tgt_item.id])
    api_src_item2.change_module(add_projs=[api_tgt_item.id])
    api_src_item3.change_module(add_projs=[api_tgt_item.id])
    api_src_item4.change_module(add_projs=[api_tgt_item.id])
    api_src_item5.change_module(add_projs=[api_tgt_item.id])
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is False
    assert api_val.details.offense_immunity == {
        api_src_item1.id: [api_tgt_item.id],
        api_src_item3.id: [api_tgt_item.id],
        api_src_item4.id: [api_tgt_item.id]}


def test_assist_switch_src_type_id(client, consts):
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_offensive_modifiers)
    eve_assist_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_vs_ew_immune_tgt)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, is_assistance=True, is_offensive=False)
    eve_src_item1_id = client.mk_eve_item(
        attrs={eve_assist_attr_id: 1},
        eff_ids=[eve_src_effect_id],
        defeff_id=eve_src_effect_id)
    eve_src_item2_id = client.mk_eve_item(
        attrs={eve_assist_attr_id: 0},
        eff_ids=[eve_src_effect_id],
        defeff_id=eve_src_effect_id)
    eve_tgt_item_id = client.mk_eve_ship(attrs={eve_immunity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_item = api_src_fit.add_module(type_id=eve_src_item1_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item_id)
    api_src_item.change_module(add_projs=[api_tgt_item.id])
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is False
    assert api_val.details.offense_immunity == {api_src_item.id: [api_tgt_item.id]}
    # Action
    api_src_item.change_module(type_id=eve_src_item2_id)
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_src_item.change_module(type_id=eve_src_item1_id)
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is False
    assert api_val.details.offense_immunity == {api_src_item.id: [api_tgt_item.id]}


def test_assist_src_mutation_add(client, consts):
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_offensive_modifiers)
    eve_assist_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_vs_ew_immune_tgt)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, is_assistance=True, is_offensive=False)
    eve_src_base_item_id = client.mk_eve_item(
        attrs={eve_assist_attr_id: 0},
        eff_ids=[eve_src_effect_id],
        defeff_id=eve_src_effect_id)
    eve_src_mutated_item_id = client.mk_eve_item(
        attrs={eve_assist_attr_id: 1},
        eff_ids=[eve_src_effect_id],
        defeff_id=eve_src_effect_id)
    eve_src_mutator_id = client.mk_eve_mutator(
        items=[([eve_src_base_item_id], eve_src_mutated_item_id)],
        attrs={eve_assist_attr_id: (0, 2)})
    eve_tgt_item_id = client.mk_eve_ship(attrs={eve_immunity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_item = api_src_fit.add_module(
        type_id=eve_src_base_item_id,
        state=consts.ApiModuleState.active,
        mutation=(eve_src_mutator_id, {eve_assist_attr_id: Muta.roll_to_api(val=0)}))
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item_id)
    api_src_item.change_module(add_projs=[api_tgt_item.id])
    # Verification - validation fails, since unmutated value is used to determine behavior
    assert api_src_item.update().attrs[eve_assist_attr_id].extra == approx(0)
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is False
    assert api_val.details.offense_immunity == {api_src_item.id: [api_tgt_item.id]}


def test_assist_src_mutation_change(client, consts):
    # Mutated value is ignored, only unmutated/unmodified item value is considered
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_offensive_modifiers)
    eve_assist_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_vs_ew_immune_tgt)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, is_assistance=True, is_offensive=False)
    eve_src_base_item_id = client.mk_eve_item(
        attrs={eve_assist_attr_id: 0},
        eff_ids=[eve_src_effect_id],
        defeff_id=eve_src_effect_id)
    eve_src_mutated_item_id = client.mk_eve_item(
        attrs={eve_assist_attr_id: 1},
        eff_ids=[eve_src_effect_id],
        defeff_id=eve_src_effect_id)
    eve_src_mutator_id = client.mk_eve_mutator(
        items=[([eve_src_base_item_id], eve_src_mutated_item_id)],
        attrs={eve_assist_attr_id: (0, 2)})
    eve_tgt_item_id = client.mk_eve_ship(attrs={eve_immunity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_item = api_src_fit.add_module(type_id=eve_src_base_item_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item_id)
    api_src_item.change_module(add_projs=[api_tgt_item.id])
    # Verification
    assert api_src_item.update().attrs[eve_assist_attr_id].extra == approx(0)
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_src_item.change_module(mutation=(eve_src_mutator_id, {eve_assist_attr_id: Muta.roll_to_api(val=0)}))
    # Verification
    assert api_src_item.update().attrs[eve_assist_attr_id].extra == approx(0)
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is False
    assert api_val.details.offense_immunity == {api_src_item.id: [api_tgt_item.id]}
    # Action
    api_src_item.change_module(mutation=(eve_src_mutator_id, {eve_assist_attr_id: Muta.roll_to_api(val=0.5)}))
    # Verification
    assert api_src_item.update().attrs[eve_assist_attr_id].extra == approx(1)
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is False
    assert api_val.details.offense_immunity == {api_src_item.id: [api_tgt_item.id]}
    # Action
    api_src_item.change_module(mutation=None)
    # Verification
    assert api_src_item.update().attrs[eve_assist_attr_id].extra == approx(0)
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_known_failures(client, consts):
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_offensive_modifiers)
    eve_assist_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_vs_ew_immune_tgt)
    eve_effect_attr_id = client.mk_eve_attr()
    eve_src_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_effect_attr_id,
        affectee_attr_id=eve_effect_attr_id)
    eve_src_offense_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        is_offensive=True,
        mod_info=[eve_src_mod])
    eve_src_assist_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        is_assistance=True,
        is_offensive=False)
    eve_src_offense_item_id = client.mk_eve_item(
        eff_ids=[eve_src_offense_effect_id],
        defeff_id=eve_src_offense_effect_id)
    eve_src_assist_item_id = client.mk_eve_item(
        attrs={eve_assist_attr_id: 1},
        eff_ids=[eve_src_assist_effect_id],
        defeff_id=eve_src_assist_effect_id)
    eve_tgt_item_id = client.mk_eve_ship(attrs={eve_immunity_attr_id: 1})
    eve_other_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_other = api_src_fit.add_implant(type_id=eve_other_id)
    api_src_offense = api_src_fit.add_module(type_id=eve_src_offense_item_id, state=consts.ApiModuleState.active)
    api_src_assist = api_src_fit.add_module(type_id=eve_src_assist_item_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item_id)
    api_src_offense.change_module(add_projs=[api_tgt_item.id])
    api_src_assist.change_module(add_projs=[api_tgt_item.id])
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=(True, [api_src_offense.id])))
    assert api_val.passed is False
    assert api_val.details.offense_immunity == {api_src_assist.id: [api_tgt_item.id]}
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=(True, [api_src_assist.id])))
    assert api_val.passed is False
    assert api_val.details.offense_immunity == {api_src_offense.id: [api_tgt_item.id]}
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=(True, [api_src_offense.id, api_src_assist.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_src_fit.validate(options=ValOptions(
        offense_immunity=(True, [api_src_offense.id, api_src_other.id, api_src_assist.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_assist_immunity(client, consts):
    # Check that target flagged as assist immune doesn't trigger this validation, for both offensive
    # effects with modifiers, and assistive effects which shouldn't be used against offense immune
    # targets
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_assistance)
    eve_assist_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_vs_ew_immune_tgt)
    eve_effect_attr_id = client.mk_eve_attr()
    eve_src_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_effect_attr_id,
        affectee_attr_id=eve_effect_attr_id)
    eve_src_offense_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        is_offensive=True,
        mod_info=[eve_src_mod])
    eve_src_assist_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        is_assistance=True,
        is_offensive=False)
    eve_src_offense_item_id = client.mk_eve_item(
        eff_ids=[eve_src_offense_effect_id],
        defeff_id=eve_src_offense_effect_id)
    eve_src_assist_item_id = client.mk_eve_item(
        attrs={eve_assist_attr_id: 1},
        eff_ids=[eve_src_assist_effect_id],
        defeff_id=eve_src_assist_effect_id)
    eve_tgt_item_id = client.mk_eve_ship(attrs={eve_immunity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_offense = api_src_fit.add_module(type_id=eve_src_offense_item_id, state=consts.ApiModuleState.active)
    api_src_assist = api_src_fit.add_module(type_id=eve_src_assist_item_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item_id)
    api_src_offense.change_module(add_projs=[api_tgt_item.id])
    api_src_assist.change_module(add_projs=[api_tgt_item.id])
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_not_loaded_src(client, consts):
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_offensive_modifiers)
    eve_src_item_id = client.alloc_item_id()
    eve_tgt_item_id = client.mk_eve_ship(attrs={eve_immunity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_item = api_src_fit.add_module(type_id=eve_src_item_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item_id)
    api_src_item.change_module(add_projs=[api_tgt_item.id])
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_not_loaded_tgt(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.disallow_offensive_modifiers)
    eve_assist_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_vs_ew_immune_tgt)
    eve_effect_attr_id = client.mk_eve_attr()
    eve_src_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_effect_attr_id,
        affectee_attr_id=eve_effect_attr_id)
    eve_src_offense_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        is_offensive=True,
        mod_info=[eve_src_mod])
    eve_src_assist_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        is_assistance=True,
        is_offensive=False)
    eve_src_offense_item_id = client.mk_eve_item(
        eff_ids=[eve_src_offense_effect_id],
        defeff_id=eve_src_offense_effect_id)
    eve_src_assist_item_id = client.mk_eve_item(
        attrs={eve_assist_attr_id: 1},
        eff_ids=[eve_src_assist_effect_id],
        defeff_id=eve_src_assist_effect_id)
    eve_tgt_item_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_offense = api_src_fit.add_module(type_id=eve_src_offense_item_id, state=consts.ApiModuleState.active)
    api_src_assist = api_src_fit.add_module(type_id=eve_src_assist_item_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item_id)
    api_src_offense.change_module(add_projs=[api_tgt_item.id])
    api_src_assist.change_module(add_projs=[api_tgt_item.id])
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(offense_immunity=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
