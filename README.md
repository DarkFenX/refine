# reefast
Rust Engine for Eve Fit Analysis, Statistics and Theorycrafting

# Features
In no particular order:
- switch between different data sources (e.g. Tranquility and Singularity)
- mutated modules/drones
- environmental effects: system-wide (applied to all items in a system), fit-wide (applied to all items of a fit) and projected (applied only to selected items)
- booster side effects
- fighter abilities (TODO)
- fleet buffs
- automatically loaded charges by some effects (e.g. LR fighter bombs) with possibility to check auto-charge's attributes and effects
- fast & flexible fit validation
- reactive armor hardener support (including ability to have multiples with different stats)
- modifications applied via space component EVE system: things like insurgency tackle range, skyhook silo link, nexus event buff "mines", Pochven subpylons are defined via it
- exposure of 3 attribute values for every attribute: base value, dogma value (like you see in game), and value with extra/hidden modifications applied (e.g. AAR rep amount multiplied by 3 if there is paste loaded)
