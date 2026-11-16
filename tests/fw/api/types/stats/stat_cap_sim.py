import dataclasses
import typing


@dataclasses.dataclass(kw_only=True)
class StatCapSim:

    status: str
    value: float

    @classmethod
    def stable(cls, *, value: float) -> typing.Self:
        return cls(status='stable', value=value)

    @classmethod
    def time(cls, *, value: float) -> typing.Self:
        return cls(status='time', value=value)

    @classmethod
    def from_data(cls, *, data: list | tuple | None) -> typing.Self | None:
        # Options which failed to produce a result have no data
        if data is None:
            return None
        status, value = data
        return cls(status=status, value=value)
