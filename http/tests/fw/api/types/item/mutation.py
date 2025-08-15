import dataclasses
import typing


@dataclasses.dataclass
class ItemMutation:

    base_type_id: int
    mutator_id: int
    attrs: dict[int, AttrMutation]

    def __init__(self, *, data: list | tuple) -> None:
        self.base_type_id, self.mutator_id, attrs = data
        self.attrs = {int(k): AttrMutation(data=v) for k, v in attrs.items()}

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.base_type_id, self.mutator_id, self.attrs] == other



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
