from dataclasses import dataclass


@dataclass
class ValSlotAmountFail:

    used: int
    total: int | None
    users: list[str]

    def __init__(self, *, data: tuple) -> None:
        self.used = data[0]
        self.total = data[1]
        self.users = sorted(data[2])
