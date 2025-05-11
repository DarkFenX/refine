from tests import check_no_field, effect_dogma_to_api
from tests.fw.api import ValOptions


def test_project_unproject(client, consts):
    # Also test that only validation of target fit is affected
    eve_tgt_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active)
    eve_src_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.stopper,
        loc=consts.EveModLoc.tgt_stopper,
        effect_id=eve_tgt_effect_id)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_src_mod])
    eve_src_item_id = client.mk_eve_item(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_tgt_item_id = client.mk_eve_item(eff_ids=[eve_tgt_effect_id], defeff_id=eve_tgt_effect_id)
    client.create_sources()
    api_tgt_effect_id = effect_dogma_to_api(dogma_effect_id=eve_tgt_effect_id)
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_item = api_src_fit.add_module(type_id=eve_src_item_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.add_fighter(type_id=eve_tgt_item_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_val = api_tgt_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_src_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_src_item.change_module(add_projs=[api_tgt_item.id])
    # Verification
    api_val = api_tgt_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is False
    assert api_val.details.effect_stopper == {api_tgt_item.id: [api_tgt_effect_id]}
    api_val = api_src_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_src_item.change_module(rm_projs=[api_tgt_item.id])
    # Verification
    api_val = api_tgt_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_src_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_multiple_src_items(client, consts):
    eve_tgt_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active)
    eve_src_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.stopper,
        loc=consts.EveModLoc.tgt_stopper,
        effect_id=eve_tgt_effect_id)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_src_mod])
    eve_src_item_id = client.mk_eve_item(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_tgt_item_id = client.mk_eve_item(eff_ids=[eve_tgt_effect_id], defeff_id=eve_tgt_effect_id)
    client.create_sources()
    api_tgt_effect_id = effect_dogma_to_api(dogma_effect_id=eve_tgt_effect_id)
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_item1 = api_src_fit.add_module(type_id=eve_src_item_id, state=consts.ApiModuleState.active)
    api_src_item2 = api_src_fit.add_module(type_id=eve_src_item_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.add_fighter(type_id=eve_tgt_item_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_val = api_tgt_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_src_item1.change_module(add_projs=[api_tgt_item.id])
    # Verification
    api_val = api_tgt_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is False
    assert api_val.details.effect_stopper == {api_tgt_item.id: [api_tgt_effect_id]}
    # Action
    api_src_item2.change_module(add_projs=[api_tgt_item.id])
    # Verification
    api_val = api_tgt_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is False
    assert api_val.details.effect_stopper == {api_tgt_item.id: [api_tgt_effect_id]}
    # Action
    api_src_item1.change_module(rm_projs=[api_tgt_item.id])
    # Verification
    api_val = api_tgt_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is False
    assert api_val.details.effect_stopper == {api_tgt_item.id: [api_tgt_effect_id]}
    # Action
    api_src_item2.change_module(rm_projs=[api_tgt_item.id])
    # Verification
    api_val = api_tgt_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_multiple_src_effects(client, consts):
    eve_tgt_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active)
    eve_src_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.stopper,
        loc=consts.EveModLoc.tgt_stopper,
        effect_id=eve_tgt_effect_id)
    eve_src_effect1_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_src_mod])
    eve_src_effect2_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_src_mod])
    eve_src_item_id = client.mk_eve_item(eff_ids=[eve_src_effect1_id, eve_src_effect2_id], defeff_id=eve_src_effect1_id)
    eve_tgt_item_id = client.mk_eve_item(eff_ids=[eve_tgt_effect_id], defeff_id=eve_tgt_effect_id)
    client.create_sources()
    api_src_effect1_id = effect_dogma_to_api(dogma_effect_id=eve_src_effect1_id)
    api_src_effect2_id = effect_dogma_to_api(dogma_effect_id=eve_src_effect2_id)
    api_tgt_effect_id = effect_dogma_to_api(dogma_effect_id=eve_tgt_effect_id)
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_item = api_src_fit.add_module(type_id=eve_src_item_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.add_fighter(type_id=eve_tgt_item_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_val = api_tgt_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_src_item.change_module(add_projs=[api_tgt_item.id])
    # Verification
    api_val = api_tgt_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is False
    assert api_val.details.effect_stopper == {api_tgt_item.id: [api_tgt_effect_id]}
    # Action
    api_src_item.change_module(effect_modes={api_src_effect2_id: consts.ApiEffMode.state_compliance})
    # Verification
    api_val = api_tgt_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is False
    assert api_val.details.effect_stopper == {api_tgt_item.id: [api_tgt_effect_id]}
    # Action
    api_src_item.change_module(effect_modes={api_src_effect1_id: consts.ApiEffMode.force_stop})
    # Verification
    api_val = api_tgt_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is False
    assert api_val.details.effect_stopper == {api_tgt_item.id: [api_tgt_effect_id]}
    # Action
    api_src_item.change_module(effect_modes={api_src_effect2_id: consts.ApiEffMode.force_stop})
    # Verification
    api_val = api_tgt_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
