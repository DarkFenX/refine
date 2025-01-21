from __future__ import annotations

from dataclasses import dataclass
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from typing import Union


@dataclass
class ValSlotAmountDetails:

    used: int
    total: Union[int, None]
    users: list[str]

    def __init__(self, *, data):
        self.used = data[0]
        self.total = data[1]
        self.users = sorted(data[2])
