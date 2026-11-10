from fw import Muta, Spool, approx, check_no_field


def test_add_proj_not_found(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_tgt_attr_id = client.mk_eve_attr()
    eve_local_attr_id = client.mk_eve_attr()
    eve_charge_attr_id = client.mk_eve_attr()
    eve_tgt_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_tgt_attr_id)
    eve_tgt_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_tgt_mod])
    eve_local_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_local_attr_id)
    eve_local_effect_id = client.mk_eve_effect(mod_info=[eve_local_mod])
    eve_charge_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_charge_attr_id)
    eve_charge_effect_id = client.mk_eve_effect(mod_info=[eve_charge_mod])
    eve_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_tgt_effect_id, eve_local_effect_id],
        defeff_id=eve_tgt_effect_id)
    eve_charge_id = client.mk_eve_item(attrs={eve_affector_attr_id: 50}, eff_ids=[eve_charge_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_tgt_attr_id: 100, eve_local_attr_id: 100, eve_charge_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module1 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high)
    api_module2 = api_fit.add_module(type_id=eve_module_id, rack=consts.Rack.high)
    # Verification
    assert api_ship.update().attrs[eve_local_attr_id].modified == approx(144)
    api_other_tgt = api_sol.add_sw_effect(type_id=eve_ship_id)
    api_other_tgt.remove()
    # Action & verification - insertion shifts both modules, then the new module gets a charge,
    # overrides and its first projection, and its second projection fails
    api_fit.add_module(
        type_id=eve_module_id,
        rack=consts.Rack.high,
        add_mode={consts.ApiModAddMode.insert: 0},
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id,
        spool_override=Spool.cycles_to_api(count=5),
        optional_reload_override=consts.ApiOptionalReload.on_empty,
        proj_item_ids=[api_tgt_ship.id, api_other_tgt.id],
        effect_modes={eve_local_effect_id: consts.ApiEffMode.force_stop},
        status_code=400,
        json_predicate={
            'code': 'MOD-002',
            'message': f'failed to add module: failed to add projection: item {api_other_tgt.id} not found'})
    api_fit.update()
    assert api_fit.modules.high.map(lambda m: m.id) == [api_module1.id, api_module2.id]
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 1
    api_ship.update()
    assert api_ship.attrs[eve_local_attr_id].modified == approx(144)
    assert api_ship.attrs[eve_charge_attr_id].modified == approx(100)
    assert api_tgt_ship.update().attrs[eve_tgt_attr_id].modified == approx(100)


def test_add_proj_unreceptive(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_tgt_attr_id = client.mk_eve_attr()
    eve_tgt_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_tgt_attr_id)
    eve_tgt_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_tgt_mod])
    eve_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_tgt_effect_id],
        defeff_id=eve_tgt_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_tgt_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_tgt_module = api_tgt_fit.add_module(type_id=eve_module_id)
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Action & verification
    api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_tgt_ship.id, api_tgt_module.id],
        status_code=400,
        json_predicate={
            'code': 'MOD-002',
            'message': (f"failed to add module: failed to add projection: "
                        f"module {api_tgt_module.id} can't receive projections")})
    api_fit.update()
    with check_no_field():
        api_fit.modules  # ruff:ignore[useless-expression]
    assert api_tgt_ship.update().attrs[eve_tgt_attr_id].modified == approx(100)


