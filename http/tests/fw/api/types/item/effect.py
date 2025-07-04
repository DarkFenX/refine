from __future__ import annotations

import dataclasses
import typing

if typing.TYPE_CHECKING:
    from tests.fw.consts import ApiEffMode


@dataclasses.dataclass
class EffectInfo:

    running: bool
    mode: ApiEffMode

    def __init__(self, *, data: tuple) -> None:
        self.running = data[0]
        self.mode = data[1]

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: tuple) -> bool:
        return (self.running, self.mode) == (other[0], other[1])
