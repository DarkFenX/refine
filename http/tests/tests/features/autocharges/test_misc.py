from fw import Effect


def test_switch_state_invalid_reference(client, consts):
    # Just check that nothing crashes when switching fighter state / effect mode for fighters which
    # attempted to load an autocharge, but failed
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_abil_id = client.mk_eve_abil(id_=consts.EveAbil.launch_bomb)
    eve_charge_id = client.alloc_item_id()
    eve_fighter_id = client.mk_eve_fighter(
        attrs={eve_autocharge_attr_id: eve_charge_id},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id,
        abils=[client.mk_eve_item_abil(id_=eve_abil_id)])
    client.create_sources()
    api_effect_id = Effect.dogma_to_api(dogma_effect_id=eve_effect_id)
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_fighter.change_fighter(state=consts.ApiMinionState.in_bay)
    api_fighter.change_fighter(state=consts.ApiMinionState.engaging)
    api_fighter.change_fighter(abilities={eve_abil_id: False})
    api_fighter.change_fighter(abilities={eve_abil_id: True})
    api_fighter.change_fighter(effect_modes={api_effect_id: consts.ApiEffMode.force_stop})
    api_fighter.change_fighter(effect_modes={api_effect_id: consts.ApiEffMode.full_compliance})


def test_switch_state_no_reference(client, consts):
    # Just check that nothing crashes when switching fighter state / effect mode for fighters which
    # attempted to load an autocharge, but failed
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_abil_id = client.mk_eve_abil(id_=consts.EveAbil.launch_bomb)
    eve_fighter_id = client.mk_eve_fighter(
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id,
        abils=[client.mk_eve_item_abil(id_=eve_abil_id)])
    client.create_sources()
    api_effect_id = Effect.dogma_to_api(dogma_effect_id=eve_effect_id)
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_fighter.change_fighter(state=consts.ApiMinionState.in_bay)
    api_fighter.change_fighter(state=consts.ApiMinionState.engaging)
    api_fighter.change_fighter(abilities={eve_abil_id: False})
    api_fighter.change_fighter(abilities={eve_abil_id: True})
    api_fighter.change_fighter(effect_modes={api_effect_id: consts.ApiEffMode.force_stop})
    api_fighter.change_fighter(effect_modes={api_effect_id: consts.ApiEffMode.full_compliance})
