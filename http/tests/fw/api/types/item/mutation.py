from __future__ import annotations

import dataclasses


@dataclasses.dataclass(kw_only=True)
class ItemMutation:

    base_type_id: int
    mutator_id: int
    attrs: dict[int, AttrMutation]


@dataclasses.dataclass(kw_only=True)
class AttrMutation:

    roll: float | None
    absolute: float
