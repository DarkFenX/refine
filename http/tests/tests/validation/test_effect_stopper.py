from tests import check_no_field, effect_dogma_to_api, range_s2s_to_api
from tests.fw.api import ValOptions


def test_module_project_unproject(client, consts):
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


def test_drone_fighter(client, consts):
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
    api_src_item1 = api_src_fit.add_drone(type_id=eve_src_item_id, state=consts.ApiMinionState.engaging)
    api_src_item2 = api_src_fit.add_fighter(type_id=eve_src_item_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.add_fighter(type_id=eve_tgt_item_id, state=consts.ApiMinionState.engaging)
    api_src_item1.change_drone(add_projs=[api_tgt_item.id])
    # Verification
    api_val = api_tgt_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is False
    assert api_val.details.effect_stopper == {api_tgt_item.id: [api_tgt_effect_id]}
    # Action
    api_src_item1.remove()
    # Verification
    api_val = api_tgt_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_src_item2.change_fighter(add_projs=[api_tgt_item.id])
    # Verification
    api_val = api_tgt_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is False
    assert api_val.details.effect_stopper == {api_tgt_item.id: [api_tgt_effect_id]}


def test_proj_effect(client, consts):
    # Doesn't work for projected effects
    eve_tgt_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active)
    eve_src_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.stopper,
        loc=consts.EveModLoc.tgt_stopper,
        effect_id=eve_tgt_effect_id)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_src_mod])
    eve_src_item_id = client.mk_eve_item(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_tgt_item_id = client.mk_eve_item(eff_ids=[eve_tgt_effect_id], defeff_id=eve_tgt_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_item = api_sol.add_proj_effect(type_id=eve_src_item_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.add_fighter(type_id=eve_tgt_item_id, state=consts.ApiMinionState.engaging)
    api_src_item.change_proj_effect(add_projs=[api_tgt_item.id])
    # Verification
    api_val = api_tgt_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_range(client, consts):
    # Check that effect stoppers follow dogma assign operation treatment
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_tgt_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active)
    eve_src_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.stopper,
        loc=consts.EveModLoc.tgt_stopper,
        effect_id=eve_tgt_effect_id)
    eve_src_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.tgt_normal1,
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id,
        mod_info=[eve_src_mod])
    eve_src_item_id = client.mk_eve_item(
        attrs={eve_optimal_attr_id: 1000, eve_falloff_attr_id: 500},
        eff_ids=[eve_src_effect_id],
        defeff_id=eve_src_effect_id)
    eve_tgt_item_id = client.mk_eve_item(eff_ids=[eve_tgt_effect_id], defeff_id=eve_tgt_effect_id)
    client.create_sources()
    api_tgt_effect_id = effect_dogma_to_api(dogma_effect_id=eve_tgt_effect_id)
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_item1 = api_src_fit.add_module(type_id=eve_src_item_id, state=consts.ApiModuleState.active)
    api_src_item2 = api_src_fit.add_module(type_id=eve_src_item_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.add_fighter(type_id=eve_tgt_item_id, state=consts.ApiMinionState.engaging)
    api_src_item1.change_module(add_projs=[api_tgt_item.id])
    api_src_item2.change_module(add_projs=[api_tgt_item.id])
    # Verification
    api_val = api_tgt_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is False
    assert api_val.details.effect_stopper == {api_tgt_item.id: [api_tgt_effect_id]}
    # Action
    api_src_item1.change_module(change_projs=[(api_tgt_item.id, range_s2s_to_api(val=1000))])
    api_src_item2.change_module(change_projs=[(api_tgt_item.id, range_s2s_to_api(val=1000))])
    # Verification
    api_val = api_tgt_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is False
    assert api_val.details.effect_stopper == {api_tgt_item.id: [api_tgt_effect_id]}
    # Action
    api_src_item1.change_module(change_projs=[(api_tgt_item.id, range_s2s_to_api(val=1500))])
    api_src_item2.change_module(change_projs=[(api_tgt_item.id, range_s2s_to_api(val=1500))])
    # Verification - still fails in falloff
    api_val = api_tgt_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is False
    assert api_val.details.effect_stopper == {api_tgt_item.id: [api_tgt_effect_id]}
    # Action
    api_src_item1.change_module(change_projs=[(api_tgt_item.id, range_s2s_to_api(val=300000))])
    # Verification - once range multiplier reaches 0, validation passes, but here only 1/2 have it
    api_val = api_tgt_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is False
    assert api_val.details.effect_stopper == {api_tgt_item.id: [api_tgt_effect_id]}
    # Action
    api_src_item2.change_module(change_projs=[(api_tgt_item.id, range_s2s_to_api(val=300000))])
    # Verification - both projectors have multipliers of 0
    api_val = api_tgt_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_src_item1.change_module(change_projs=[(api_tgt_item.id, range_s2s_to_api(val=1500))])
    # Verification - back to 1/2
    api_val = api_tgt_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is False
    assert api_val.details.effect_stopper == {api_tgt_item.id: [api_tgt_effect_id]}


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


def test_known_failures(client, consts):
    eve_tgt_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active)
    eve_src_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.stopper,
        loc=consts.EveModLoc.tgt_stopper,
        effect_id=eve_tgt_effect_id)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_src_mod])
    eve_src_item_id = client.mk_eve_item(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_tgt_item_id = client.mk_eve_item(eff_ids=[eve_tgt_effect_id], defeff_id=eve_tgt_effect_id)
    eve_other_id = client.mk_eve_item()
    client.create_sources()
    api_tgt_effect_id = effect_dogma_to_api(dogma_effect_id=eve_tgt_effect_id)
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_item = api_src_fit.add_module(type_id=eve_src_item_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_other = api_tgt_fit.add_implant(type_id=eve_other_id)
    api_tgt_item1 = api_tgt_fit.add_fighter(type_id=eve_tgt_item_id, state=consts.ApiMinionState.engaging)
    api_tgt_item2 = api_tgt_fit.add_fighter(type_id=eve_tgt_item_id, state=consts.ApiMinionState.engaging)
    api_src_item.change_module(add_projs=[api_tgt_item1.id, api_tgt_item2.id])
    # Verification
    api_val = api_tgt_fit.validate(options=ValOptions(effect_stopper=(True, [api_src_item.id])))
    assert api_val.passed is False
    assert api_val.details.effect_stopper == {
        api_tgt_item1.id: [api_tgt_effect_id],
        api_tgt_item2.id: [api_tgt_effect_id]}
    api_val = api_tgt_fit.validate(options=ValOptions(effect_stopper=(True, [api_tgt_item1.id])))
    assert api_val.passed is False
    assert api_val.details.effect_stopper == {api_tgt_item2.id: [api_tgt_effect_id]}
    api_val = api_tgt_fit.validate(options=ValOptions(effect_stopper=(True, [api_tgt_item2.id])))
    assert api_val.passed is False
    assert api_val.details.effect_stopper == {api_tgt_item1.id: [api_tgt_effect_id]}
    api_val = api_tgt_fit.validate(options=ValOptions(effect_stopper=(True, [api_tgt_item1.id, api_tgt_item2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_tgt_fit.validate(options=ValOptions(
        effect_stopper=(True, [api_tgt_item1.id, api_tgt_other.id, api_tgt_item2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_not_loaded_src(client, consts):
    eve_tgt_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active)
    eve_src_item_id = client.alloc_item_id()
    eve_tgt_item_id = client.mk_eve_item(eff_ids=[eve_tgt_effect_id], defeff_id=eve_tgt_effect_id)
    client.create_sources()
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


def test_not_loaded_tgt(client, consts):
    eve_tgt_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active)
    eve_src_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.stopper,
        loc=consts.EveModLoc.tgt_stopper,
        effect_id=eve_tgt_effect_id)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_src_mod])
    eve_src_item_id = client.mk_eve_item(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_tgt_item_id = client.alloc_item_id()
    client.create_sources()
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


def test_criterion_effect_cat(client, consts):
    # Only targeted effects can stop other effects
    eve_tgt_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active)
    eve_src_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.stopper,
        loc=consts.EveModLoc.tgt_stopper,
        effect_id=eve_tgt_effect_id)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, mod_info=[eve_src_mod])
    eve_src_item_id = client.mk_eve_item(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_tgt_item_id = client.mk_eve_item(eff_ids=[eve_tgt_effect_id], defeff_id=eve_tgt_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_item = api_src_fit.add_module(type_id=eve_src_item_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.add_fighter(type_id=eve_tgt_item_id, state=consts.ApiMinionState.engaging)
    api_src_item.change_module(add_projs=[api_tgt_item.id])
    # Verification
    api_val = api_tgt_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_src_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
