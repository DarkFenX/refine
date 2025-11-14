import dataclasses

from tests.fw.util import Absent, dc_to_dict


@dataclasses.dataclass(kw_only=True)
class StatsOptionFitRemoteRps:

    item_kinds: StatRemoteRepItemKinds | type[Absent] = Absent
    spool: str | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatsOptionItemRemoteRps:

    spool: str | type[Absent] = Absent
    ignore_state: bool | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatRemoteRepItemKinds:

    default: bool | type[Absent] = Absent
    module: bool | type[Absent] = Absent
    minion: bool | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)
