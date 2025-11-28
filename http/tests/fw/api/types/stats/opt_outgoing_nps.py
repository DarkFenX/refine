import dataclasses

from tests.fw.util import Absent, dc_to_dict


@dataclasses.dataclass(kw_only=True)
class StatsOptionFitOutNps:

    item_kinds: StatNeutItemKinds | type[Absent] = Absent
    projectee_item_id: str | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatsOptionItemOutNps:

    include_charges: bool | type[Absent] = Absent
    ignore_state: bool | type[Absent] = Absent
    projectee_item_id: str | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatNeutItemKinds:

    default: bool | type[Absent] = Absent
    module: bool | type[Absent] = Absent
    minion: bool | type[Absent] = Absent
    bomb: bool | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)
