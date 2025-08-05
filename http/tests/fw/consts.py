from enum import Enum, IntEnum, StrEnum, unique


@unique
class EveItem(IntEnum):
    missile_launcher_operation = 3319
    high_speed_maneuvering = 3454
    micro_jump_drive_operation = 4385
    capital_ships = 20533
    thermodynamics = 28164
    nanite_repair_paste = 28668
    confessor = 34317
    confessor_defense_mode = 34319
    confessor_sharpshooter_mode = 34321
    confessor_propulsion_mode = 34323
    svipul = 34562
    svipul_defense_mode = 34564
    svipul_propulsion_mode = 34566
    svipul_sharpshooter_mode = 34570
    jackdaw = 34828
    jackdaw_defense_mode = 35676
    jackdaw_propulsion_mode = 35677
    jackdaw_sharpshooter_mode = 35678
    hecate = 35683
    hecate_defense_mode = 35686
    hecate_propulsion_mode = 35687
    hecate_sharpshooter_mode = 35688
    stasis_webification_probe = 56748
    capital_micro_jump_drive_operation = 83464
    electric_stability_generator = 87950


@unique
class EveItemGrp(IntEnum):
    character = 1
    effect_beacon = 920
    strategic_cruiser = 963
    ship_modifier = 1306
    light_fighter = 1652
    mutaplasmids = 1964


@unique
class EveItemCat(IntEnum):
    celestial = 2
    ship = 6
    module = 7
    charge = 8
    skill = 16
    commodity = 17
    drone = 18
    implant = 20
    subsystem = 32
    structure = 65
    structure_module = 66
    fighter = 87


@unique
class EveItemList(IntEnum):
    wormhole_jump_blacklist = 245


@unique
class EveEffect(IntEnum):
    shield_boosting = 4
    missile_launching = 9
    tgt_attack = 10
    lo_power = 11
    hi_power = 12
    med_power = 13
    online = 16
    structure_repair = 26
    armor_repair = 27
    projectile_fired = 34
    emp_wave = 38
    launcher_fitted = 40
    turret_fitted = 42
    use_missiles = 101
    defender_missile_launching = 103
    fof_missile_launching = 104
    missile_em_dmg_bonus = 660
    missile_expl_dmg_bonus = 661
    missile_therm_dmg_bonus = 662
    missile_kin_dmg_bonus = 668
    drone_dmg_bonus = 1730
    self_rof = 1851
    rig_slot = 2663
    overload_self_duration_bonus = 3002
    warp_disrupt_sphere = 3380
    script_warp_scramble_range_bonus = 3648
    hardpoint_modifier_effect = 3773
    slot_modifier = 3774
    max_range_hidden_preass_warp_scramble_range = 4894
    adaptive_armor_hardener = 4928
    fueled_shield_boosting = 4936
    fueled_armor_repair = 5275
    ship_mod_remote_capacitor_transmitter = 6184
    ship_mod_remote_hull_repairer = 6185
    ship_mod_remote_shield_booster = 6186
    ship_mod_remote_armor_repairer = 6188
    structure_warp_scramble_block_mwd_with_npc = 6222
    service_slot = 6306
    ftr_abil_missiles = 6431
    ftr_abil_mwd = 6441
    ftr_abil_mjd = 6442
    ftr_abil_attack_m = 6465
    doomsday_aoe_web = 6476
    ftr_abil_launch_bomb = 6485
    ship_mod_ancillary_remote_armor_repairer = 6651
    ship_mod_ancillary_remote_shield_booster = 6652
    npc_entity_remote_armor_repairer = 6687
    npc_entity_remote_shield_booster = 6688
    npc_entity_remote_hull_repairer = 6689
    mod_bonus_microwarpdrive = 6730
    mod_bonus_afterburner = 6731
    mod_bonus_warfare_link_armor = 6732
    mod_titan_effect_generator = 6753
    ship_mod_focused_warp_scrambling_script = 6848
    ship_mod_focused_warp_disruption_script = 6849
    tgt_disintegrator_attack = 6995
    script_st_warp_scram = 7026
    weather_darkness = 7060
    ship_mod_remote_armor_mutadaptive_repairer = 7166
    chain_lightning = 8037
    debuff_lance = 11691
    dot_missile_launching = 12174


@unique
class EveEffCat(IntEnum):
    passive = 0
    active = 1
    target = 2
    area = 3
    online = 4
    overload = 5
    dungeon = 6
    system = 7


