from fw import Muta, Spool, approx, check_no_field
from tests.rollback.kinds import rollback_batch, setup_mod_basics


def setup_mutation(*, client, basic_info):
    eve_effect_id = client.mk_eve_effect(mod_info=[basic_info.ship_mod])
    eve_roll_attr_id = client.mk_eve_attr()
    eve_base_module1_id = client.mk_eve_item(
        attrs={basic_info.affector_attr_id: 20, eve_roll_attr_id: 100},
        eff_ids=[eve_effect_id])
    eve_base_module2_id = client.mk_eve_item(
        attrs={basic_info.affector_attr_id: 30, eve_roll_attr_id: 100},
        eff_ids=[eve_effect_id])
    eve_mutated_module1_id = client.mk_eve_item(eff_ids=[eve_effect_id])
    eve_mutated_module2_id = client.mk_eve_item(eff_ids=[eve_effect_id])
    eve_mutator1_id = client.mk_eve_mutator(
        items=[([eve_base_module1_id], eve_mutated_module1_id), ([eve_base_module2_id], eve_mutated_module2_id)],
        attrs={basic_info.affector_attr_id: (0.5, 1.5), eve_roll_attr_id: (0.5, 1.5)})
    eve_mutator2_id = client.mk_eve_mutator(
        items=[([eve_base_module1_id], eve_mutated_module1_id), ([eve_base_module2_id], eve_mutated_module2_id)],
        attrs={basic_info.affector_attr_id: (0.9, 1.1), eve_roll_attr_id: (0.9, 1.1)})
    return (
        eve_roll_attr_id, eve_base_module1_id, eve_base_module2_id, eve_mutated_module1_id, eve_mutator1_id,
        eve_mutator2_id)


def test_add_append(client, consts):
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(type_id=eve_module_id, add_mode={consts.ApiModAddMode.replace: 0})
    api_module3 = api_fit.add_module(type_id=eve_module_id, add_mode={consts.ApiModAddMode.replace: 2})
    # Action & verification
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.add_module(fit_id=api_fit.id, type_id=eve_module_id, add_mode=consts.ApiModAddMode.append)
    assert api_fit.update().modules.high.map(lambda m: m.id) == [api_module1.id, None, api_module3.id]
    assert api_module1.update().pos == 0
    assert api_module3.update().pos == 2


def test_add_equip(client, consts):
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(type_id=eve_module_id, add_mode={consts.ApiModAddMode.replace: 0})
    api_module3 = api_fit.add_module(type_id=eve_module_id, add_mode={consts.ApiModAddMode.replace: 2})
    # Action & verification - module takes the free slot
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.add_module(fit_id=api_fit.id, type_id=eve_module_id, add_mode=consts.ApiModAddMode.equip)
    assert api_fit.update().modules.high.map(lambda m: m.id) == [api_module1.id, None, api_module3.id]
    assert api_module1.update().pos == 0
    assert api_module3.update().pos == 2


def test_add_insert(client, consts):
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(type_id=eve_module_id, add_mode={consts.ApiModAddMode.replace: 0})
    api_module3 = api_fit.add_module(type_id=eve_module_id, add_mode={consts.ApiModAddMode.replace: 2})
    # Action & verification - module shifts everything after it, hole included
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.add_module(fit_id=api_fit.id, type_id=eve_module_id, add_mode={consts.ApiModAddMode.insert: 0})
    assert api_fit.update().modules.high.map(lambda m: m.id) == [api_module1.id, None, api_module3.id]
    assert api_module1.update().pos == 0
    assert api_module3.update().pos == 2


