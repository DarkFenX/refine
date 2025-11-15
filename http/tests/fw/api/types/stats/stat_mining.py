import dataclasses
import typing


@dataclasses.dataclass
class StatMining:

    ore: StatMiningAmount | None
    ice: StatMiningAmount | None
    gas: StatMiningAmount | None

    def __init__(self, *, data: dict) -> None:
        self.ore = StatMiningAmount(data=data.get('ore')) if data.get('ore') is not None else None
        self.ice = StatMiningAmount(data=data.get('ice')) if data.get('ice') is not None else None
        self.gas = StatMiningAmount(data=data.get('gas')) if data.get('gas') is not None else None

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.ore, self.ice, self.gas] == other


@dataclasses.dataclass
class StatMiningAmount:

    yield_: float
    drain: float

    def __init__(self, *, data: list | tuple) -> None:
        self.yield_, self.drain = data

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.yield_, self.drain] == other
