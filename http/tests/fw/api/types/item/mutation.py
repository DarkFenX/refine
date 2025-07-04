from __future__ import annotations

import dataclasses
import typing


@dataclasses.dataclass
class ItemMutation:

    base_type_id: int
    mutator_id: int
    attrs: dict[int, AttrMutation]

    def __init__(self, *, data: tuple) -> None:
        self.base_type_id = data[0]
        self.mutator_id = data[1]
        self.attrs = {int(k): AttrMutation(data=v) for k, v in data[2].items()}

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: tuple) -> bool:
        return (self.base_type_id, self.mutator_id, self.attrs) == (other[0], other[1], other[2])



@dataclasses.dataclass
class AttrMutation:

    roll: float | None
    absolute: float

    def __init__(self, *, data: tuple) -> None:
        self.roll = data[0]
        self.absolute = data[1]

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: tuple) -> bool:
        return (self.roll, self.absolute) == (other[0], other[1])
