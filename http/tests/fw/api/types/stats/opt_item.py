import dataclasses
import typing

from tests.fw.util import Absent
from .opt_shared import dc_to_dict

if typing.TYPE_CHECKING:
    from .opt_shared import (
        StatOptionAlias,
        StatOptionCapBalanceAlias,
        StatOptionEhpAlias,
        StatOptionErpsAlias,
        StatOptionRpsAlias,
    )

type StatOptionItemDpsAlias = StatOptionAlias | tuple[bool, list[StatsOptionItemDps]]
type StatOptionItemVolleyAlias = StatOptionAlias | tuple[bool, list[StatsOptionItemVolley]]
type StatOptionItemRemoteRpsAlias = StatOptionAlias | tuple[bool, list[StatsOptionItemRemoteRps]]
type StatOptionItemRemoteCpsAlias = StatOptionAlias | tuple[bool, list[StatsOptionItemRemoteCps]]
type StatOptionItemRemoteNpsAlias = StatOptionAlias | tuple[bool, list[StatsOptionItemRemoteNps]]


@dataclasses.dataclass(kw_only=True)
class ItemStatsOptions:

    default: bool = False
    speed: StatOptionAlias = Absent
    agility: StatOptionAlias = Absent
    align_time: StatOptionAlias = Absent
    sig_radius: StatOptionAlias = Absent
    mass: StatOptionAlias = Absent
    warp_speed: StatOptionAlias = Absent
    max_warp_range: StatOptionAlias = Absent
    locks: StatOptionAlias = Absent
    lock_range: StatOptionAlias = Absent
    scan_res: StatOptionAlias = Absent
    sensor: StatOptionAlias = Absent
    dscan_range: StatOptionAlias = Absent
    probing_size: StatOptionAlias = Absent
    jam_chance: StatOptionAlias = Absent
    drone_control_range: StatOptionAlias = Absent
    dps: StatOptionItemDpsAlias = Absent
    volley: StatOptionItemVolleyAlias = Absent
    hp: StatOptionAlias = Absent
    ehp: StatOptionEhpAlias = Absent
    wc_ehp: StatOptionAlias = Absent
    rps: StatOptionRpsAlias = Absent
    erps: StatOptionErpsAlias = Absent
    resists: StatOptionAlias = Absent
    remote_rps: StatOptionItemRemoteRpsAlias = Absent
    remote_cps: StatOptionItemRemoteCpsAlias = Absent
    remote_nps: StatOptionItemRemoteNpsAlias = Absent
    cap_amount: StatOptionAlias = Absent
    cap_balance: StatOptionCapBalanceAlias = Absent
    neut_resist: StatOptionAlias = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatsOptionItemDps:

    reload: bool | type[Absent] = Absent
    spool: str | type[Absent] = Absent
    include_charges: bool | type[Absent] = Absent
    ignore_state: bool | type[Absent] = Absent
    projectee_item_id: str | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatsOptionItemVolley:

    spool: str | type[Absent] = Absent
    include_charges: bool | type[Absent] = Absent
    ignore_state: bool | type[Absent] = Absent
    projectee_item_id: str | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatsOptionItemRemoteRps:

    spool: str | type[Absent] = Absent
    ignore_state: bool | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatsOptionItemRemoteCps:

    ignore_state: bool | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatsOptionItemRemoteNps:

    include_charges: bool | type[Absent] = Absent
    ignore_state: bool | type[Absent] = Absent
    projectee_item_id: str | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)
