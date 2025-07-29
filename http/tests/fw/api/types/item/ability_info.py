from __future__ import annotations

import dataclasses
import typing


@dataclasses.dataclass
class AbilityInfo:

    status: bool
    charge_count: int | None

    def __init__(self, *, data: list | tuple) -> None:
        self.status, self.charge_count = data

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.status, self.charge_count] == other
