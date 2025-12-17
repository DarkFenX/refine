from fw import approx


def test_switch_state(client, consts):
    eve_affector_attr1_id = client.mk_eve_attr()
    eve_affector_attr2_id = client.mk_eve_attr()
    eve_affectee_attr1_id = client.mk_eve_attr()
    eve_affectee_attr2_id = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr1_id,
        affectee_attr_id=eve_affectee_attr1_id)
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr2_id,
        affectee_attr_id=eve_affectee_attr2_id)
    eve_primary_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_attack_m,
        cat_id=consts.EveEffCat.target,
        mod_info=[eve_mod1])
    eve_secondary_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_missiles,
        cat_id=consts.EveEffCat.target,
        mod_info=[eve_mod2])
    eve_primary_abil_id = client.mk_eve_abil(id_=consts.EveAbil.pulse_cannon)
    eve_secondary_abil_id = client.mk_eve_abil(id_=consts.EveAbil.heavy_rocket_salvo)
    eve_fighter_id = client.mk_eve_fighter(
        attrs={eve_affector_attr1_id: 20, eve_affector_attr2_id: 30},
        eff_ids=[eve_primary_effect_id, eve_secondary_effect_id],
        defeff_id=eve_primary_effect_id,
        abils=[client.mk_eve_item_abil(id_=eve_primary_abil_id), client.mk_eve_item_abil(id_=eve_secondary_abil_id)])
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr1_id: 100, eve_affectee_attr2_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_fighter = api_affector_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship_id)
    api_affector_fighter.change_fighter(add_projs=[api_affectee_ship.id])
    # Verification
    api_affector_fighter.update()
    assert len(api_affector_fighter.abilities) == 2
    assert api_affector_fighter.abilities[eve_primary_abil_id].state is True
    assert api_affector_fighter.abilities[eve_secondary_abil_id].state is False
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr1_id].extra == approx(120)
    assert api_affectee_ship.attrs[eve_affectee_attr2_id].extra == approx(100)
    # Action
    api_affector_fighter.change_fighter(state=consts.ApiMinionState.in_space)
    # Verification
    api_affector_fighter.update()
    assert len(api_affector_fighter.abilities) == 2
    assert api_affector_fighter.abilities[eve_primary_abil_id].state is True
    assert api_affector_fighter.abilities[eve_secondary_abil_id].state is False
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr1_id].extra == approx(100)
    assert api_affectee_ship.attrs[eve_affectee_attr2_id].extra == approx(100)
    # Action
    api_affector_fighter.change_fighter(abilities={eve_secondary_abil_id: True})
    # Verification
    api_affector_fighter.update()
    assert len(api_affector_fighter.abilities) == 2
    assert api_affector_fighter.abilities[eve_primary_abil_id].state is True
    assert api_affector_fighter.abilities[eve_secondary_abil_id].state is True
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr1_id].extra == approx(100)
    assert api_affectee_ship.attrs[eve_affectee_attr2_id].extra == approx(100)
    # Action
    api_affector_fighter.change_fighter(state=consts.ApiMinionState.engaging)
    # Verification
    api_affector_fighter.update()
    assert len(api_affector_fighter.abilities) == 2
    assert api_affector_fighter.abilities[eve_primary_abil_id].state is True
    assert api_affector_fighter.abilities[eve_secondary_abil_id].state is True
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr1_id].extra == approx(120)
    assert api_affectee_ship.attrs[eve_affectee_attr2_id].extra == approx(130)
    # Action
    api_affector_fighter.change_fighter(abilities={eve_primary_abil_id: False})
    # Verification
    api_affector_fighter.update()
    assert len(api_affector_fighter.abilities) == 2
    assert api_affector_fighter.abilities[eve_primary_abil_id].state is False
    assert api_affector_fighter.abilities[eve_secondary_abil_id].state is True
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr1_id].extra == approx(100)
    assert api_affectee_ship.attrs[eve_affectee_attr2_id].extra == approx(130)
    # Action
    api_affector_fighter.change_fighter(abilities={eve_secondary_abil_id: False})
    # Verification
    api_affector_fighter.update()
    assert len(api_affector_fighter.abilities) == 2
    assert api_affector_fighter.abilities[eve_primary_abil_id].state is False
    assert api_affector_fighter.abilities[eve_secondary_abil_id].state is False
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr1_id].extra == approx(100)
    assert api_affectee_ship.attrs[eve_affectee_attr2_id].extra == approx(100)