@unique
class EveModFunc(StrEnum):
    item = 'ItemModifier'
    loc = 'LocationModifier'
    loc_grp = 'LocationGroupModifier'
    loc_srq = 'LocationRequiredSkillModifier'
    own_srq = 'OwnerRequiredSkillModifier'
    stopper = 'EffectStopper'


@unique
class EveModLoc(StrEnum):
    item = 'itemID'
    char = 'charID'
    ship = 'shipID'
    struct = 'structureID'
    tgt = 'targetID'
    tgt_stopper = 'target'
    other = 'otherID'


@unique
class EveModOp(IntEnum):
    pre_assign = -1
    pre_mul = 0
    pre_div = 1
    mod_add = 2
    mod_sub = 3
    post_mul = 4
    post_div = 5
    post_percent = 6
    post_assign = 7
    post_percent_immune = 8


@unique
class EveBuff(IntEnum):
    warp_penalty = 4
    disallow_dock_jump = 6
    stasis_webification_burst = 27
    disallow_tether = 57
    remote_repair_impedance = 2201
    capacitor_recharge_bonus = 2437


@unique
class EveBuffAggrMode(StrEnum):
    min = 'Minimum'
    max = 'Maximum'


@unique
class EveBuffOp(StrEnum):
    pre_assign = 'PreAssignment'
    pre_mul = 'PreMul'
    pre_div = 'PreDiv'
    mod_add = 'ModAdd'
    mod_sub = 'ModSub'
    post_mul = 'PostMul'
    post_div = 'PostDiv'
    post_percent = 'PostPercent'
    post_assign = 'PostAssignment'


@unique
class EveAbil(IntEnum):
    launch_bomb = 7
    pulse_cannon = 22
    heavy_rocket_salvo = 33


