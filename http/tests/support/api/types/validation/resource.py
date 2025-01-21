from __future__ import annotations

from dataclasses import dataclass
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from typing import Union


@dataclass
class ValResourceDetails:

    used: int
    output: Union[float, None]
    users: dict[str, float]

    def __init__(self, *, data):
        self.used = data[0]
        self.output = data[1]
        self.users = data[2]
