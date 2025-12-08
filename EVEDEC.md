Some knowledge used to write the library came from EVE common sense, my work on pyfa, and tests. Later, more data was found in decompiled EVE client, or already known things were confirmed. This file lists those findings, and where in decompiled source code one could find it:

- mapping between modifier operator IDs and operators:
  - library: adg::get_mod_operation()
  - client: dogma/const.py
- mapping between fighter abilities and effects:
  - library: adg::get_abil_effect()
  - client: fighters/abilityAttributes.py
- attribute 2358 securityModifier taking values of security-zone-specific attributes for its base value:
  - library: attribute calculation functions
  - client: dogma/attributes/securityModifierAttribute.py
- some of custom effects, which have no modifiers in static data (e.g. basic modifiers for propulsion modules):
  - library: nd::effect::* definitions
  - client: eve/common/script/dogma/pythonEffects/*.py