def test_add_mutated_proj_not_found(client, consts):
    eve_roll_attr_id = client.mk_eve_attr()
    eve_local_attr_id = client.mk_eve_attr()
    eve_local_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_roll_attr_id,
        affectee_attr_id=eve_local_attr_id)
    eve_local_effect_id = client.mk_eve_effect(mod_info=[eve_local_mod])
    eve_base_module_id = client.mk_eve_item(attrs={eve_roll_attr_id: 20}, eff_ids=[eve_local_effect_id])
    eve_mutated_module_id = client.mk_eve_item(is_dynamic=True, eff_ids=[eve_local_effect_id])
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_module_id], eve_mutated_module_id)],
        attrs={eve_roll_attr_id: (0.5, 1.5)})
    eve_ship_id = client.mk_eve_ship(attrs={eve_local_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_other_tgt = api_sol.add_sw_effect(type_id=eve_ship_id)
    api_other_tgt.remove()
    # Action & verification
    api_fit.add_module(
        type_id=eve_base_module_id,
        mutation=(eve_mutator_id, {eve_roll_attr_id: Muta.roll_to_api(val=1)}),
        proj_item_ids=[api_other_tgt.id],
        status_code=400,
        json_predicate={
            'code': 'MOD-002',
            'message': f'failed to add module: failed to add projection: item {api_other_tgt.id} not found'})
    api_fit.update()
    with check_no_field():
        api_fit.modules  # ruff:ignore[useless-expression]
    assert api_ship.update().attrs[eve_local_attr_id].modified == approx(100)


def test_change_proj_add_fail(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_tgt_attr_id = client.mk_eve_attr()
    eve_charge_attr_id = client.mk_eve_attr()
    eve_tgt_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_tgt_attr_id)
    eve_tgt_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_tgt_mod])
    eve_charge_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_charge_attr_id)
    eve_charge_effect_id = client.mk_eve_effect(mod_info=[eve_charge_mod])
    eve_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_tgt_effect_id],
        defeff_id=eve_tgt_effect_id)
    eve_charge1_id = client.mk_eve_item(attrs={eve_affector_attr_id: 50}, eff_ids=[eve_charge_effect_id])
    eve_charge2_id = client.mk_eve_item(attrs={eve_affector_attr_id: 70}, eff_ids=[eve_charge_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_tgt_attr_id: 100, eve_charge_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship1 = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_tgt_fit2 = api_sol.create_fit()
    api_tgt_ship2 = api_tgt_fit2.set_ship(type_id=eve_ship_id)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module1 = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge1_id,
        spool_override=Spool.cycles_to_api(count=5),
        optional_reload_override=consts.ApiOptionalReload.on_empty,
        proj_item_ids=[api_tgt_ship1.id])
    api_charge1_id = api_module1.charge.id
    api_module2 = api_fit.add_module(type_id=eve_module_id)
    # Verification
    assert api_tgt_ship1.update().attrs[eve_tgt_attr_id].modified == approx(120)
    assert api_ship.update().attrs[eve_charge_attr_id].modified == approx(150)
    api_other_tgt = api_sol.add_sw_effect(type_id=eve_ship_id)
    api_other_tgt.remove()
    # Action & verification - every part of the change is applied before the second projection fails
    api_module1.change_module(
        move={consts.ApiModMvMode.swap: 1},
        state=consts.ApiModuleState.online,
        charge_type_id=eve_charge2_id,
        spool_override=None,
        optional_reload_override=None,
        rm_proj_item_ids=[api_tgt_ship1.id],
        add_proj_item_ids=[api_tgt_ship2.id, api_other_tgt.id],
        effect_modes={eve_tgt_effect_id: consts.ApiEffMode.force_run},
        status_code=400,
        json_predicate={
            'code': 'MOD-003',
            'message': f'failed to change module: failed to add projection: item {api_other_tgt.id} not found'})
    api_fit.update()
    assert api_fit.modules.high.map(lambda m: m.id) == [api_module1.id, api_module2.id]
    api_module1.update()
    assert api_module1.pos == 0
    assert api_module1.state == consts.ApiModuleState.active
    assert api_module1.charge.id == api_charge1_id
    assert api_module1.charge.type_id == eve_charge1_id
    assert api_module1.spool_override == Spool.cycles_to_api(count=5)
    assert api_module1.optional_reload_override == consts.ApiOptionalReload.on_empty
    assert api_module1.projs.keys() == {api_tgt_ship1.id}
    with check_no_field():
        api_module1.effect_mode_overrides  # ruff:ignore[useless-expression]
    assert api_module2.update().pos == 1
    assert api_tgt_ship1.update().attrs[eve_tgt_attr_id].modified == approx(120)
    assert api_tgt_ship2.update().attrs[eve_tgt_attr_id].modified == approx(100)
    assert api_ship.update().attrs[eve_charge_attr_id].modified == approx(150)


def test_change_type_proj_add_fail(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_local_attr_id = client.mk_eve_attr()
    eve_local_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_local_attr_id)
    eve_local_effect_id = client.mk_eve_effect(mod_info=[eve_local_mod])
    eve_module1_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_local_effect_id])
    eve_module2_id = client.mk_eve_item(attrs={eve_affector_attr_id: 50}, eff_ids=[eve_local_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_local_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module1_id)
    # Verification
    assert api_ship.update().attrs[eve_local_attr_id].modified == approx(120)
    api_other_tgt = api_sol.add_sw_effect(type_id=eve_ship_id)
    api_other_tgt.remove()
    # Action & verification
    api_module.change_module(
        type_id=eve_module2_id,
        add_proj_item_ids=[api_other_tgt.id],
        status_code=400,
        json_predicate={
            'code': 'MOD-003',
            'message': f'failed to change module: failed to add projection: item {api_other_tgt.id} not found'})
    api_module.update()
    assert api_module.type_id == eve_module1_id
    assert api_module.attrs[eve_affector_attr_id].modified == approx(20)
    assert api_ship.update().attrs[eve_local_attr_id].modified == approx(120)


def test_change_rolls_proj_add_fail(client, consts):
    eve_roll_attr_id = client.mk_eve_attr()
    eve_local_attr_id = client.mk_eve_attr()
    eve_local_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_roll_attr_id,
        affectee_attr_id=eve_local_attr_id)
    eve_local_effect_id = client.mk_eve_effect(mod_info=[eve_local_mod])
    eve_base_module_id = client.mk_eve_item(attrs={eve_roll_attr_id: 20}, eff_ids=[eve_local_effect_id])
    eve_mutated_module_id = client.mk_eve_item(is_dynamic=True, eff_ids=[eve_local_effect_id])
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_module_id], eve_mutated_module_id)],
        attrs={eve_roll_attr_id: (0.5, 1.5)})
    eve_ship_id = client.mk_eve_ship(attrs={eve_local_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(
        type_id=eve_base_module_id,
        mutation=(eve_mutator_id, {eve_roll_attr_id: Muta.roll_to_api(val=0.8)}))
    # Verification
    assert api_ship.update().attrs[eve_local_attr_id].modified == approx(126)
    api_other_tgt = api_sol.add_sw_effect(type_id=eve_ship_id)
    api_other_tgt.remove()
    # Action & verification
    api_module.change_module(
        mutation={eve_roll_attr_id: Muta.roll_to_api(val=0.1)},
        add_proj_item_ids=[api_other_tgt.id],
        status_code=400,
        json_predicate={
            'code': 'MOD-003',
            'message': f'failed to change module: failed to add projection: item {api_other_tgt.id} not found'})
    api_module.update()
    assert api_module.mutation.rolls[eve_roll_attr_id] == approx(0.8)
    assert api_module.attrs[eve_roll_attr_id].modified == approx(26)
    assert api_ship.update().attrs[eve_local_attr_id].modified == approx(126)


def test_change_unmutate_proj_add_fail(client, consts):
    eve_roll_attr_id = client.mk_eve_attr()
    eve_local_attr_id = client.mk_eve_attr()
    eve_local_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_roll_attr_id,
        affectee_attr_id=eve_local_attr_id)
    eve_local_effect_id = client.mk_eve_effect(mod_info=[eve_local_mod])
    eve_base_module_id = client.mk_eve_item(attrs={eve_roll_attr_id: 20}, eff_ids=[eve_local_effect_id])
    eve_mutated_module_id = client.mk_eve_item(is_dynamic=True, eff_ids=[eve_local_effect_id])
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_module_id], eve_mutated_module_id)],
        attrs={eve_roll_attr_id: (0.5, 1.5)})
    eve_ship_id = client.mk_eve_ship(attrs={eve_local_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(
        type_id=eve_base_module_id,
        mutation=(eve_mutator_id, {eve_roll_attr_id: Muta.roll_to_api(val=0.8)}))
    api_other_tgt = api_sol.add_sw_effect(type_id=eve_ship_id)
    api_other_tgt.remove()
    # Action & verification
    api_module.change_module(
        mutation=None,
        add_proj_item_ids=[api_other_tgt.id],
        status_code=400,
        json_predicate={
            'code': 'MOD-003',
            'message': f'failed to change module: failed to add projection: item {api_other_tgt.id} not found'})
    api_module.update()
    assert api_module.type_id == eve_mutated_module_id
    assert api_module.mutation.base_type_id == eve_base_module_id
    assert api_module.mutation.rolls[eve_roll_attr_id] == approx(0.8)
    assert api_module.attrs[eve_roll_attr_id].modified == approx(26)
    assert api_ship.update().attrs[eve_local_attr_id].modified == approx(126)


def test_change_not_mutated(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_charge_attr_id = client.mk_eve_attr()
    eve_charge_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_charge_attr_id)
    eve_charge_effect_id = client.mk_eve_effect(mod_info=[eve_charge_mod])
    eve_module_id = client.mk_eve_item()
    eve_charge_id = client.mk_eve_item(attrs={eve_affector_attr_id: 50}, eff_ids=[eve_charge_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_charge_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id)
    # Action & verification - state and charge are changed before rolls are found to have no
    # mutation to go to
    api_module.change_module(
        state=consts.ApiModuleState.online,
        charge_type_id=eve_charge_id,
        mutation={eve_affector_attr_id: Muta.roll_to_api(val=0.1)},
        status_code=400,
        json_predicate={
            'code': 'MOD-005',
            'message': (f'failed to change module: failed to mutate attributes: '
                        f'item {api_module.id} is not mutated')})
    api_module.update()
    assert api_module.state == consts.ApiModuleState.offline
    with check_no_field():
        api_module.charge  # ruff:ignore[useless-expression]
    assert api_ship.update().attrs[eve_charge_attr_id].modified == approx(100)


def test_change_proj_rm_fail(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_tgt_attr_id = client.mk_eve_attr()
    eve_tgt_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_tgt_attr_id)
    eve_tgt_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_tgt_mod])
    eve_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_tgt_effect_id],
        defeff_id=eve_tgt_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_tgt_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_tgt_ship.id])
    api_other_tgt = api_sol.add_sw_effect(type_id=eve_ship_id)
    api_other_tgt.remove()
    # Action & verification - existing projection is removed before the next projectee is not found
    api_module.change_module(
        rm_proj_item_ids=[api_tgt_ship.id, api_other_tgt.id],
        status_code=400,
        json_predicate={
            'code': 'MOD-004',
            'message': f'failed to change module: failed to remove projection: item {api_other_tgt.id} not found'})
    assert api_module.update().projs.keys() == {api_tgt_ship.id}
    assert api_tgt_ship.update().attrs[eve_tgt_attr_id].modified == approx(120)
