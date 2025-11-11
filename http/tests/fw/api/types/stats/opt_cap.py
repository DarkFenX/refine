import dataclasses

from tests.fw.util import Absent, dc_to_dict


@dataclasses.dataclass(kw_only=True)
class StatCapSrcKinds:

    default: bool | type[Absent] = Absent
    regen: bool | tuple[bool, StatCapRegenOptions] | type[Absent] = Absent
    cap_injectors: bool | type[Absent] = Absent
    consumers: bool | tuple[bool, StatCapConsumerOptions] | type[Absent] = Absent
    incoming_transfers: bool | type[Absent] = Absent
    incoming_neuts: bool | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatCapRegenOptions:

    cap_perc: float | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatCapConsumerOptions:

    reload: bool | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)
