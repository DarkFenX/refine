import dataclasses
import typing

from fw.util import Absent

if typing.TYPE_CHECKING:
    from fw.consts import ApiStatItemState, ApiStatMiningResourceKind
    from .opt_shared import StatTimeBurst, StatTimeSim


@dataclasses.dataclass(kw_only=True)
class StatsOptionFitMining:

    item_kinds: StatMiningItemKinds | type[Absent] = Absent
    time: StatTimeBurst | StatTimeSim | type[Absent] = Absent
    resource_kind: ApiStatMiningResourceKind | type[Absent] = Absent


@dataclasses.dataclass(kw_only=True)
class StatsOptionItemMining:

    time: StatTimeBurst | StatTimeSim | type[Absent] = Absent
    resource_kind: ApiStatMiningResourceKind | type[Absent] = Absent
    state: ApiStatItemState | type[Absent] = Absent


@dataclasses.dataclass(kw_only=True)
class StatMiningItemKinds:

    default: bool | type[Absent] = Absent
    module: bool | type[Absent] = Absent
    minion: bool | type[Absent] = Absent
