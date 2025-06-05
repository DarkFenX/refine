
import dataclasses
import typing


@dataclasses.dataclass
class ValItemVsShipKindFail:

    ship_kind: str
    items: dict[str, str]

    def __init__(self, *, data: tuple) -> None:
        self.ship_kind = data[0]
        self.items = data[1]

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: tuple) -> bool:
        return (self.ship_kind, self.items) == (other[0], other[1])
