# reefast
Rust Engine for Eve Fit Analysis, Statistics and Theorycrafting

# Crate layout
### reefast-core
Core calculation library with Rust interface.
### reefast-edh
Data handlers for core library, which provide access to EVE data.
### reefast-adc
Data cachers for core library, which allow it to cache adapted data. Adapted data is processed EVE data, adapted for needs of the library.
### reefast-http
HTTP interface to the library.

# Documentation
HTTP is considered as main interface, and it will be documented later. Until then, the easiest way to see how it works is to check tests and test framework client, as well as [non-synthetic-data tests](http/tests/playground/actual_data.py) which use it.

# Features
In no particular order:
- separation between user data and game data. This allows switches between different data sources (e.g. Tranquility and Singularity), and leads to very few checks done when an item is added/changed, with most checks postponed till fit validation happens
- mutated modules/drones
  - can specify attribute rolls via absolute values, or rolls (roll quality as a number between 0 and 1)
  - attribute mutations are stored as rolls internally, so will properly update on base attribute/roll range changes between data sources
- environmental effects: system-wide (applied to all items in a system), fit-wide (applied to all items of a fit) and projected (applied only to selected items)
- booster side effects
- fighter abilities
- fleet buffs
- automatically loaded charges by some effects (e.g. LR fighter bombs)
  - auto-charges are exposed as regular items so that it's possible to check their attributes and effects (but there are some limits, e.g. you cannot remove them)
- fast & flexible fit validation
  - "try fit items" functionality which accepts list of items, checks which can be fit to a fit according to passed validation settings, and returns those which can be fit
- fleet, fit and item stats
  - some stats support various options, e.g. it is possible to get raw DPS output of a fit, or only vorton DPS of a fit applied to a specific item
- reactive armor hardener support
  - it is possible to fit multiple RAHs with different attributes (validation fails, but they are properly simulated nevertheless)
- modifications applied via space component EVE system: things like insurgency tackle range, skyhook silo link, nexus event buff "mines", Pochven subpylons, Odysseus-specific buffs are defined via it
- exposure of 3 attribute values for every attribute: base value, dogma value (like you see in game), and value with extra/hidden modifications applied (e.g. AAR rep amount multiplied by 3 if there is paste loaded, or hidden missile flight time bonus based on ship radius)
- ability to disable most items (or special "disabled" state for modules, which can be needed for modules like cargo expanders applying their penalties even when offline) to completely disable modifications applied by them, with ability to check their attributes with all the received modifications
- modification resistance support
- falloff effect reduction
- drone propulsion mode (cruise/chase), which affects their speed and signature, and can affect ability of various items apply to them
- ability to override amount of fighters in a squad
- ability to control effect run mode (force stop, force run, state compliance, full compliance)
- list modifications applied upon an attribute, with meaningless modifications filtered out (e.g. multiplication by 1)
- security zone support for calculations (e.g. structure rigs)
- pilot security status support (e.g. for concord ships)
