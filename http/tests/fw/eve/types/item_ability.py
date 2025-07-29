from __future__ import annotations

import typing
from dataclasses import dataclass

if typing.TYPE_CHECKING:
    from tests.fw.util import Absent


@dataclass(kw_only=True)
class ItemAbilityData:

    id: int | type[Absent]
    cooldown: float | None | type[Absent]
    charge_count: int | None | type[Absent]
    charge_rearm_time: float | None | type[Absent]
