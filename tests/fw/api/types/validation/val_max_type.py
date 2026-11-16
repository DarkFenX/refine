import dataclasses
import typing
from collections import UserDict


class ValMaxTypeFail(UserDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__({int(k): ValMaxTypeType(data=v) for k, v in data.items()})


@dataclasses.dataclass
class ValMaxTypeType:

    item_type_item_ids: list[str]
    items: dict[str, int]

    def __init__(self, *, data: list | tuple) -> None:
        item_type_item_ids, items = data
        self.item_type_item_ids = sorted(item_type_item_ids)
        self.items = dict(items)

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.item_type_item_ids, self.items] == other
