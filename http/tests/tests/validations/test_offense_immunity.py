from tests import check_no_field, effect_dogma_to_api
from tests.fw.api import ValOptions


def test_projector_module_project_unproject(client, consts):
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


def test_projector_drone_fighter(client, consts):
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


def test_multiple_src_effects(client, consts):
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
    api_src_effect1_id = effect_dogma_to_api(dogma_effect_id=eve_src_effect1_id)
    api_src_effect2_id = effect_dogma_to_api(dogma_effect_id=eve_src_effect2_id)
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
