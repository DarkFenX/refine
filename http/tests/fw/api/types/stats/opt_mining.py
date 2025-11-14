import dataclasses

from tests.fw.util import Absent, dc_to_dict


@dataclasses.dataclass(kw_only=True)
class StatsOptionFitMining:

    item_kinds: StatMiningItemKinds | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatsOptionItemMining:

    ignore_state: bool | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatMiningItemKinds:

    default: bool | type[Absent] = Absent
    module: bool | type[Absent] = Absent
    minion: bool | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)
