# reefast
Rust Engine for Eve Fit Analysis, Statistics and Theorycrafting

# Features
In no particular order:
- separation between user data and game data. This alllows switches between different data sources (e.g. Tranquility and Singularity), and leads to very few checks done when an item is added/changed, with most checks postponed till fit validation happens
- mutated modules/drones
- environmental effects: system-wide (applied to all items in a system), fit-wide (applied to all items of a fit) and projected (applied only to selected items)
- booster side effects
- (TODO) fighter abilities
- fleet buffs
- automatically loaded charges by some effects (e.g. LR fighter bombs) with possibility to check auto-charge's attributes and effects
- fast & flexible fit validation
- reactive armor hardener support (including ability to have multiples with different stats)
- modifications applied via space component EVE system: things like insurgency tackle range, skyhook silo link, nexus event buff "mines", Pochven subpylons are defined via it
- exposure of 3 attribute values for every attribute: base value, dogma value (like you see in game), and value with extra/hidden modifications applied (e.g. AAR rep amount multiplied by 3 if there is paste loaded)
- ability to control effect status
- ability to disable most items (or special "ghost" state for modules) to completely disable modifications applied by them, with ability to check their attributes with all the received modifications
- modification resistance support
- falloff effect reduction
- ability to override amount of fighters in a squad
