import dataclasses
import typing

from fw.util import Absent

if typing.TYPE_CHECKING:
    from fw.consts import ApiStatItemState
    from .opt_shared import StatTime


@dataclasses.dataclass(kw_only=True)
class StatsOptionFitOutCps:

    time: StatTime | type[Absent] = Absent
    projectee_item_id: str | type[Absent] = Absent


@dataclasses.dataclass(kw_only=True)
class StatsOptionItemOutCps:

    time: StatTime | type[Absent] = Absent
    state: ApiStatItemState | type[Absent] = Absent
    projectee_item_id: str | type[Absent] = Absent