def test_add_replace(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_basic_info.ship_mod])
    eve_module1_id = client.mk_eve_item(attrs={eve_basic_info.affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_module2_id = client.mk_eve_item(attrs={eve_basic_info.affector_attr_id: 50}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module1 = api_fit.add_module(type_id=eve_module1_id, add_mode={consts.ApiModAddMode.replace: 0})
    api_module3 = api_fit.add_module(type_id=eve_module1_id, add_mode={consts.ApiModAddMode.replace: 2})
    # Verification
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(144)
    # Action & verification - new module removes the old one
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.add_module(fit_id=api_fit.id, type_id=eve_module2_id, add_mode={consts.ApiModAddMode.replace: 2})
    assert api_fit.update().modules.high.map(lambda m: m.id) == [api_module1.id, None, api_module3.id]
    assert api_module1.update().pos == 0
    assert api_module3.update().pos == 2
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(144)


def test_remove_remove(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_basic_info.ship_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_basic_info.affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module1 = api_fit.add_module(type_id=eve_module_id, add_mode={consts.ApiModAddMode.replace: 0})
    api_module3 = api_fit.add_module(type_id=eve_module_id, add_mode={consts.ApiModAddMode.replace: 2})
    # Verification
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(144)
    # Action & verification - removal shifts everything after the module, hole included
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.remove_item(item_id=api_module1.id, rm_mode=consts.ApiModRmMode.remove)
    assert api_fit.update().modules.high.map(lambda m: m.id) == [api_module1.id, None, api_module3.id]
    assert api_module1.update().pos == 0
    assert api_module3.update().pos == 2
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(144)


def test_remove_free(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_basic_info.ship_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_basic_info.affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module1 = api_fit.add_module(type_id=eve_module_id, add_mode={consts.ApiModAddMode.replace: 0})
    api_module3 = api_fit.add_module(type_id=eve_module_id, add_mode={consts.ApiModAddMode.replace: 2})
    # Verification
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(144)
    # Action & verification - removal of the last module trims the hole before it
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.remove_item(item_id=api_module3.id, rm_mode=consts.ApiModRmMode.free)
    assert api_fit.update().modules.high.map(lambda m: m.id) == [api_module1.id, None, api_module3.id]
    assert api_module1.update().pos == 0
    assert api_module3.update().pos == 2
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(144)


def test_move_shift(client, consts):
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(type_id=eve_module_id, add_mode={consts.ApiModAddMode.replace: 0})
    api_module2 = api_fit.add_module(type_id=eve_module_id, add_mode={consts.ApiModAddMode.replace: 1})
    api_module4 = api_fit.add_module(type_id=eve_module_id, add_mode={consts.ApiModAddMode.replace: 3})
    # Action & verification - first module goes past the rack end, the rest shift into its place
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.change_module(item_id=api_module1.id, move={consts.ApiModMvMode.shift: 5})
    assert api_fit.update().modules.high.map(lambda m: m.id) == [api_module1.id, api_module2.id, None, api_module4.id]
    assert api_module1.update().pos == 0
    assert api_module2.update().pos == 1
    assert api_module4.update().pos == 3


def test_move_swap(client, consts):
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(type_id=eve_module_id, add_mode={consts.ApiModAddMode.replace: 0})
    api_module3 = api_fit.add_module(type_id=eve_module_id, add_mode={consts.ApiModAddMode.replace: 2})
    # Action & verification - modules swap places, then the first module goes past the rack end
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.change_module(item_id=api_module1.id, move={consts.ApiModMvMode.swap: 2})
        api_sol_batch.change_module(item_id=api_module1.id, move={consts.ApiModMvMode.swap: 4})
    assert api_fit.update().modules.high.map(lambda m: m.id) == [api_module1.id, None, api_module3.id]
    assert api_module1.update().pos == 0
    assert api_module3.update().pos == 2


def test_change(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_basic_info.tgt_mod])
    eve_module1_id = client.mk_eve_item(
        attrs={eve_basic_info.affector_attr_id: 20},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_module2_id = client.mk_eve_item(
        attrs={eve_basic_info.affector_attr_id: 50},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module1_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_tgt_ship.id])
    # Verification
    assert api_tgt_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)
    # Action & verification
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.change_module(
            item_id=api_module.id,
            type_id=eve_module2_id,
            state=consts.ApiModuleState.online,
            spool_override=Spool.cycles_to_api(count=1),
            optional_reload_override=consts.ApiOptionalReload.on_empty,
            effect_modes={eve_effect_id: consts.ApiEffMode.force_stop})
    api_module.update()
    assert api_module.type_id == eve_module1_id
    assert api_module.state == consts.ApiModuleState.active
    with check_no_field():
        api_module.spool_override  # ruff:ignore[useless-expression]
    with check_no_field():
        api_module.optional_reload_override  # ruff:ignore[useless-expression]
    with check_no_field():
        api_module.effect_mode_overrides  # ruff:ignore[useless-expression]
    assert api_tgt_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)


def test_change_projs(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_basic_info.tgt_mod])
    eve_module_id = client.mk_eve_item(
        attrs={eve_basic_info.affector_attr_id: 20},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit1 = api_sol.create_fit()
    api_tgt_ship1 = api_tgt_fit1.set_ship(type_id=eve_ship_id)
    api_tgt_fit2 = api_sol.create_fit()
    api_tgt_ship2 = api_tgt_fit2.set_ship(type_id=eve_ship_id)
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_tgt_ship1.id])
    # Verification
    assert api_tgt_ship1.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)
    # Action & verification
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.change_module(
            item_id=api_module.id,
            rm_proj_item_ids=[api_tgt_ship1.id],
            add_proj_item_ids=[api_tgt_ship2.id])
    assert api_module.update().projs.keys() == {api_tgt_ship1.id}
    assert api_tgt_ship1.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)
    assert api_tgt_ship2.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(100)


