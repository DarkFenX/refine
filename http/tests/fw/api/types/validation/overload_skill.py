from dataclasses import dataclass


@dataclass
class ValOverloadSkillFail:

    td_lvl: int | None
    items: dict[str, int]

    def __init__(self, *, data: tuple) -> None:
        self.td_lvl = data[0]
        self.items = data[1]

    def __eq__(self, other: tuple) -> bool:
        return (self.td_lvl, self.items) == (other[0], other[1])
