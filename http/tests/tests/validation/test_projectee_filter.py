from fw import Effect, Muta, approx, check_no_field
from fw.api import ValOptions


def test_src_breacher_tgt_ship_project_unproject(client, consts):
    # Also test that only validation of source fit is affected
    eve_tgt_list_attr_id = client.mk_eve_attr(id_=consts.EveAttr.valid_tgt_whitelist)
    eve_launcher_effect_id = client.mk_eve_effect(id_=consts.EveEffect.use_missiles, cat_id=consts.EveEffCat.active)
    eve_breacher_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.dot_missile_launching,
        cat_id=consts.EveEffCat.target)
    eve_ship1_id = client.mk_eve_ship()
    eve_ship2_id = client.mk_eve_ship()
    eve_item_list_id = client.mk_eve_item_list(inc_type_ids=[eve_ship2_id])
    eve_launcher_id = client.mk_eve_item(
        attrs={eve_tgt_list_attr_id: eve_item_list_id},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_breacher_id = client.mk_eve_item(eff_ids=[eve_breacher_effect_id], defeff_id=eve_breacher_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_launcher = api_src_fit.add_module(
        type_id=eve_launcher_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_breacher_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship1_id)
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_tgt_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_src_launcher.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is False
    assert api_val.details.projectee_filter == {api_src_launcher.id: [api_tgt_ship.id]}
    api_val = api_tgt_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_src_launcher.change_module(rm_projs=[api_tgt_ship.id])
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_tgt_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_src_launcher.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is False
    assert api_val.details.projectee_filter == {api_src_launcher.id: [api_tgt_ship.id]}
    # Action
    api_tgt_ship.change_ship(type_id=eve_ship2_id)
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_src_direct_dd(client, consts):
    # Also check multiple projector items
    eve_capships_skill_id = client.mk_eve_item(id_=consts.EveItem.capital_ships)
    eve_other_skill_id = client.mk_eve_item()
    eve_src_effect_id = client.mk_eve_effect(id_=consts.EveEffect.super_weapon_amarr, cat_id=consts.EveEffCat.target)
    eve_src_item_id = client.mk_eve_item(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_cap_ship_id = client.mk_eve_ship(srqs={eve_capships_skill_id: 1})
    eve_freighter_id = client.mk_eve_ship(grp_id=consts.EveItemGrp.freighter)
    eve_jf_id = client.mk_eve_ship(grp_id=consts.EveItemGrp.jump_freighter)
    eve_other_ship1_id = client.mk_eve_ship()
    eve_other_ship2_id = client.mk_eve_ship(srqs={eve_other_skill_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_module = api_src_fit.add_module(type_id=eve_src_item_id, state=consts.ApiModuleState.active)
    api_cap_ship = api_sol.create_fit().set_ship(type_id=eve_cap_ship_id)
    api_freighter = api_sol.create_fit().set_ship(type_id=eve_freighter_id)
    api_jf = api_sol.create_fit().set_ship(type_id=eve_jf_id)
    api_other_ship1 = api_sol.create_fit().set_ship(type_id=eve_other_ship1_id)
    api_other_ship2 = api_sol.create_fit().set_ship(type_id=eve_other_ship2_id)
    api_src_module.change_module(add_projs=[api_other_ship1.id])
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is False
    assert api_val.details.projectee_filter == {api_src_module.id: [api_other_ship1.id]}
    # Action
    api_src_module.change_module(add_projs=[api_cap_ship.id, api_freighter.id, api_jf.id, api_other_ship2.id])
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is False
    assert api_val.details.projectee_filter == {api_src_module.id: [api_other_ship1.id, api_other_ship2.id]}
    # Action
    api_src_module.change_module(rm_projs=[api_other_ship1.id, api_other_ship2.id])
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_src_standup_vorton(client, consts):
    eve_tgt_list_attr_id = client.mk_eve_attr(id_=consts.EveAttr.tgt_filter_typelist_id)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.lightning_weapon, cat_id=consts.EveEffCat.target)
    eve_ship1_id = client.mk_eve_ship()
    eve_ship2_id = client.mk_eve_ship()
    eve_item_list_id = client.mk_eve_item_list(inc_type_ids=[eve_ship2_id])
    eve_module_id = client.mk_eve_item(
        attrs={eve_tgt_list_attr_id: eve_item_list_id},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_module = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship1_id)
    api_src_module.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is False
    assert api_val.details.projectee_filter == {api_src_module.id: [api_tgt_ship.id]}
    # Action
    api_tgt_ship.change_ship(type_id=eve_ship2_id)
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_multiple_src_effects(client, consts):
    eve_vorton_tgt_list_attr_id = client.mk_eve_attr(id_=consts.EveAttr.tgt_filter_typelist_id)
    eve_direct_effect_id = client.mk_eve_effect(id_=consts.EveEffect.super_weapon_amarr, cat_id=consts.EveEffCat.target)
    eve_vorton_effect_id = client.mk_eve_effect(id_=consts.EveEffect.lightning_weapon, cat_id=consts.EveEffCat.target)
    eve_ship_id = client.mk_eve_ship()
    eve_item_list_id = client.mk_eve_item_list()
    eve_module_id = client.mk_eve_item(
        attrs={eve_vorton_tgt_list_attr_id: eve_item_list_id},
        eff_ids=[eve_direct_effect_id, eve_vorton_effect_id],
        defeff_id=eve_direct_effect_id)
    client.create_sources()
    api_direct_effect_id = Effect.dogma_to_api(dogma_effect_id=eve_direct_effect_id)
    api_vorton_effect_id = Effect.dogma_to_api(dogma_effect_id=eve_vorton_effect_id)
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_module = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_src_module.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is False
    assert api_val.details.projectee_filter == {api_src_module.id: [api_tgt_ship.id]}
    # Action
    api_src_module.change_module(effect_modes={api_vorton_effect_id: consts.ApiEffMode.force_run})
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is False
    assert api_val.details.projectee_filter == {api_src_module.id: [api_tgt_ship.id]}
    # Action
    api_src_module.change_module(effect_modes={api_direct_effect_id: consts.ApiEffMode.force_stop})
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is False
    assert api_val.details.projectee_filter == {api_src_module.id: [api_tgt_ship.id]}
    # Action
    api_src_module.change_module(effect_modes={api_vorton_effect_id: consts.ApiEffMode.force_stop})
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_filter_reference_values(client, consts):
    eve_tgt_list_attr_id = client.mk_eve_attr(id_=consts.EveAttr.tgt_filter_typelist_id)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.lightning_weapon, cat_id=consts.EveEffCat.target)
    eve_ship_id = client.mk_eve_ship()
    eve_item_list_id = client.mk_eve_item_list(inc_type_ids=[eve_ship_id])
    client.mk_eve_item_list(id_=eve_item_list_id - 1)
    client.mk_eve_item_list(id_=eve_item_list_id + 1)
    eve_module1_id = client.mk_eve_item(
        attrs={eve_tgt_list_attr_id: eve_item_list_id - 0.6},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_module2_id = client.mk_eve_item(
        attrs={eve_tgt_list_attr_id: eve_item_list_id - 0.4},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_module3_id = client.mk_eve_item(
        attrs={eve_tgt_list_attr_id: eve_item_list_id + 0.4},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_module4_id = client.mk_eve_item(
        attrs={eve_tgt_list_attr_id: eve_item_list_id + 0.6},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_module = api_src_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_src_module.change_module(add_projs=[api_tgt_ship.id])
    # Verification - ID is resolved to lower value which is an empty list, thus check fails
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is False
    assert api_val.details.projectee_filter == {api_src_module.id: [api_tgt_ship.id]}
    # Action
    api_src_module.change_module(type_id=eve_module2_id)
    # Verification - ID is rounded to the closest value, which is the type list ID
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_src_module.change_module(type_id=eve_module3_id)
    # Verification - ID is rounded to our type list ID again
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_src_module.change_module(type_id=eve_module4_id)
    # Verification - ID is resolved to higher value which is an empty list
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is False
    assert api_val.details.projectee_filter == {api_src_module.id: [api_tgt_ship.id]}


def test_empty_list(client, consts):
    eve_tgt_list_attr_id = client.mk_eve_attr(id_=consts.EveAttr.tgt_filter_typelist_id)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.lightning_weapon, cat_id=consts.EveEffCat.target)
    eve_ship_id = client.mk_eve_ship()
    eve_item_list_id = client.mk_eve_item_list()
    eve_module_id = client.mk_eve_item(
        attrs={eve_tgt_list_attr_id: eve_item_list_id},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_module = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_src_module.change_module(add_projs=[api_tgt_ship.id])
    # Verification - if list is defined and fetched but is empty, validation fails
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is False
    assert api_val.details.projectee_filter == {api_src_module.id: [api_tgt_ship.id]}


def test_tgt_mutation(client, consts):
    eve_tgt_list_attr_id = client.mk_eve_attr(id_=consts.EveAttr.tgt_filter_typelist_id)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.lightning_weapon, cat_id=consts.EveEffCat.target)
    eve_tgt_base_drone_id = client.mk_eve_drone()
    eve_tgt_mutated_drone_id = client.mk_eve_drone()
    eve_tgt_mutator_id = client.mk_eve_mutator(items=[([eve_tgt_base_drone_id], eve_tgt_mutated_drone_id)])
    eve_item_list1_id = client.mk_eve_item_list(inc_type_ids=[eve_tgt_base_drone_id])
    eve_item_list2_id = client.mk_eve_item_list(inc_type_ids=[eve_tgt_mutated_drone_id])
    eve_module1_id = client.mk_eve_item(
        attrs={eve_tgt_list_attr_id: eve_item_list1_id},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_module2_id = client.mk_eve_item(
        attrs={eve_tgt_list_attr_id: eve_item_list2_id},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_module = api_src_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_drone = api_tgt_fit.add_drone(type_id=eve_tgt_base_drone_id)
    api_src_module.change_module(add_projs=[api_tgt_drone.id])
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_tgt_drone.change_drone(mutation=eve_tgt_mutator_id)
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is False
    assert api_val.details.projectee_filter == {api_src_module.id: [api_tgt_drone.id]}
    # Action
    api_src_module.change_module(type_id=eve_module2_id)
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_tgt_drone.change_drone(mutation=None)
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is False
    assert api_val.details.projectee_filter == {api_src_module.id: [api_tgt_drone.id]}


def test_src_mutation_itemlist_replaced(client, consts):
    eve_tgt_list_attr_id = client.mk_eve_attr(id_=consts.EveAttr.tgt_filter_typelist_id)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.lightning_weapon, cat_id=consts.EveEffCat.target)
    eve_ship1_id = client.mk_eve_ship()
    eve_ship2_id = client.mk_eve_ship()
    eve_item_list1_id = client.mk_eve_item_list(inc_type_ids=[eve_ship1_id])
    eve_item_list2_id = client.mk_eve_item_list(inc_type_ids=[eve_ship2_id])
    eve_base_module_id = client.mk_eve_item(
        attrs={eve_tgt_list_attr_id: eve_item_list2_id},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_mutated_module_id = client.mk_eve_item(
        attrs={eve_tgt_list_attr_id: eve_item_list1_id},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_module_id], eve_mutated_module_id)],
        attrs={eve_tgt_list_attr_id: (0, 2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_module = api_src_fit.add_module(type_id=eve_base_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship1_id)
    api_src_module.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    assert api_src_module.update().attrs[eve_tgt_list_attr_id].modified == approx(eve_item_list2_id)
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is False
    assert api_val.details.projectee_filter == {api_src_module.id: [api_tgt_ship.id]}
    # Action
    api_src_module.change_module(mutation=eve_mutator_id)
    # Verification
    assert api_src_module.update().attrs[eve_tgt_list_attr_id].modified == approx(eve_item_list1_id)
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_src_module.change_module(mutation={eve_tgt_list_attr_id: Muta.roll_to_api(val=0)})
    # Verification
    assert api_src_module.update().attrs[eve_tgt_list_attr_id].modified == approx(0)
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_tgt_ship.change_ship(type_id=eve_ship2_id)
    # Verification
    assert api_src_module.update().attrs[eve_tgt_list_attr_id].modified == approx(0)
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is False
    assert api_val.details.projectee_filter == {api_src_module.id: [api_tgt_ship.id]}
    # Action
    api_src_module.change_module(mutation=None)
    # Verification
    assert api_src_module.update().attrs[eve_tgt_list_attr_id].modified == approx(eve_item_list2_id)
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_src_mutation_itemlist_inherited(client, consts):
    eve_tgt_list_attr_id = client.mk_eve_attr(id_=consts.EveAttr.tgt_filter_typelist_id)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.lightning_weapon, cat_id=consts.EveEffCat.target)
    eve_ship1_id = client.mk_eve_ship()
    eve_ship2_id = client.mk_eve_ship()
    eve_item_list_id = client.mk_eve_item_list(inc_type_ids=[eve_ship2_id])
    eve_base_module_id = client.mk_eve_item(attrs={eve_tgt_list_attr_id: eve_item_list_id})
    eve_mutated_module_id = client.mk_eve_item(eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_module_id], eve_mutated_module_id)])
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_module = api_src_fit.add_module(type_id=eve_base_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship1_id)
    api_src_module.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_src_module.change_module(mutation=eve_mutator_id)
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is False
    assert api_val.details.projectee_filter == {api_src_module.id: [api_tgt_ship.id]}
    # Action
    api_tgt_ship.change_ship(type_id=eve_ship2_id)
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_tgt_ship.change_ship(type_id=eve_ship1_id)
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is False
    assert api_val.details.projectee_filter == {api_src_module.id: [api_tgt_ship.id]}
    # Action
    api_src_module.change_module(mutation=None)
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_known_failures(client, consts):
    eve_tgt_list_attr_id = client.mk_eve_attr(id_=consts.EveAttr.tgt_filter_typelist_id)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.lightning_weapon, cat_id=consts.EveEffCat.target)
    eve_tgt_item_id = client.mk_eve_ship()
    eve_item_list_id = client.mk_eve_item_list()
    eve_src_item_id = client.mk_eve_item(
        attrs={eve_tgt_list_attr_id: eve_item_list_id},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_other_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_other = api_src_fit.add_implant(type_id=eve_other_id)
    api_src_item1 = api_src_fit.add_module(type_id=eve_src_item_id, state=consts.ApiModuleState.active)
    api_src_item2 = api_src_fit.add_module(type_id=eve_src_item_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item_id)
    api_src_item1.change_module(add_projs=[api_tgt_item.id])
    api_src_item2.change_module(add_projs=[api_tgt_item.id])
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=(True, [api_src_item1.id])))
    assert api_val.passed is False
    assert api_val.details.projectee_filter == {api_src_item2.id: [api_tgt_item.id]}
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=(True, [api_src_item2.id])))
    assert api_val.passed is False
    assert api_val.details.projectee_filter == {api_src_item1.id: [api_tgt_item.id]}
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=(True, [api_src_item1.id, api_src_item2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_src_fit.validate(options=ValOptions(
        projectee_filter=(True, [api_src_item1.id, api_src_other.id, api_src_item2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_no_attr(client, consts):
    eve_tgt_list_attr_id = consts.EveAttr.tgt_filter_typelist_id
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.lightning_weapon, cat_id=consts.EveEffCat.target)
    eve_ship1_id = client.mk_eve_ship()
    eve_ship2_id = client.mk_eve_ship()
    eve_item_list_id = client.mk_eve_item_list(inc_type_ids=[eve_ship2_id])
    eve_module_id = client.mk_eve_item(
        attrs={eve_tgt_list_attr_id: eve_item_list_id},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_module = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship1_id)
    api_src_module.change_module(add_projs=[api_tgt_ship.id])
    # Verification - if list is defined and fetched but is empty, validation fails
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_not_loaded_src(client, consts):
    eve_tgt_item_id = client.mk_eve_ship()
    eve_src_item_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_item = api_src_fit.add_module(type_id=eve_src_item_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item_id)
    api_src_item.change_module(add_projs=[api_tgt_item.id])
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_not_loaded_tgt(client, consts):
    eve_tgt_list_attr_id = client.mk_eve_attr(id_=consts.EveAttr.tgt_filter_typelist_id)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.lightning_weapon, cat_id=consts.EveEffCat.target)
    eve_tgt_item1_id = client.alloc_item_id()
    eve_tgt_item2_id = client.alloc_item_id()
    eve_item_list_id = client.mk_eve_item_list(inc_type_ids=[eve_tgt_item2_id])
    eve_src_item_id = client.mk_eve_item(
        attrs={eve_tgt_list_attr_id: eve_item_list_id},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_item = api_src_fit.add_module(type_id=eve_src_item_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item1_id)
    api_src_item.change_module(add_projs=[api_tgt_item.id])
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is False
    assert api_val.details.projectee_filter == {api_src_item.id: [api_tgt_item.id]}
    # Action
    api_tgt_item.change_ship(type_id=eve_tgt_item2_id)
    # Verification
    api_val = api_src_fit.validate(options=ValOptions(projectee_filter=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
