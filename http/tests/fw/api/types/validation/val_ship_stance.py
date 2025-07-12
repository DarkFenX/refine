import dataclasses
import typing


@dataclasses.dataclass
class ValShipStanceFail:

    item_id: str

    def __init__(self, *, data: list | tuple) -> None:
        self.item_id, = data

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.item_id] == other
