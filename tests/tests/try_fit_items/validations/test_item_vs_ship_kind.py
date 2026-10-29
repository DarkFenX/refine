from fw.api import ValOptions


def make_eve_items(*, client, consts) -> tuple[list[int], list[int], int]:
    # Ship and structure items of every kind the validation applies to, and a subsystem, which it
    # does not apply to
    eve_ship_item_ids = [
        client.mk_eve_module(rack=consts.Rack.high),
        client.mk_eve_module(rack=consts.Rack.mid),
        client.mk_eve_module(rack=consts.Rack.low),
        client.mk_eve_rig(),
        client.mk_eve_service(cat_id=consts.EveItemCat.module)]
    eve_struct_item_ids = [
        client.mk_eve_module(cat_id=consts.EveItemCat.structure_module, rack=consts.Rack.high),
        client.mk_eve_module(cat_id=consts.EveItemCat.structure_module, rack=consts.Rack.mid),
        client.mk_eve_module(cat_id=consts.EveItemCat.structure_module, rack=consts.Rack.low),
        client.mk_eve_rig(cat_id=consts.EveItemCat.structure_module),
        client.mk_eve_service()]
    eve_other_item_id = client.mk_eve_subsystem()
    return eve_ship_item_ids, eve_struct_item_ids, eve_other_item_id


def test_ship_kind_switching(client, consts):
    eve_ship_item_ids, eve_struct_item_ids, eve_other_item_id = make_eve_items(client=client, consts=consts)
    eve_ship_id = client.mk_eve_ship()
    eve_struct_id = client.mk_eve_struct()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(item_vs_ship_kind=True),
        passing=[*eve_ship_item_ids, *eve_struct_item_ids, eve_other_item_id],
        failing=[])
    # Action
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(item_vs_ship_kind=True),
        passing=[*eve_ship_item_ids, eve_other_item_id],
        failing=eve_struct_item_ids)
    # Action
    api_fit.unset_ship()
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(item_vs_ship_kind=True),
        passing=[*eve_ship_item_ids, *eve_struct_item_ids, eve_other_item_id],
        failing=[])
    # Action
    api_fit.set_ship(type_id=eve_struct_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(item_vs_ship_kind=True),
        passing=[*eve_struct_item_ids, eve_other_item_id],
        failing=eve_ship_item_ids)
    # Action
    api_fit.set_ship(type_id=eve_not_loaded_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(item_vs_ship_kind=True),
        passing=[*eve_ship_item_ids, *eve_struct_item_ids, eve_other_item_id],
        failing=[])
    # Action
    api_fit.set_ship(type_id=eve_other_item_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(item_vs_ship_kind=True),
        passing=[eve_other_item_id],
        failing=[*eve_ship_item_ids, *eve_struct_item_ids])


def test_item_add_remove_ship_ship(client, consts):
    eve_ship_item_ids, eve_struct_item_ids, eve_other_item_id = make_eve_items(client=client, consts=consts)
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(item_vs_ship_kind=True),
        passing=[*eve_ship_item_ids, eve_other_item_id],
        failing=eve_struct_item_ids)


def test_item_add_remove_ship_struct(client, consts):
    eve_ship_item_ids, eve_struct_item_ids, eve_other_item_id = make_eve_items(client=client, consts=consts)
    eve_struct_id = client.mk_eve_struct()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(item_vs_ship_kind=True),
        passing=[*eve_struct_item_ids, eve_other_item_id],
        failing=eve_ship_item_ids)


def test_item_add_remove_ship_unknown(client, consts):
    eve_ship_item_ids, eve_struct_item_ids, eve_other_item_id = make_eve_items(client=client, consts=consts)
    eve_other_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_other_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(item_vs_ship_kind=True),
        passing=[eve_other_item_id],
        failing=[*eve_ship_item_ids, *eve_struct_item_ids])


def test_item_add_remove_ship_none(client, consts):
    eve_ship_item_ids, eve_struct_item_ids, eve_other_item_id = make_eve_items(client=client, consts=consts)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(item_vs_ship_kind=True),
        passing=[*eve_ship_item_ids, *eve_struct_item_ids, eve_other_item_id],
        failing=[])