@unique
class EveAttr(IntEnum):
    mass = 4
    hp = 9
    power_output = 11
    low_slots = 12
    med_slots = 13
    hi_slots = 14
    speed_factor = 20
    power = 30
    max_velocity = 37
    capacity = 38
    cpu_output = 48
    cpu = 50
    speed = 51
    max_range = 54
    charge_rate = 56
    dmg_mult = 64
    shield_bonus = 68
    agility = 70
    duration = 73
    structure_dmg_amount = 83
    armor_dmg_amount = 84
    power_transfer_amount = 90
    launcher_slots_left = 101
    turret_slots_left = 102
    warp_scramble_range = 103
    warp_scramble_status = 104
    warp_scramble_strength = 105
    kin_dmg_resonance = 109
    therm_dmg_resonance = 110
    expl_dmg_resonance = 111
    em_dmg_resonance = 113
    em_dmg = 114
    expl_dmg = 116
    kin_dmg = 117
    therm_dmg = 118
    ammo_loaded = 127
    charge_size = 128
    launcher_group = 137
    falloff = 158
    volume = 161
    radius = 162
    missile_dmg_mult = 212
    shield_capacity = 263
    armor_hp = 265
    armor_em_dmg_resonance = 267
    armor_expl_dmg_resonance = 268
    armor_kin_dmg_resonance = 269
    armor_therm_dmg_resonance = 270
    shield_em_dmg_resonance = 271
    shield_expl_dmg_resonance = 272
    shield_kin_dmg_resonance = 273
    shield_therm_dmg_resonance = 274
    skill_level = 280
    explosion_delay = 281
    drone_capacity = 283
    dmg_mult_bonus = 292
    rof_bonus = 293
    implantness = 331
    max_active_drones = 352
    sig_radius = 552
    sig_radius_bonus = 554
    speed_boost_factor = 567
    launcher_group2 = 602
    launcher_group3 = 603
    charge_group1 = 604
    charge_group2 = 605
    charge_group3 = 606
    charge_group4 = 609
    charge_group5 = 610
    max_group_active = 763
    crystal_volatility_chance = 783
    crystal_volatility_damage = 784
    crystals_get_damaged = 786
    mass_addition = 796
    disallow_assistance = 854
    disallow_offensive_modifiers = 872
    max_group_online = 978
    disallow_in_empire_space = 1074
    boosterness = 1087
    upgrade_capacity = 1132
    upgrade_cost = 1153
    upgrade_slots_left = 1154
    overload_self_duration_bonus = 1206
    required_thermodynamics_skill = 1212
    drone_bandwidth = 1271
    drone_bandwidth_used = 1272
    can_fit_ship_group1 = 1298
    can_fit_ship_group2 = 1299
    can_fit_ship_group3 = 1300
    can_fit_ship_group4 = 1301
    can_fit_ship_type1 = 1302
    can_fit_ship_type2 = 1303
    can_fit_ship_type3 = 1304
    can_fit_ship_type4 = 1305
    max_range_hidden = 1317
    warp_scramble_range_bonus = 1327
    activation_blocked = 1349
    activation_blocked_strength = 1350
    subsystem_slot = 1366
    max_subsystems = 1367
    turret_hardpoint_modifier = 1368
    launcher_hardpoint_modifier = 1369
    hi_slot_modifier = 1374
    med_slot_modifier = 1375
    low_slot_modifier = 1376
    fits_to_ship_type = 1380
    armor_max_dmg_resonance = 1527
    max_group_fitted = 1544
    rig_size = 1547
    allowed_drone_group1 = 1782
    allowed_drone_group2 = 1783
    reload_time = 1795
    disallow_vs_ew_immune_tgt = 1798
    resist_shift_amount = 1849
    charged_armor_dmg_mult = 1886
    can_fit_ship_type5 = 1944
    disallow_in_hisec = 1970
    gate_scramble_status = 1973
    gate_scramble_strength = 1974
    ftr_capacity = 2055
    service_slots = 2056
    can_fit_ship_group9 = 2065
    launcher_group4 = 2076
    launcher_group5 = 2077
    launcher_group6 = 2078
    can_fit_ship_type6 = 2103
    remote_resistance_id = 2138
    ftr_tubes = 2216
    ftr_light_slots = 2217
    ftr_support_slots = 2218
    ftr_heavy_slots = 2219
    ftr_sq_is_light = 2212
    ftr_sq_is_support = 2213
    ftr_sq_is_heavy = 2214
    ftr_sq_max_size = 2215
    doomsday_warning_duration = 2262
    doomsday_dmg_duration = 2264
    doomsday_dmg_cycle_time = 2265
    doomsday_aoe_range = 2279
    ftr_abil_launch_bomb_type = 2324
    hisec_modifier = 2355
    lowsec_modifier = 2356
    nullsec_modifier = 2357
    security_modifier = 2358
    can_fit_ship_group10 = 2396
    max_type_fitted = 2431
    can_fit_ship_type7 = 2463
    warfare_buff_1_id = 2468
    warfare_buff_1_value = 2469
    warfare_buff_2_id = 2470
    warfare_buff_2_value = 2471
    warfare_buff_3_id = 2472
    warfare_buff_3_value = 2473
    can_fit_ship_group11 = 2476
    can_fit_ship_group12 = 2477
    can_fit_ship_group13 = 2478
    can_fit_ship_group14 = 2479
    can_fit_ship_group15 = 2480
    can_fit_ship_group16 = 2481
    can_fit_ship_group17 = 2482
    can_fit_ship_group18 = 2483
    can_fit_ship_group19 = 2484
    can_fit_ship_group20 = 2485
    can_fit_ship_type8 = 2486
    can_fit_ship_type9 = 2487
    can_fit_ship_type10 = 2488
    warfare_buff_4_id = 2536
    warfare_buff_4_value = 2537
    online_max_security_class = 2581
    pilot_security_status= 2610
    dmg_mult_bonus_per_cycle = 2733
    dmg_mult_bonus_max = 2734
    ftr_st_light_slots = 2737
    ftr_st_support_slots = 2738
    ftr_st_heavy_slots = 2739
    ftr_sq_is_st_light = 2740
    ftr_sq_is_st_support = 2741
    ftr_sq_is_st_heavy = 2742
    can_fit_ship_type11 = 2758
    repair_mult_bonus_per_cycle = 2796
    repair_mult_bonus_max = 2797
    disallow_in_hazard = 5561
    allow_in_fully_corrupted_lowsec = 5599
    allow_in_fully_corrupted_hisec = 5600


@unique
class EveAttrUnit(IntEnum):
    group_id = 115
    item_id = 116


@unique
class ApiSecZone(StrEnum):
    hisec = 'hisec'
    hisec_c5 = 'hisec_c5'
    lowsec = 'lowsec'
    lowsec_c5 = 'lowsec_c5'
    nullsec = 'nullsec'
    wspace = 'wspace'
    hazard = 'hazard'


