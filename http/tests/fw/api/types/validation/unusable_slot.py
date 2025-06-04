import dataclasses


@dataclasses.dataclass
class ValUnusableSlotFail:

    max: int | None
    users: list[str]

    def __init__(self, *, data: tuple) -> None:
        self.max = data[0]
        self.users = sorted(data[1])
