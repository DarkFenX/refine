import dataclasses
import typing

from fw.api.types.helpers import attr_http_to_fw


@dataclasses.dataclass
class ItemMutation:

    base_type_id: int
    mutator_id: int
    attrs: dict[int, AttrMutation]
    rolls: dict[int, float]

    def __init__(self, *, data: list | tuple) -> None:
        self.base_type_id, self.mutator_id, attrs, rolls = data
        self.attrs = {attr_http_to_fw(attr_id=k): AttrMutation(data=v) for k, v in attrs.items()}
        self.rolls = {attr_http_to_fw(attr_id=k): v for k, v in rolls.items()}

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.base_type_id, self.mutator_id, self.attrs, self.rolls] == other


@dataclasses.dataclass
class AttrMutation:

    roll: float | None
    absolute: float

    def __init__(self, *, data: list | tuple) -> None:
        self.roll, self.absolute = data

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.roll, self.absolute] == other
