import dataclasses
import typing


@dataclasses.dataclass
class SideEffectInfo:

    chance: float
    state: bool
    mod: SideEffectModInfo | None

    def __init__(self, *, data: list | tuple) -> None:
        self.chance, self.state, side_mod = data
        self.mod = None if side_mod is None else SideEffectModInfo(data=side_mod)

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.chance, self.state, self.mod] == other


@dataclasses.dataclass
class SideEffectModInfo:

    op: str
    str: float

    def __init__(self, *, data: list | tuple) -> None:
        self.op, self.str = data

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.op, self.str] == other
