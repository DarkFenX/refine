from __future__ import annotations

from dataclasses import dataclass


@dataclass
class ValShipStanceFail:

    item_id: str

    def __init__(self, *, data: tuple) -> None:
        self.item_id = data[0]
