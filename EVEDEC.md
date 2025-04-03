Some data used in the library came from EVE common knowledge and tests, but it is sometimes exposed in code of decompiled EVE client. This file lists those things, and where in decompiled source code one could find it:

- mapping between modifier operator IDs and operators:
  - library: adg::get_mod_operation()
  - client: dogma/const.py
- mapping between fighter abilities and effects:
  - library: adg::get_abil_effect()
  - client: fighters/abilityAttributes.py
- attribute 2358 securityModifier taking values of security-zone-specific attributes for its base value:
  - library: attribute calculation functions
  - client: dogma/attributes/securityModifierAttribute.py
- some of custom effects, which have no modifiers in static data (e.g. bastion):
  - library: spread between several locations (adapted data generator custom effects, attribute calculator custom effects)
  - client: eve/common/script/dogma/pythonEffects/*.py
