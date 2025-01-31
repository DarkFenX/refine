from dataclasses import dataclass


@dataclass
class ValSlotAmountDetails:

    used: int
    total: int | None
    users: list[str]

    def __init__(self, *, data):
        self.used = data[0]
        self.total = data[1]
        self.users = sorted(data[2])
