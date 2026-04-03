import dataclasses
import typing

from fw.util import Absent

if typing.TYPE_CHECKING:
    from fw.consts import ApiOptionalReload
    from .opt_shared import StatTimeBurst, StatTimeSim


@dataclasses.dataclass(kw_only=True)
class StatsOptionCapBalance:

    src_kinds: StatCapSrcKinds | type[Absent] = Absent
    time_options: StatTimeBurst | StatTimeSim | type[Absent] = Absent


@dataclasses.dataclass(kw_only=True)
class StatCapSrcKinds:

    default: bool | type[Absent] = Absent
    regen: bool | tuple[bool, StatCapRegenOptions] | type[Absent] = Absent
    cap_injectors: bool | type[Absent] = Absent
    nosfs: bool | tuple[bool, StatCapNosfsOptions] | type[Absent] = Absent
    consumers: bool | type[Absent] = Absent
    incoming_transfers: bool | type[Absent] = Absent
    incoming_neuts: bool | type[Absent] = Absent


@dataclasses.dataclass(kw_only=True)
class StatCapRegenOptions:

    cap_perc: float | type[Absent] = Absent


@dataclasses.dataclass(kw_only=True)
class StatCapNosfsOptions:

    projectee_item_id: str | type[Absent] = Absent


@dataclasses.dataclass(kw_only=True)
class StatsOptionCapSim:

    cap_perc: float | type[Absent] = Absent
    optional_reloads: ApiOptionalReload | type[Absent] = Absent
    stagger: bool | tuple[bool, list[str]] | type[Absent] = Absent