def test_item_add_remove_ship_not_loaded(client, consts):
    eve_ship_item_ids, eve_struct_item_ids, eve_other_item_id = make_eve_items(client=client, consts=consts)
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(item_vs_ship_kind=True),
        passing=[*eve_ship_item_ids, *eve_struct_item_ids, eve_other_item_id],
        failing=[])


def test_known_failures(client, consts):
    eve_ship_item_ids, eve_struct_item_ids, eve_other_item_id = make_eve_items(client=client, consts=consts)
    eve_ship_item_id = client.mk_eve_item(cat_id=consts.EveItemCat.module)
    eve_struct_id = client.mk_eve_struct()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct_id)
    api_ship_module = api_fit.add_module(type_id=eve_ship_item_id)
    api_ship_rig = api_fit.add_rig(type_id=eve_ship_item_id)
    api_ship_service = api_fit.add_service(type_id=eve_ship_item_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(item_vs_ship_kind=[api_ship_module.id, api_ship_rig.id]),
        passing=[],
        failing=[*eve_struct_item_ids, eve_other_item_id, *eve_ship_item_ids])
    api_fit.check_try_fit_items(
        val_options=ValOptions(item_vs_ship_kind=[api_ship_module.id, api_ship_service.id]),
        passing=[],
        failing=[*eve_struct_item_ids, eve_other_item_id, *eve_ship_item_ids])
    api_fit.check_try_fit_items(
        val_options=ValOptions(item_vs_ship_kind=[api_ship_rig.id, api_ship_service.id]),
        passing=[],
        failing=[*eve_struct_item_ids, eve_other_item_id, *eve_ship_item_ids])
    api_fit.check_try_fit_items(
        val_options=ValOptions(item_vs_ship_kind=[api_ship_module.id, api_ship_rig.id, api_ship_service.id]),
        passing=[*eve_struct_item_ids, eve_other_item_id],
        failing=eve_ship_item_ids)


def test_mutation(client, consts):
    eve_ship_item_ids, eve_struct_item_ids, eve_other_item_id = make_eve_items(client=client, consts=consts)
    eve_base_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module)
    eve_mutated_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.structure_module)
    eve_ship_id = client.mk_eve_ship()
    eve_struct_id = client.mk_eve_struct()
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_module_id], eve_mutated_module_id)])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_base_module_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(item_vs_ship_kind=True),
        passing=[*eve_ship_item_ids, eve_other_item_id],
        failing=eve_struct_item_ids)
    # Action
    api_module.change_module(mutation=eve_mutator_id)
    api_fit.set_ship(type_id=eve_struct_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(item_vs_ship_kind=True),
        passing=[*eve_struct_item_ids, eve_other_item_id],
        failing=eve_ship_item_ids)


def test_criterion_item_category(client):
    eve_item_id = client.mk_eve_subsystem()
    eve_ship_id = client.mk_eve_ship()
    eve_struct_id = client.mk_eve_struct()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(item_vs_ship_kind=True), passing=[eve_item_id], failing=[])
    # Action
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(item_vs_ship_kind=True), passing=[eve_item_id], failing=[])
    # Action
    api_fit.set_ship(type_id=eve_struct_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(item_vs_ship_kind=True), passing=[eve_item_id], failing=[])


def test_criterion_item_kind(client, consts):
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_booster_id = client.mk_eve_booster()
    eve_charge_id = client.mk_eve_charge()
    eve_drone_id = client.mk_eve_drone()
    eve_fighter_id = client.mk_eve_fighter()
    eve_implant_id = client.mk_eve_implant()
    eve_module_id = client.mk_eve_module()
    eve_launcher_id = client.mk_eve_item(cat_id=consts.EveItemCat.structure_module, attrs={eve_capacity_attr_id: 1})
    eve_struct_id = client.mk_eve_struct()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct_id)
    api_fit.add_module(type_id=eve_launcher_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(item_vs_ship_kind=True),
        passing=[eve_booster_id, eve_charge_id, eve_drone_id, eve_fighter_id, eve_implant_id],
        failing=[eve_module_id])
