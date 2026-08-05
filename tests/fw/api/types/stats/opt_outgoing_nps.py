import dataclasses
import typing

from fw.util import Absent

if typing.TYPE_CHECKING:
    from fw.consts import ApiStatItemCharges, ApiStatItemState
    from .opt_shared import StatTimeBurst, StatTimeSim


@dataclasses.dataclass(kw_only=True)
class StatsOptionFitOutNps:

    item_kinds: StatNeutItemKinds | type[Absent] = Absent
    time: StatTimeBurst | StatTimeSim | type[Absent] = Absent
    projectee_item_id: str | type[Absent] = Absent


@dataclasses.dataclass(kw_only=True)
class StatsOptionItemOutNps:

    time: StatTimeBurst | StatTimeSim | type[Absent] = Absent
    charges: ApiStatItemCharges | type[Absent] = Absent
    state: ApiStatItemState | type[Absent] = Absent
    projectee_item_id: str | type[Absent] = Absent


@dataclasses.dataclass(kw_only=True)
class StatNeutItemKinds:

    default: bool | type[Absent] = Absent
    module: bool | type[Absent] = Absent
    minion: bool | type[Absent] = Absent
    bomb: bool | type[Absent] = Absent
    side_effect: bool | type[Absent] = Absent
