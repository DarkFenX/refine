import dataclasses
import typing


@dataclasses.dataclass
class StatMining:

    ore: StatMiningAmount
    ice: StatMiningAmount
    gas: StatMiningAmount

    def __init__(self, *, data: list | tuple) -> None:
        ore, ice, gas = data
        self.ore = StatMiningAmount(data=ore)
        self.ice = StatMiningAmount(data=ice)
        self.gas = StatMiningAmount(data=gas)

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
