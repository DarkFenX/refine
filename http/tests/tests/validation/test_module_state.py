from tests import check_no_field
from tests.fw.api import ValOptions


def test_ship_modules(client, consts):
    eve_passive_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive)
    eve_online_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.online)
    eve_active_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active)
    eve_overload_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.overload)
    eve_none_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module)
    eve_passive_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, eff_ids=[eve_passive_effect_id])
    eve_online_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, eff_ids=[eve_online_effect_id])
    eve_active_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, eff_ids=[eve_active_effect_id])
    eve_overload_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, eff_ids=[eve_overload_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_none_module = api_fit.add_mod(type_id=eve_none_module_id, state=consts.ApiModuleState.offline)
    api_passive_module = api_fit.add_mod(type_id=eve_passive_module_id, state=consts.ApiModuleState.offline)
    api_online_module = api_fit.add_mod(type_id=eve_online_module_id, state=consts.ApiModuleState.offline)
    api_active_module = api_fit.add_mod(type_id=eve_active_module_id, state=consts.ApiModuleState.offline)
    api_overload_module = api_fit.add_mod(type_id=eve_overload_module_id, state=consts.ApiModuleState.offline)
    # Verification
    api_val = api_fit.validate(options=ValOptions(module_state=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_none_module.change_mod(state=consts.ApiModuleState.online)
    api_passive_module.change_mod(state=consts.ApiModuleState.online)
    api_online_module.change_mod(state=consts.ApiModuleState.online)
    api_active_module.change_mod(state=consts.ApiModuleState.online)
    api_overload_module.change_mod(state=consts.ApiModuleState.online)
    # Verification
    api_val = api_fit.validate(options=ValOptions(module_state=True))
    assert api_val.passed is False
    assert api_val.details.module_state == {
        api_none_module.id: [consts.ApiModuleState.online, consts.ApiModuleState.offline],
        api_passive_module.id: [consts.ApiModuleState.online, consts.ApiModuleState.offline]}
    # Action
    api_none_module.change_mod(state=consts.ApiModuleState.active)
    api_passive_module.change_mod(state=consts.ApiModuleState.active)
    api_online_module.change_mod(state=consts.ApiModuleState.active)
    api_active_module.change_mod(state=consts.ApiModuleState.active)
    api_overload_module.change_mod(state=consts.ApiModuleState.active)
    # Verification
    api_val = api_fit.validate(options=ValOptions(module_state=True))
    assert api_val.passed is False
    assert api_val.details.module_state == {
        api_none_module.id: [consts.ApiModuleState.active, consts.ApiModuleState.offline],
        api_passive_module.id: [consts.ApiModuleState.active, consts.ApiModuleState.offline],
        api_online_module.id: [consts.ApiModuleState.active, consts.ApiModuleState.online]}
    # Action
    api_none_module.change_mod(state=consts.ApiModuleState.overload)
    api_passive_module.change_mod(state=consts.ApiModuleState.overload)
    api_online_module.change_mod(state=consts.ApiModuleState.overload)
    api_active_module.change_mod(state=consts.ApiModuleState.overload)
    api_overload_module.change_mod(state=consts.ApiModuleState.overload)
    # Verification
    api_val = api_fit.validate(options=ValOptions(module_state=True))
    assert api_val.passed is False
    assert api_val.details.module_state == {
        api_none_module.id: [consts.ApiModuleState.overload, consts.ApiModuleState.offline],
        api_passive_module.id: [consts.ApiModuleState.overload, consts.ApiModuleState.offline],
        api_online_module.id: [consts.ApiModuleState.overload, consts.ApiModuleState.online],
        api_active_module.id: [consts.ApiModuleState.overload, consts.ApiModuleState.active]}
    # Action
    api_none_module.change_mod(state=consts.ApiModuleState.active)
    api_passive_module.change_mod(state=consts.ApiModuleState.active)
    api_online_module.change_mod(state=consts.ApiModuleState.active)
    api_active_module.change_mod(state=consts.ApiModuleState.active)
    api_overload_module.change_mod(state=consts.ApiModuleState.active)
    # Verification
    api_val = api_fit.validate(options=ValOptions(module_state=True))
    assert api_val.passed is False
    assert api_val.details.module_state == {
        api_none_module.id: [consts.ApiModuleState.active, consts.ApiModuleState.offline],
        api_passive_module.id: [consts.ApiModuleState.active, consts.ApiModuleState.offline],
        api_online_module.id: [consts.ApiModuleState.active, consts.ApiModuleState.online]}
    # Action
    api_none_module.change_mod(state=consts.ApiModuleState.online)
    api_passive_module.change_mod(state=consts.ApiModuleState.online)
    api_online_module.change_mod(state=consts.ApiModuleState.online)
    api_active_module.change_mod(state=consts.ApiModuleState.online)
    api_overload_module.change_mod(state=consts.ApiModuleState.online)
    # Verification
    api_val = api_fit.validate(options=ValOptions(module_state=True))
    assert api_val.passed is False
    assert api_val.details.module_state == {
        api_none_module.id: [consts.ApiModuleState.online, consts.ApiModuleState.offline],
        api_passive_module.id: [consts.ApiModuleState.online, consts.ApiModuleState.offline]}
    # Action
    api_none_module.change_mod(state=consts.ApiModuleState.offline)
    api_passive_module.change_mod(state=consts.ApiModuleState.offline)
    api_online_module.change_mod(state=consts.ApiModuleState.offline)
    api_active_module.change_mod(state=consts.ApiModuleState.offline)
    api_overload_module.change_mod(state=consts.ApiModuleState.offline)
    # Verification
    api_val = api_fit.validate(options=ValOptions(module_state=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_structure_modules(client, consts):
    eve_passive_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive)
    eve_online_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.online)
    eve_active_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active)
    eve_overload_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.overload)
    eve_none_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.structure_module)
    eve_passive_module_id = client.mk_eve_item(
        cat_id=consts.EveItemCat.structure_module,
        eff_ids=[eve_passive_effect_id])
    eve_online_module_id = client.mk_eve_item(
        cat_id=consts.EveItemCat.structure_module,
        eff_ids=[eve_online_effect_id])
    eve_active_module_id = client.mk_eve_item(
        cat_id=consts.EveItemCat.structure_module,
        eff_ids=[eve_active_effect_id])
    eve_overload_module_id = client.mk_eve_item(
        cat_id=consts.EveItemCat.structure_module,
        eff_ids=[eve_overload_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_none_module = api_fit.add_mod(type_id=eve_none_module_id, state=consts.ApiModuleState.offline)
    api_passive_module = api_fit.add_mod(type_id=eve_passive_module_id, state=consts.ApiModuleState.offline)
    api_online_module = api_fit.add_mod(type_id=eve_online_module_id, state=consts.ApiModuleState.offline)
    api_active_module = api_fit.add_mod(type_id=eve_active_module_id, state=consts.ApiModuleState.offline)
    api_overload_module = api_fit.add_mod(type_id=eve_overload_module_id, state=consts.ApiModuleState.offline)
    # Verification
    api_val = api_fit.validate(options=ValOptions(module_state=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_none_module.change_mod(state=consts.ApiModuleState.online)
    api_passive_module.change_mod(state=consts.ApiModuleState.online)
    api_online_module.change_mod(state=consts.ApiModuleState.online)
    api_active_module.change_mod(state=consts.ApiModuleState.online)
    api_overload_module.change_mod(state=consts.ApiModuleState.online)
    # Verification
    api_val = api_fit.validate(options=ValOptions(module_state=True))
    assert api_val.passed is False
    assert api_val.details.module_state == {
        api_none_module.id: [consts.ApiModuleState.online, consts.ApiModuleState.offline],
        api_passive_module.id: [consts.ApiModuleState.online, consts.ApiModuleState.offline]}
    # Action
    api_none_module.change_mod(state=consts.ApiModuleState.active)
    api_passive_module.change_mod(state=consts.ApiModuleState.active)
    api_online_module.change_mod(state=consts.ApiModuleState.active)
    api_active_module.change_mod(state=consts.ApiModuleState.active)
    api_overload_module.change_mod(state=consts.ApiModuleState.active)
    # Verification
    api_val = api_fit.validate(options=ValOptions(module_state=True))
    assert api_val.passed is False
    assert api_val.details.module_state == {
        api_none_module.id: [consts.ApiModuleState.active, consts.ApiModuleState.offline],
        api_passive_module.id: [consts.ApiModuleState.active, consts.ApiModuleState.offline],
        api_online_module.id: [consts.ApiModuleState.active, consts.ApiModuleState.online]}
    # Action
    api_none_module.change_mod(state=consts.ApiModuleState.overload)
    api_passive_module.change_mod(state=consts.ApiModuleState.overload)
    api_online_module.change_mod(state=consts.ApiModuleState.overload)
    api_active_module.change_mod(state=consts.ApiModuleState.overload)
    api_overload_module.change_mod(state=consts.ApiModuleState.overload)
    # Verification
    api_val = api_fit.validate(options=ValOptions(module_state=True))
    assert api_val.passed is False
    assert api_val.details.module_state == {
        api_none_module.id: [consts.ApiModuleState.overload, consts.ApiModuleState.offline],
        api_passive_module.id: [consts.ApiModuleState.overload, consts.ApiModuleState.offline],
        api_online_module.id: [consts.ApiModuleState.overload, consts.ApiModuleState.online],
        api_active_module.id: [consts.ApiModuleState.overload, consts.ApiModuleState.active]}
    # Action
    api_none_module.change_mod(state=consts.ApiModuleState.active)
    api_passive_module.change_mod(state=consts.ApiModuleState.active)
    api_online_module.change_mod(state=consts.ApiModuleState.active)
    api_active_module.change_mod(state=consts.ApiModuleState.active)
    api_overload_module.change_mod(state=consts.ApiModuleState.active)
    # Verification
    api_val = api_fit.validate(options=ValOptions(module_state=True))
    assert api_val.passed is False
    assert api_val.details.module_state == {
        api_none_module.id: [consts.ApiModuleState.active, consts.ApiModuleState.offline],
        api_passive_module.id: [consts.ApiModuleState.active, consts.ApiModuleState.offline],
        api_online_module.id: [consts.ApiModuleState.active, consts.ApiModuleState.online]}
    # Action
    api_none_module.change_mod(state=consts.ApiModuleState.online)
    api_passive_module.change_mod(state=consts.ApiModuleState.online)
    api_online_module.change_mod(state=consts.ApiModuleState.online)
    api_active_module.change_mod(state=consts.ApiModuleState.online)
    api_overload_module.change_mod(state=consts.ApiModuleState.online)
    # Verification
    api_val = api_fit.validate(options=ValOptions(module_state=True))
    assert api_val.passed is False
    assert api_val.details.module_state == {
        api_none_module.id: [consts.ApiModuleState.online, consts.ApiModuleState.offline],
        api_passive_module.id: [consts.ApiModuleState.online, consts.ApiModuleState.offline]}
    # Action
    api_none_module.change_mod(state=consts.ApiModuleState.offline)
    api_passive_module.change_mod(state=consts.ApiModuleState.offline)
    api_online_module.change_mod(state=consts.ApiModuleState.offline)
    api_active_module.change_mod(state=consts.ApiModuleState.offline)
    api_overload_module.change_mod(state=consts.ApiModuleState.offline)
    # Verification
    api_val = api_fit.validate(options=ValOptions(module_state=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_multiple_states(client, consts):
    # Covers all transitions: offline-overload-online-overload-offline-active-offline
    eve_passive_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive)
    eve_online_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.online)
    eve_active_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active)
    eve_overload_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.overload)
    eve_none_module_id = client.mk_eve_item()
    eve_passive_module_id = client.mk_eve_item(eff_ids=[eve_passive_effect_id])
    eve_online_module_id = client.mk_eve_item(eff_ids=[eve_online_effect_id])
    eve_active_module_id = client.mk_eve_item(eff_ids=[eve_active_effect_id])
    eve_overload_module_id = client.mk_eve_item(eff_ids=[eve_overload_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_none_module = api_fit.add_mod(type_id=eve_none_module_id, state=consts.ApiModuleState.offline)
    api_passive_module = api_fit.add_mod(type_id=eve_passive_module_id, state=consts.ApiModuleState.offline)
    api_online_module = api_fit.add_mod(type_id=eve_online_module_id, state=consts.ApiModuleState.offline)
    api_active_module = api_fit.add_mod(type_id=eve_active_module_id, state=consts.ApiModuleState.offline)
    api_overload_module = api_fit.add_mod(type_id=eve_overload_module_id, state=consts.ApiModuleState.offline)
    # Verification
    api_val = api_fit.validate(options=ValOptions(module_state=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_none_module.change_mod(state=consts.ApiModuleState.overload)
    api_passive_module.change_mod(state=consts.ApiModuleState.overload)
    api_online_module.change_mod(state=consts.ApiModuleState.overload)
    api_active_module.change_mod(state=consts.ApiModuleState.overload)
    api_overload_module.change_mod(state=consts.ApiModuleState.overload)
    # Verification
    api_val = api_fit.validate(options=ValOptions(module_state=True))
    assert api_val.passed is False
    assert api_val.details.module_state == {
        api_none_module.id: [consts.ApiModuleState.overload, consts.ApiModuleState.offline],
        api_passive_module.id: [consts.ApiModuleState.overload, consts.ApiModuleState.offline],
        api_online_module.id: [consts.ApiModuleState.overload, consts.ApiModuleState.online],
        api_active_module.id: [consts.ApiModuleState.overload, consts.ApiModuleState.active]}
    # Action
    api_none_module.change_mod(state=consts.ApiModuleState.online)
    api_passive_module.change_mod(state=consts.ApiModuleState.online)
    api_online_module.change_mod(state=consts.ApiModuleState.online)
    api_active_module.change_mod(state=consts.ApiModuleState.online)
    api_overload_module.change_mod(state=consts.ApiModuleState.online)
    # Verification
    api_val = api_fit.validate(options=ValOptions(module_state=True))
    assert api_val.passed is False
    assert api_val.details.module_state == {
        api_none_module.id: [consts.ApiModuleState.online, consts.ApiModuleState.offline],
        api_passive_module.id: [consts.ApiModuleState.online, consts.ApiModuleState.offline]}
    # Action
    api_none_module.change_mod(state=consts.ApiModuleState.overload)
    api_passive_module.change_mod(state=consts.ApiModuleState.overload)
    api_online_module.change_mod(state=consts.ApiModuleState.overload)
    api_active_module.change_mod(state=consts.ApiModuleState.overload)
    api_overload_module.change_mod(state=consts.ApiModuleState.overload)
    # Verification
    api_val = api_fit.validate(options=ValOptions(module_state=True))
    assert api_val.passed is False
    assert api_val.details.module_state == {
        api_none_module.id: [consts.ApiModuleState.overload, consts.ApiModuleState.offline],
        api_passive_module.id: [consts.ApiModuleState.overload, consts.ApiModuleState.offline],
        api_online_module.id: [consts.ApiModuleState.overload, consts.ApiModuleState.online],
        api_active_module.id: [consts.ApiModuleState.overload, consts.ApiModuleState.active]}
    # Action
    api_none_module.change_mod(state=consts.ApiModuleState.offline)
    api_passive_module.change_mod(state=consts.ApiModuleState.offline)
    api_online_module.change_mod(state=consts.ApiModuleState.offline)
    api_active_module.change_mod(state=consts.ApiModuleState.offline)
    api_overload_module.change_mod(state=consts.ApiModuleState.offline)
    # Verification
    api_val = api_fit.validate(options=ValOptions(module_state=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_none_module.change_mod(state=consts.ApiModuleState.active)
    api_passive_module.change_mod(state=consts.ApiModuleState.active)
    api_online_module.change_mod(state=consts.ApiModuleState.active)
    api_active_module.change_mod(state=consts.ApiModuleState.active)
    api_overload_module.change_mod(state=consts.ApiModuleState.active)
    # Verification
    api_val = api_fit.validate(options=ValOptions(module_state=True))
    assert api_val.passed is False
    assert api_val.details.module_state == {
        api_none_module.id: [consts.ApiModuleState.active, consts.ApiModuleState.offline],
        api_passive_module.id: [consts.ApiModuleState.active, consts.ApiModuleState.offline],
        api_online_module.id: [consts.ApiModuleState.active, consts.ApiModuleState.online]}
    # Action
    api_none_module.change_mod(state=consts.ApiModuleState.offline)
    api_passive_module.change_mod(state=consts.ApiModuleState.offline)
    api_online_module.change_mod(state=consts.ApiModuleState.offline)
    api_active_module.change_mod(state=consts.ApiModuleState.offline)
    api_overload_module.change_mod(state=consts.ApiModuleState.offline)
    # Verification
    api_val = api_fit.validate(options=ValOptions(module_state=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_known_failures(client, consts):
    eve_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module)
    eve_other_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_other = api_fit.add_implant(type_id=eve_other_id)
    api_fit.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.offline)
    api_module2 = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.online)
    api_module3 = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.online)
    # Verification
    api_val = api_fit.validate(options=ValOptions(module_state=(True, [api_module2.id])))
    assert api_val.passed is False
    assert api_val.details.module_state == {
        api_module3.id: [consts.ApiModuleState.online, consts.ApiModuleState.offline]}
    api_val = api_fit.validate(options=ValOptions(module_state=(True, [api_module3.id])))
    assert api_val.passed is False
    assert api_val.details.module_state == {
        api_module2.id: [consts.ApiModuleState.online, consts.ApiModuleState.offline]}
    api_val = api_fit.validate(options=ValOptions(module_state=(True, [api_module2.id, api_module3.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(options=ValOptions(module_state=(True, [api_module2.id, api_other.id, api_module3.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_mutation(client, consts):
    eve_passive_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive)
    eve_online_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.online)
    eve_base_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, eff_ids=[eve_passive_effect_id])
    eve_mutated_module_id = client.mk_eve_item(
        cat_id=consts.EveItemCat.module,
        eff_ids=[eve_passive_effect_id, eve_online_effect_id])
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_module_id], eve_mutated_module_id)])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_mod(type_id=eve_base_module_id, state=consts.ApiModuleState.online)
    # Verification
    api_val = api_fit.validate(options=ValOptions(module_state=True))
    assert api_val.passed is False
    assert api_val.details.module_state == {
        api_module.id: [consts.ApiModuleState.online, consts.ApiModuleState.offline]}
    # Action
    api_module.change_mod(mutation=eve_mutator_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(module_state=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_mod(mutation=None)
    # Verification
    api_val = api_fit.validate(options=ValOptions(module_state=True))
    assert api_val.passed is False
    assert api_val.details.module_state == {
        api_module.id: [consts.ApiModuleState.online, consts.ApiModuleState.offline]}


def test_not_loaded(client, consts):
    eve_module_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.offline)
    api_fit.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.online)
    api_fit.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fit.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.overload)
    # Verification
    api_val = api_fit.validate(options=ValOptions(module_state=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_criterion_item_kind(client, consts):
    eve_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_rig(type_id=eve_item_id, state=True)
    api_fit.add_drone(type_id=eve_item_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_val = api_fit.validate(options=ValOptions(module_state=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
