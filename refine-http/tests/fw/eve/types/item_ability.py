import typing
from dataclasses import dataclass

if typing.TYPE_CHECKING:
    from fw.util import Absent


@dataclass(kw_only=True)
class ItemAbilityData:

    id: int | type[Absent]
    cooldown: float | type[Absent] | None
    charge_count: int | type[Absent] | None
    charge_rearm_time: float | type[Absent] | None
