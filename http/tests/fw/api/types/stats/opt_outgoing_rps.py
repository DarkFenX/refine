import dataclasses

from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class StatsOptionFitOutRps:

    item_kinds: StatOutRepItemKinds | type[Absent] = Absent
    spool: str | type[Absent] = Absent


@dataclasses.dataclass(kw_only=True)
class StatsOptionItemOutRps:

    spool: str | type[Absent] = Absent
    ignore_state: bool | type[Absent] = Absent


@dataclasses.dataclass(kw_only=True)
class StatOutRepItemKinds:

    default: bool | type[Absent] = Absent
    module: bool | type[Absent] = Absent
    minion: bool | type[Absent] = Absent