@unique
class ApiItemKind(StrEnum):
    autocharge = 'autocharge'
    booster = 'booster'
    character = 'character'
    charge = 'charge'
    drone = 'drone'
    fighter = 'fighter'
    fw_effect = 'fw_effect'
    implant = 'implant'
    module = 'module'
    proj_effect = 'proj_effect'
    rig = 'rig'
    ship = 'ship'
    skill = 'skill'
    stance = 'stance'
    subsystem = 'subsystem'
    sw_effect = 'sw_effect'


@unique
class ApiModuleState(StrEnum):
    ghost = 'ghost'
    offline = 'offline'
    online = 'online'
    active = 'active'
    overload = 'overload'


@unique
class ApiServiceState(StrEnum):
    ghost = 'ghost'
    offline = 'offline'
    online = 'online'


@unique
class ApiMinionState(StrEnum):
    in_bay = 'in_bay'
    in_space = 'in_space'
    engaging = 'engaging'


@unique
class ApiRack(StrEnum):
    high = 'high'
    mid = 'mid'
    low = 'low'


@unique
class ApiEffMode(StrEnum):
    full_compliance = 'full'
    state_compliance = 'state'
    force_run = 'run'
    force_stop = 'stop'


@unique
class ApiModOp(StrEnum):
    base_assign = 'base_assign'
    pre_assign = 'pre_assign'
    pre_mul = 'pre_mul'
    pre_div = 'pre_div'
    mod_add = 'add'
    mod_sub = 'sub'
    post_mul = 'post_mul'
    post_div = 'post_div'
    post_percent = 'post_perc'
    post_assign = 'post_assign'
    min_limit = 'min_limit'
    max_limit = 'max_limit'
    extra_add = 'extra_add'
    extra_mul = 'extra_mul'


@unique
class ApiAggrMode(StrEnum):
    stack = 'stack'


@unique
class ApiModAddMode(StrEnum):
    append = 'append'
    equip = 'equip'
    insert = 'insert'
    replace = 'replace'


@unique
class ApiModRmMode(StrEnum):
    remove = 'remove'
    free = 'free'


@unique
class ApiSideEffectOp(StrEnum):
    add = 'add'
    perc = 'perc'


@unique
class ApiValItemType(StrEnum):
    booster = 'booster'
    character = 'character'
    charge = 'charge'
    drone = 'drone'
    fighter = 'fighter'
    implant = 'implant'
    module_high = 'module_high'
    module_mid = 'module_mid'
    module_low = 'module_low'
    rig = 'rig'
    service = 'service'
    ship = 'ship'
    skill = 'skill'
    stance = 'stance'
    subsystem = 'subsystem'


@unique
class ApiValShipType(StrEnum):
    ship = 'ship'
    structure = 'structure'
    unknown = 'unknown'


@unique
class ApiSolInfoMode(StrEnum):
    id = 'id'
    full = 'full'


@unique
class ApiFitInfoMode(StrEnum):
    id = 'id'
    full = 'full'


@unique
class ApiFleetInfoMode(StrEnum):
    id = 'id'
    full = 'full'


@unique
class ApiItemInfoMode(StrEnum):
    id = 'id'
    partial = 'partial'
    full = 'full'


@unique
class ApiValInfoMode(StrEnum):
    simple = 'simple'
    detailed = 'detailed'


@unique
class PenaltyStr(float, Enum):
    p1 = 1
    p2 = 0.8691199808003974
    p3 = 0.5705831435105602
    p4 = 0.28295515402326105
    p5 = 0.10599264974270436
    p6 = 0.02999116653328046
    p7 = 0.006410183117533506
    p8 = 0.0010349204826687042


class UtilEffect(IntEnum):
    buff_everything = EveEffect.weather_darkness
    buff_ships = EveEffect.mod_titan_effect_generator
    buff_fleet_ships = EveEffect.mod_bonus_warfare_link_armor
    # Use targetAttack/projectileFired effects, since they bring no extra modifiers and define
    # optimal/falloff as range-defining attributes
    tgt_normal1 = EveEffect.tgt_attack
    tgt_normal2 = EveEffect.projectile_fired
    # Use targetDisintegratorAttack effect, since it brings no extra modifiers and defines optimal
    # as a range-defining attribute
    tgt_simple = EveEffect.tgt_disintegrator_attack
    cycle_charge_rate = EveEffect.projectile_fired
    cycle_crystal = EveEffect.tgt_attack
    cycle_none = EveEffect.warp_disrupt_sphere
    activates_charge = EveEffect.use_missiles
    activates_autocharge = EveEffect.ftr_abil_launch_bomb
    not_activates_charge = EveEffect.projectile_fired
