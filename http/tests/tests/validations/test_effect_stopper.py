from tests import check_no_field, effect_dogma_to_api
from tests.fw.api import ValOptions


def test_project_unproject(client, consts):
    eve_projectee_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active)
    eve_projector_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.stopper,
        loc=consts.EveModLoc.tgt_stopper,
        effect_id=eve_projectee_effect_id)
    eve_projector_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_projector_mod])
    eve_projector_id = client.mk_eve_item(eff_ids=[eve_projector_effect_id], defeff_id=eve_projector_effect_id)
    eve_projectee_id = client.mk_eve_item(eff_ids=[eve_projectee_effect_id], defeff_id=eve_projectee_effect_id)
    client.create_sources()
    api_projectee_effect_id = effect_dogma_to_api(dogma_effect_id=eve_projectee_effect_id)
    api_sol = client.create_sol()
    api_projector_fit = api_sol.create_fit()
    api_projector_item = api_projector_fit.add_module(type_id=eve_projector_id, state=consts.ApiModuleState.active)
    api_projectee_fit = api_sol.create_fit()
    api_projectee_item = api_projectee_fit.add_fighter(type_id=eve_projectee_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_val = api_projectee_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_projector_item.change_module(add_projs=[api_projectee_item.id])
    # Verification
    api_val = api_projectee_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is False
    assert api_val.details.effect_stopper == {api_projectee_item.id: [api_projectee_effect_id]}
    # Action
    api_projector_item.change_module(rm_projs=[api_projectee_item.id])
    # Verification
    api_val = api_projectee_fit.validate(options=ValOptions(effect_stopper=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
