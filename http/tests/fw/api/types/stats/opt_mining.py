import dataclasses

from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class StatsOptionFitMining:

    item_kinds: StatMiningItemKinds | type[Absent] = Absent
    reload: bool | type[Absent] = Absent


@dataclasses.dataclass(kw_only=True)
class StatsOptionItemMining:

    reload: bool | type[Absent] = Absent
    ignore_state: bool | type[Absent] = Absent


@dataclasses.dataclass(kw_only=True)
class StatMiningItemKinds:

    default: bool | type[Absent] = Absent
    module: bool | type[Absent] = Absent
    minion: bool | type[Absent] = Absent