def test_change_proj_with_charge(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_charge_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_basic_info.tgt_mod])
    eve_module_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.activates_charge, cat_id=consts.EveEffCat.active)
    eve_charge_id = client.mk_eve_item(
        attrs={eve_basic_info.affector_attr_id: 20},
        eff_ids=[eve_charge_effect_id],
        defeff_id=eve_charge_effect_id)
    eve_module_id = client.mk_eve_item(eff_ids=[eve_module_effect_id], defeff_id=eve_module_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit1 = api_sol.create_fit()
    api_tgt_ship1 = api_tgt_fit1.set_ship(type_id=eve_ship_id)
    api_tgt_fit2 = api_sol.create_fit()
    api_tgt_ship2 = api_tgt_fit2.set_ship(type_id=eve_ship_id)
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id,
        proj_item_ids=[api_tgt_ship1.id])
    # Verification
    assert api_tgt_ship1.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)
    # Action & verification - charge follows projections of its module
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.change_module(
            item_id=api_module.id,
            rm_proj_item_ids=[api_tgt_ship1.id],
            add_proj_item_ids=[api_tgt_ship2.id])
    assert api_module.update().projs.keys() == {api_tgt_ship1.id}
    assert api_tgt_ship1.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)
    assert api_tgt_ship2.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(100)


def test_mutation_add(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    _, eve_base_module_id, _, _, eve_mutator_id, _ = setup_mutation(client=client, basic_info=eve_basic_info)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_base_module_id)
    # Verification
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)
    # Action & verification
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.change_module(
            item_id=api_module.id,
            mutation=(eve_mutator_id, {eve_basic_info.affector_attr_id: Muta.roll_to_api(val=0.8)}))
    api_module.update()
    assert api_module.type_id == eve_base_module_id
    with check_no_field():
        api_module.mutation  # ruff:ignore[useless-expression]
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)


