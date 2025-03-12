from dataclasses import dataclass


@dataclass
class ValUnusableResFail:

    max: float | None
    users: dict[str, float]

    def __init__(self, *, data: tuple) -> None:
        self.max = data[0]
        self.users = data[1]
