from __future__ import annotations

import typing
from dataclasses import dataclass

if typing.TYPE_CHECKING:
    from tests.fw.consts import ApiSecZone


@dataclass
class ValSecZoneFail:

    zone: ApiSecZone
    items: dict[str, list[ApiSecZone]]

    def __init__(self, *, data: tuple) -> None:
        self.zone = data[0]
        self.items = {k: sorted(v) for k, v in data[1].items()}

    def __eq__(self, other: tuple) -> bool:
        return (self.zone, self.items) == (other[0], other[1])