def test_mutation_change(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_roll_attr_id, eve_base_module_id, _, _, eve_mutator1_id, eve_mutator2_id = setup_mutation(
        client=client, basic_info=eve_basic_info)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(
        type_id=eve_base_module_id,
        mutation=(eve_mutator1_id, {
            eve_basic_info.affector_attr_id: Muta.roll_to_api(val=0.8),
            eve_roll_attr_id: Muta.roll_to_api(val=0.3)}))
    # Verification
    assert api_module.update().attrs[eve_roll_attr_id].modified == approx(80)
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(126)
    # Action & verification - mutator is replaced, one roll is changed and another one is removed
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.change_module(
            item_id=api_module.id,
            mutation=(eve_mutator2_id, {
                eve_basic_info.affector_attr_id: Muta.roll_to_api(val=0.1),
                eve_roll_attr_id: None}))
    api_module.update()
    assert api_module.mutation.mutator_id == eve_mutator1_id
    assert api_module.mutation.rolls == {eve_basic_info.affector_attr_id: approx(0.8), eve_roll_attr_id: approx(0.3)}
    assert api_module.attrs[eve_roll_attr_id].modified == approx(80)
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(126)


def test_mutation_change_type(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    _, eve_base_module1_id, eve_base_module2_id, eve_mutated_module1_id, eve_mutator_id, _ = setup_mutation(
        client=client, basic_info=eve_basic_info)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(
        type_id=eve_base_module1_id,
        mutation=(eve_mutator_id, {eve_basic_info.affector_attr_id: Muta.roll_to_api(val=0.8)}))
    # Verification
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(126)
    # Action & verification - base type is changed, the mutation stays
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.change_module(item_id=api_module.id, type_id=eve_base_module2_id)
    api_module.update()
    assert api_module.type_id == eve_mutated_module1_id
    assert api_module.mutation.base_type_id == eve_base_module1_id
    assert api_module.mutation.rolls == {eve_basic_info.affector_attr_id: approx(0.8)}
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(126)


def test_mutation_change_type_mutated_item_same(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_basic_info.ship_mod])
    eve_base_module1_id = client.mk_eve_item(attrs={eve_basic_info.affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_base_module2_id = client.mk_eve_item(attrs={eve_basic_info.affector_attr_id: 30}, eff_ids=[eve_effect_id])
    eve_mutated_module_id = client.mk_eve_item(eff_ids=[eve_effect_id])
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_module1_id, eve_base_module2_id], eve_mutated_module_id)],
        attrs={eve_basic_info.affector_attr_id: (0.5, 1.5)})
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(
        type_id=eve_base_module1_id,
        mutation=(eve_mutator_id, {eve_basic_info.affector_attr_id: Muta.roll_to_api(val=0.8)}))
    # Verification
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(126)
    # Action & verification - base type is changed, mutated item stays the same
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.change_module(item_id=api_module.id, type_id=eve_base_module2_id)
    api_module.update()
    assert api_module.type_id == eve_mutated_module_id
    assert api_module.mutation.base_type_id == eve_base_module1_id
    assert api_module.attrs[eve_basic_info.affector_attr_id].base == approx(26)
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(126)


def test_mutation_remove_effective(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    _, eve_base_module_id, _, eve_mutated_module_id, eve_mutator_id, _ = setup_mutation(
        client=client, basic_info=eve_basic_info)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(
        type_id=eve_base_module_id,
        mutation=(eve_mutator_id, {eve_basic_info.affector_attr_id: Muta.roll_to_api(val=0.8)}))
    # Verification
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(126)
    # Action & verification
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.change_module(item_id=api_module.id, mutation=None)
    api_module.update()
    assert api_module.type_id == eve_mutated_module_id
    assert api_module.mutation.mutator_id == eve_mutator_id
    assert api_module.mutation.rolls == {eve_basic_info.affector_attr_id: approx(0.8)}
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(126)


def test_mutation_remove_dormant(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    _, eve_base_module_id, _, _, _, _ = setup_mutation(client=client, basic_info=eve_basic_info)
    eve_mutator_id = client.alloc_item_id()
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(
        type_id=eve_base_module_id,
        mutation=(eve_mutator_id, {eve_basic_info.affector_attr_id: Muta.roll_to_api(val=0.8)}))
    # Verification
    assert api_module.update().mutation.type == consts.ApiMutationType.dormant
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)
    # Action & verification
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.change_module(item_id=api_module.id, mutation=None)
    api_module.update()
    assert api_module.type_id == eve_base_module_id
    assert api_module.mutation.type == consts.ApiMutationType.dormant
    assert api_module.mutation.mutator_id == eve_mutator_id
    assert api_module.mutation.rolls == {eve_basic_info.affector_attr_id: approx(0.8)}
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)
