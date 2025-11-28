import dataclasses

from tests.fw.util import Absent, dc_to_dict


@dataclasses.dataclass(kw_only=True)
class StatsOptionFitOutRps:

    item_kinds: StatOutRepItemKinds | type[Absent] = Absent
    spool: str | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatsOptionItemOutRps:

    spool: str | type[Absent] = Absent
    ignore_state: bool | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatOutRepItemKinds:

    default: bool | type[Absent] = Absent
    module: bool | type[Absent] = Absent
    minion: bool | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)
