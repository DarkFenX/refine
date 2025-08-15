import dataclasses
import typing


@dataclasses.dataclass
class StatErps:

    shield: StatLayerErps | None
    armor: StatLayerErps | None
    hull: StatLayerErps | None

    def __init__(self, *, data: list | tuple) -> None:
        shield, armor, hull = data
        self.shield = StatLayerErps(data=shield) if shield is not None else None
        self.armor = StatLayerErps(data=armor) if armor is not None else None
        self.hull = StatLayerErps(data=hull) if hull is not None else None

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.shield, self.armor, self.hull] == other


@dataclasses.dataclass
class StatLayerErps:

    local: float
    remote: float
    remote_penalized: float
    mult: float

    def __init__(self, *, data: list | tuple) -> None:
        self.local, self.remote, self.remote_penalized, self.mult = data

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.local, self.remote, self.remote_penalized, self.mult] == other
