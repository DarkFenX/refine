from dataclasses import dataclass


@dataclass
class ValResourceFail:

    used: int
    output: float | None
    users: dict[str, float]

    def __init__(self, *, data: tuple) -> None:
        self.used = data[0]
        self.output = data[1]
        self.users = data[2]
