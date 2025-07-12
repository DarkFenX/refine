
import dataclasses
import typing


@dataclasses.dataclass
class ValItemVsShipKindFail:

    ship_kind: str
    items: dict[str, str]

    def __init__(self, *, data: list | tuple) -> None:
        self.ship_kind, self.items = data

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.ship_kind, self.items] == other
